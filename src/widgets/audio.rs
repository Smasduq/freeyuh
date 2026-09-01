//! Audio widget: a pill showing the current volume icon and percentage.
//!
//! Interaction (via GTK4 event controllers):
//! - Left click toggles mute.
//! - Mouse wheel up/down raises/lowers the volume.

use gtk4::prelude::*;
use gtk4::{EventControllerScroll, GestureClick, Label};

use crate::services::audio as audio_svc;

const VOLUME_STEP: i8 = 5;

/// Creates the audio pill shown on the right side of the bar.
pub fn create() -> Label {
    let label = Label::new(Some(" 󰕾 --%"));
    label.set_use_markup(true);
    label.add_css_class("sys-item");
    label.add_css_class("audio");

    // Left click toggles mute.
    let gesture = GestureClick::new();
    gesture.set_button(1);
    let label_click = label.clone();
    gesture.connect_pressed(move |_, _n, _x, _y| {
        audio_svc::toggle_mute();
        refresh(&label_click);
    });
    label.add_controller(gesture);

    // Mouse wheel adjusts volume.
    let scroll = EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let label_scroll = label.clone();
    scroll.connect_scroll(move |_, _dx, dy| {
        if dy > 0.0 {
            audio_svc::change_volume(-VOLUME_STEP);
        } else if dy < 0.0 {
            audio_svc::change_volume(VOLUME_STEP);
        }
        refresh(&label_scroll);
        glib::Propagation::Stop
    });
    label.add_controller(scroll);

    label
}

/// Re-reads the audio state and redraws the pill.
pub fn refresh(label: &Label) {
    match audio_svc::query() {
        Some(state) => {
            let icon = audio_svc::icon(&state);
            // Render the icon glyph larger than the percentage via markup.
            let text =
                format!(" <span size=\"large\">{icon}</span> {percent}%", percent = state.volume_percent);
            label.set_markup(&text);
            if state.muted {
                label.add_css_class("muted");
                label.remove_css_class("unmuted");
            } else {
                label.add_css_class("unmuted");
                label.remove_css_class("muted");
            }
        }
        None => {
            label.set_markup(" <span size=\"large\">󰚌</span>");
        }
    }
}
