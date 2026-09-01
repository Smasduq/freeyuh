//! Clock widget: centered date + time label with an interactive calendar popover.
//!
//! The calendar opens on hover over the clock pill (or on click) and closes
//! automatically when the pointer leaves both the pill and the calendar.

use chrono::Local;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Calendar, EventControllerMotion, GestureClick, Label,
    Orientation,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

const HIDE_DELAY: Duration = Duration::from_millis(250);

/// Creates the clock pill and its associated calendar popup window.
///
/// Returns `(clock_button, clock_label)`.
pub fn create(app: &Application) -> (Box, Label) {
    let container = Box::new(Orientation::Horizontal, 0);
    container.add_css_class("clock-pill");
    container.set_valign(gtk4::Align::Center);
    container.set_halign(gtk4::Align::Center);

    let label = Label::new(None);
    label.set_use_markup(true);
    label.add_css_class("clock-label");
    container.append(&label);

    // --- Calendar Dropdown Window ---
    let calendar_window = ApplicationWindow::builder().application(app).build();
    calendar_window.init_layer_shell();
    calendar_window.set_layer(Layer::Top);
    calendar_window.set_anchor(Edge::Top, true);
    calendar_window.set_margin(Edge::Top, 42);
    calendar_window.set_keyboard_mode(KeyboardMode::None);
    calendar_window.set_exclusive_zone(0);
    calendar_window.add_css_class("calendar-window");

    let dropdown = Box::new(Orientation::Vertical, 8);
    dropdown.add_css_class("calendar-dropdown");
    dropdown.set_width_request(280);

    let now = Local::now();
    let header = Label::new(Some(&now.format("%A, %B %e, %Y").to_string()));
    header.add_css_class("calendar-header");
    header.set_halign(gtk4::Align::Center);
    dropdown.append(&header);

    let cal = Calendar::new();
    cal.add_css_class("calendar-widget");
    dropdown.append(&cal);

    calendar_window.set_child(Some(&dropdown));
    calendar_window.hide();

    // Source handle for delayed hide on pointer leave.
    let hide_source = Rc::new(Cell::new(None::<glib::SourceId>));

    // Clock pill: show calendar on hover, schedule hide on leave.
    let motion = EventControllerMotion::new();
    motion.connect_enter({
        let hide_source = hide_source.clone();
        let cal_win = calendar_window.clone();
        let header_label = header.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
            let current = Local::now();
            header_label.set_text(&current.format("%A, %B %e, %Y").to_string());
            cal_win.present();
        }
    });

    motion.connect_leave({
        let hide_source = hide_source.clone();
        let cal_win = calendar_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let cal_win = cal_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                cal_win.hide();
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    container.add_controller(motion);

    // Calendar window: cancel hide while hovered, schedule hide on leave.
    let cal_motion = EventControllerMotion::new();
    cal_motion.connect_enter({
        let hide_source = hide_source.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
        }
    });
    cal_motion.connect_leave({
        let hide_source = hide_source.clone();
        let cal_win = calendar_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let cal_win = cal_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                cal_win.hide();
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    dropdown.add_controller(cal_motion);

    // Also support clicking to toggle.
    let gesture = GestureClick::new();
    gesture.set_button(1);
    let cal_win_click = calendar_window.clone();
    let header_label = header.clone();
    gesture.connect_pressed(move |_, _, _, _| {
        if cal_win_click.is_visible() {
            cal_win_click.hide();
        } else {
            let current = Local::now();
            header_label.set_text(&current.format("%A, %B %e, %Y").to_string());
            cal_win_click.present();
        }
    });
    container.add_controller(gesture);

    (container, label)
}

/// Updates the label with the current date and time.
pub fn update(label: &Label) {
    let now = Local::now();
    let date_str = now.format("%a %b %d").to_string();
    let time_str = now.format("%H:%M").to_string();
    let text = format!(
        "<span color=\"#a4d1b4\">󰃭</span> <span color=\"#a4aea5\">{date_str}</span>   <span color=\"#a4d1b4\">󱑂</span> <span weight=\"bold\" color=\"#dee8df\">{time_str}</span>"
    );
    label.set_markup(&text);
    label.set_tooltip_text(Some(&now.format("%A, %B %e, %Y").to_string()));
}
