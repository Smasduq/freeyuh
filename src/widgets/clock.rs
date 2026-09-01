//! Clock widget: centered date + time label.

use chrono::Local;
use gtk4::prelude::*;
use gtk4::Label;

/// Creates the centered clock label.
pub fn create() -> Label {
    let label = Label::new(Some(""));
    label.add_css_class("clock");
    label.set_margin_top(6);
    label.set_margin_bottom(6);
    label
}

/// Updates the label with the current date and time.
pub fn update(label: &Label) {
    let now = Local::now();
    label.set_text(&now.format("%a %b %d  %H:%M").to_string());
}
