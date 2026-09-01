//! Assembles the top bar UI and wires up its event reactor.
//!
//! All producers (Hyprland, clock/system ticks, battery) push an [`Event`]
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

const BAR_HEIGHT: i32 = 34;
const POLL_INTERVAL: Duration = Duration::from_millis(50);
/// Delay before the notification center hides after the pointer leaves the
/// bell or the panel, so the cursor can move between them.
const HIDE_DELAY: Duration = Duration::from_millis(200);

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

    // Root horizontal box holding [workspaces | clock | sysinfo].
    let root = Box::new(Orientation::Horizontal, 0);
    root.add_css_class("bar");
    root.set_halign(Align::Fill);
    window.set_child(Some(&root));

    // Left: workspaces
    let workspaces_box = widgets::workspace::create();
    // Center: clock
    let center = Box::new(Orientation::Horizontal, 0);
    center.set_halign(Align::Center);
    center.set_valign(Align::Center);
    let clock_label = widgets::clock::create();
    center.append(&clock_label);
    // Right: system info
    let (right, sys_labels) = widgets::sysinfo::create();
    // Audio service pill, shown at the far right of the system group.
    let audio_label = widgets::audio::create();
    right.append(&audio_label);
    // Notification widget: toast + center, plus the bell button.
    let (mut notif_widget, bell) = widgets::notifications::NotificationWidget::new(app);
    right.append(&bell);
    let center_scroll = notif_widget.center_scroll().clone();

    root.append(&workspaces_box);
    root.append(&center);
    root.append(&right);

    // Stretch the three sections across the full width.
    center.set_hexpand(true);
    workspaces_box.set_hexpand(true);
    right.set_hexpand(true);

    style::load();

    // Initial render (best-effort; never panics).
    widgets::workspace::refresh(&workspaces_box);
    widgets::clock::update(&clock_label);
    widgets::sysinfo::update(&sys_labels);
    widgets::audio::refresh(&audio_label);

    // Shared event bus: every producer sends into this single channel.
    let (tx, rx) = mpsc::channel::<Event>();

    // The bell opens the notification center on hover and closes it when the
    // pointer leaves both the bell and the center window. Hovering never
    // touches the widget directly; it emits events on the bus and lets the
    // reactor mutate the widget in its own loop, which avoids panicking when a
    // GLib signal fires re-entrantly (Rust panics can't unwind through C).
    let hide_source = Rc::new(Cell::new(None::<glib::SourceId>));

    // Bell: show the center on hover, schedule a hide on leave.
    let bell_motion = EventControllerMotion::new();
    bell_motion.connect_enter({
        let hide_source = hide_source.clone();
        let tx = tx.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
            let _ = tx.send(Event::ShowNotificationCenter);
        }
    });
    bell_motion.connect_leave({
        let hide_source = hide_source.clone();
        let tx = tx.clone();
        move |_| {
            let tx = tx.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                let _ = tx.send(Event::HideNotificationCenter);
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    bell.add_controller(bell_motion);

    // Center window: hide on leave, but cancel the pending hide while hovered.
    let center_motion = EventControllerMotion::new();
    center_motion.connect_enter({
        let hide_source = hide_source.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
        }
    });
    center_motion.connect_leave({
        let hide_source = hide_source.clone();
        let tx = tx.clone();
        move |_| {
            let tx = tx.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                let _ = tx.send(Event::HideNotificationCenter);
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    center_scroll.add_controller(center_motion);

    // Spawn the producers.
    let _producers: Vec<EventProducer> = vec![
        events::spawn_hyprland(tx.clone()),
        events::spawn_tickers(tx.clone()),
        events::spawn_battery(tx.clone()),
        events::spawn_audio(tx.clone()),
        events::spawn_notifications(tx.clone()),
    ];

    // Single main-thread reactor: drain the channel and dispatch to widgets.
    glib::timeout_add_local(POLL_INTERVAL, move || {
        loop {
            match rx.recv_timeout(Duration::from_millis(1)) {
                Ok(event) => dispatch(
                    event,
                    &workspaces_box,
                    &clock_label,
                    &sys_labels,
                    &audio_label,
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
    clock_label: &gtk4::Label,
    sys_labels: &[gtk4::Label],
    audio_label: &gtk4::Label,
    notif_widget: &mut widgets::notifications::NotificationWidget,
) {
    match event {
        Event::WorkspaceActive(_) | Event::WorkspaceListChanged => {
            widgets::workspace::refresh(workspaces_box);
        }
        Event::ClockTick => widgets::clock::update(clock_label),
        Event::SystemTick => widgets::sysinfo::update_system(sys_labels),
        Event::BatteryChanged => widgets::sysinfo::update_battery(sys_labels),
        Event::AudioChanged => widgets::audio::refresh(audio_label),
        Event::Notification(_)
        | Event::NotificationClosed { .. }
        | Event::ShowNotificationCenter
        | Event::HideNotificationCenter => {
            notif_widget.handle(&event);
        }
    }
}
