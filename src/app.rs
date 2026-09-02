//! Assembles the top bar UI and wires up its event reactor.
//!
//! All producers (Hyprland, clock/system ticks, battery, audio) push an [`Event`]
//! onto a single channel. A single main-thread reactor drains that channel and
//! dispatches each event to the relevant widget update.

use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box, EventControllerMotion, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc;
use std::time::Duration;

use crate::events;
use crate::events::{Event, EventProducer};
use crate::style;
use crate::widgets;

const BAR_HEIGHT: i32 = 36;
const POLL_INTERVAL: Duration = Duration::from_millis(50);
/// Delay before the notification center hides after the pointer leaves the
/// bell or the panel, so the cursor can move between them.
const HIDE_DELAY: Duration = Duration::from_millis(250);

/// Build the top layer-shell bar and attach all behaviour.
pub fn build(app: &Application) {
    let window = ApplicationWindow::builder().application(app).build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(BAR_HEIGHT);
    window.set_default_size(-1, BAR_HEIGHT);
    window.add_css_class("bar-window");

    // Shared event bus: every producer sends into this single channel.
    let (tx, rx) = mpsc::channel::<Event>();

    // Root horizontal box holding [left | center | right].
    let root = Box::new(Orientation::Horizontal, 0);
    root.add_css_class("bar");
    root.set_halign(Align::Fill);
    window.set_child(Some(&root));

    // Left: workspaces and active window title
    let left = Box::new(Orientation::Horizontal, 4);
    left.set_halign(Align::Start);
    left.set_valign(Align::Center);
    left.set_hexpand(true);
    left.set_margin_start(8);

    let workspaces_box = widgets::workspace::create();
    let window_title = widgets::window::create();
    left.append(&workspaces_box);
    left.append(&window_title);

    // Center: clock with interactive calendar
    let center = Box::new(Orientation::Horizontal, 0);
    center.set_halign(Align::Center);
    center.set_valign(Align::Center);
    center.set_hexpand(true);

    let (clock_pill, clock_label) = widgets::clock::create(app);
    center.append(&clock_pill);

    // Right: system info, unified quicksettings (network/bluetooth/audio), notifications
    let right = Box::new(Orientation::Horizontal, 4);
    right.set_halign(Align::End);
    right.set_valign(Align::Center);
    right.set_hexpand(true);
    right.set_margin_end(8);

    let (sys_box, sys_labels) = widgets::sysinfo::create();
    let (qs_btn, qs_labels, qs_window, qs_reload) = widgets::quicksettings::create(app);
    let (mut notif_widget, bell) = widgets::notifications::NotificationWidget::new(app, tx.clone());

    right.append(&sys_box);
    right.append(&qs_btn);
    right.append(&bell);

    let center_dropdown = notif_widget.center_dropdown().clone();

    root.append(&left);
    root.append(&center);
    root.append(&right);

    style::load();

    // Initial render (best-effort; never panics).
    widgets::workspace::refresh(&workspaces_box);
    widgets::window::refresh(&window_title);
    widgets::clock::update(&clock_label);
    widgets::sysinfo::update(&sys_labels);
    widgets::quicksettings::refresh_network(&qs_labels);
    widgets::quicksettings::refresh_bluetooth(&qs_labels);
    widgets::quicksettings::refresh_audio(&qs_labels);
    widgets::quicksettings::refresh_battery(&qs_labels);

    // The bell opens the notification center on hover and closes it when the
    // pointer leaves both the bell and the center window.
    let hide_source = Rc::new(Cell::new(None::<glib::SourceId>));

    // Bell: click or hover to show the center.
    let tx_click = tx.clone();
    let hide_source_click = hide_source.clone();
    bell.connect_clicked(move |_| {
        if let Some(source) = hide_source_click.take() {
            source.remove();
        }
        let _ = tx_click.send(Event::ShowNotificationCenter);
    });

    let bell_motion = EventControllerMotion::new();
    let tx_enter = tx.clone();
    let hide_source_enter = hide_source.clone();
    bell_motion.connect_enter(move |_, _, _| {
        if let Some(source) = hide_source_enter.take() {
            source.remove();
        }
        let _ = tx_enter.send(Event::ShowNotificationCenter);
    });

    let tx_leave = tx.clone();
    let hide_source_leave = hide_source.clone();
    bell_motion.connect_leave(move |_| {
        let tx_cb = tx_leave.clone();
        let hide_source_cb = hide_source_leave.clone();
        let source = glib::timeout_add_local(HIDE_DELAY, move || {
            hide_source_cb.set(None);
            let _ = tx_cb.send(Event::HideNotificationCenter);
            glib::ControlFlow::Break
        });
        hide_source_leave.set(Some(source));
    });
    bell.add_controller(bell_motion);

    // Keep the center open while the pointer is over it.
    let center_motion = EventControllerMotion::new();
    let hide_source_dropdown = hide_source.clone();
    center_motion.connect_enter(move |_, _, _| {
        if let Some(source) = hide_source_dropdown.take() {
            source.remove();
        }
    });

    let tx_dropdown_leave = tx.clone();
    let hide_source_dropdown_leave = hide_source.clone();
    center_motion.connect_leave(move |_| {
        let tx_cb = tx_dropdown_leave.clone();
        let hide_source_cb = hide_source_dropdown_leave.clone();
        let source = glib::timeout_add_local(HIDE_DELAY, move || {
            hide_source_cb.set(None);
            let _ = tx_cb.send(Event::HideNotificationCenter);
            glib::ControlFlow::Break
        });
        hide_source_dropdown_leave.set(Some(source));
    });
    center_dropdown.add_controller(center_motion);

    // Spawn IPC Unix Socket server for external keybinds and shell commands.
    crate::ipc::spawn_server(tx.clone());

    // Spawn the background event producers.
    let _producers: Vec<EventProducer> = vec![
        events::spawn_hyprland(tx.clone()),
        events::spawn_tickers(tx.clone()),
        events::spawn_battery(tx.clone()),
        events::spawn_audio(tx.clone()),
        events::spawn_notifications(tx.clone()),
    ];

    // Single main-thread reactor: drain the channel and dispatch to widgets.
    let qs_win_cl = qs_window.clone();
    let qs_rel_cl = qs_reload.clone();
    glib::timeout_add_local(POLL_INTERVAL, move || {
        loop {
            match rx.recv_timeout(Duration::from_millis(1)) {
                Ok(event) => dispatch(
                    event,
                    &workspaces_box,
                    &window_title,
                    &clock_label,
                    &sys_labels,
                    &qs_labels,
                    &qs_win_cl,
                    &qs_rel_cl,
                    &mut notif_widget,
                ),
                Err(mpsc::RecvTimeoutError::Timeout) => break,
                Err(mpsc::RecvTimeoutError::Disconnected) => return glib::ControlFlow::Break,
            }
        }
        glib::ControlFlow::Continue
    });

    window.present();
}

