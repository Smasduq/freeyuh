//! Active window title widget.
//!
//! Shows the title of the currently focused window with a clean glyph,
//! ellipsizing long titles so the bar layout never shifts awkwardly.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::Label;

use crate::compositor::hyprland;

const MAX_WIDTH_CHARS: i32 = 38;

/// Creates the active window title label.
pub fn create() -> Label {
    let label = Label::new(None);
    label.add_css_class("active-window");
    label.set_halign(gtk4::Align::Start);
    label.set_valign(gtk4::Align::Center);
    label.set_use_markup(true);
    label.set_ellipsize(EllipsizeMode::End);
    label.set_max_width_chars(MAX_WIDTH_CHARS);
    label.set_margin_start(4);
    label
}

/// Updates the label with the provided window title.
pub fn update(label: &Label, title: Option<&str>) {
    match title {
        Some(t) if !t.is_empty() => {
            let escaped = glib::markup_escape_text(t);
            label.set_markup(&format!("<span color=\"#a4d1b4\">󰖲</span>  {escaped}"));
            label.set_tooltip_text(Some(t));
            label.set_visible(true);
        }
        _ => {
            label.set_text("");
            label.set_tooltip_text(None);
            label.set_visible(false);
        }
    }
}

/// Query Hyprland and refresh the active window label.
pub fn refresh(label: &Label) {
    let title = hyprland::active_window_title();
    update(label, title.as_deref());
}
