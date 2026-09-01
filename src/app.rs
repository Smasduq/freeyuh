//! Assembles the top bar UI and wires up its event reactor.
//!
//! All producers (Hyprland, clock/system ticks, battery) push an [`Event`]
//! onto a single channel. A single main-thread reactor drains that channel and
//! dispatches each event to the relevant widget update.

use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::sync::mpsc;
use std::time::Duration;

use crate::events;
use crate::events::{Event, EventProducer};
use crate::style;
use crate::widgets;

const BAR_HEIGHT: i32 = 34;
const POLL_INTERVAL: Duration = Duration::from_millis(50);

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

    // Spawn the producers.
    let _producers: Vec<EventProducer> = vec![
        events::spawn_hyprland(tx.clone()),
        events::spawn_tickers(tx.clone()),
        events::spawn_battery(tx.clone()),
        events::spawn_audio(tx.clone()),
    ];

    // Single main-thread reactor: drain the channel and dispatch to widgets.
    glib::timeout_add_local(POLL_INTERVAL, move || {
        loop {
            match rx.recv_timeout(Duration::from_millis(1)) {
                Ok(event) => {
                    dispatch(event, &workspaces_box, &clock_label, &sys_labels, &audio_label)
                }
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
) {
    match event {
        Event::WorkspaceActive(_) | Event::WorkspaceListChanged => {
            widgets::workspace::refresh(workspaces_box);
        }
        Event::ClockTick => widgets::clock::update(clock_label),
        Event::SystemTick => widgets::sysinfo::update_system(sys_labels),
        Event::BatteryChanged => widgets::sysinfo::update_battery(sys_labels),
        Event::AudioChanged => widgets::audio::refresh(audio_label),
    }
}