/// Apply one event to the widgets it concerns.
fn dispatch(
    event: Event,
    workspaces_box: &gtk4::Box,
    window_title: &gtk4::Label,
    clock_label: &gtk4::Label,
    sys_labels: &[gtk4::Label],
    qs_labels: &widgets::quicksettings::QuickSettingsLabels,
    qs_window: &gtk4::ApplicationWindow,
    qs_reload: &Rc<dyn Fn()>,
    notif_widget: &mut widgets::notifications::NotificationWidget,
) {
    match event {
        Event::WorkspaceActive(_) | Event::WorkspaceListChanged => {
            widgets::workspace::refresh(workspaces_box);
        }
        Event::ActiveWindow(title) => {
            widgets::window::update(window_title, title.as_deref());
        }
        Event::ClockTick => widgets::clock::update(clock_label),
        Event::SystemTick => widgets::sysinfo::update_system(sys_labels),
        Event::BatteryChanged => widgets::quicksettings::refresh_battery(qs_labels),
        Event::NetworkChanged => widgets::quicksettings::refresh_network(qs_labels),
        Event::BluetoothChanged => widgets::quicksettings::refresh_bluetooth(qs_labels),
        Event::AudioChanged => widgets::quicksettings::refresh_audio(qs_labels),
        Event::ToggleQuickSettings => widgets::quicksettings::toggle(qs_window, qs_reload),
        Event::ReloadStyle => style::load(),
        Event::Notification(_)
        | Event::NotificationClosed { .. }
        | Event::ClearAllNotifications
        | Event::DismissNotification(_)
        | Event::ShowNotificationCenter
        | Event::HideNotificationCenter
        | Event::ToggleNotifications => {
            notif_widget.handle(&event);
        }
    }
}
