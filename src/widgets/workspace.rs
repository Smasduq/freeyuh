//! Workspace widget: dynamic pill and circle indicators for Hyprland workspaces.

use gtk4::prelude::*;
use gtk4::Box;

use crate::compositor::hyprland;

const SPACING: i32 = 6;

/// Creates the container that holds the workspace indicator buttons.
pub fn create() -> Box {
    let box_ = Box::new(gtk4::Orientation::Horizontal, SPACING);
    box_.add_css_class("workspaces");
    box_.set_halign(gtk4::Align::Start);
    box_.set_valign(gtk4::Align::Center);
    box_.set_margin_start(4);
    box_.set_margin_end(4);
    box_
}

/// (Re)builds the workspace list: only workspaces that currently have windows,
/// followed by one "free" slot that opens the next workspace.
pub fn refresh(container: &Box) {
    clear_children(container);

    let workspaces = hyprland::workspaces();
    let active = hyprland::active_workspace();

    // Only workspaces that currently hold windows.
    let mut ids: Vec<i64> = workspaces
        .iter()
        .filter(|ws| ws.has_windows)
        .map(|ws| ws.id)
        .collect();
    ids.sort_unstable();
    ids.dedup();

    // Ensure the active workspace is always represented.
    if !ids.contains(&active) && active > 0 {
        ids.push(active);
        ids.sort_unstable();
    }

    for &id in &ids {
        container.append(&WorkspaceButton::new(id, true, id == active, false).0);
    }

    // Free slot: the next workspace after the highest one with windows.
    let next_id = ids.last().map(|last| last + 1).unwrap_or(1);
    if next_id <= hyprland::max_workspace() && !ids.contains(&next_id) {
        container.append(&WorkspaceButton::new(next_id, false, false, true).0);
    }
}

/// Remove every child widget from `container`.
fn clear_children(container: &Box) {
    let model = container.observe_children();
    let mut to_remove = Vec::new();
    for i in 0..model.n_items() {
        if let Some(obj) = model.item(i) {
            if let Ok(w) = obj.downcast::<gtk4::Widget>() {
                to_remove.push(w);
            }
        }
    }
    for w in to_remove {
        container.remove(&w);
    }
}

/// A single button representing one workspace.
struct WorkspaceButton(gtk4::Button);

impl WorkspaceButton {
    fn new(id: i64, has_windows: bool, active: bool, free: bool) -> Self {
        let btn = gtk4::Button::new();
        btn.add_css_class("ws");
        btn.set_cursor_from_name(Some("pointer"));

        if free {
            btn.add_css_class("free");
            btn.set_tooltip_text(Some(&format!("Open Workspace {id}")));
        } else if has_windows {
            btn.add_css_class("occupied");
            btn.set_tooltip_text(Some(&format!("Workspace {id}")));
        }

        if active {
            btn.add_css_class("active");
            btn.set_tooltip_text(Some(&format!("Workspace {id} (Active)")));
        }

        btn.connect_clicked(move |_| {
            hyprland::switch_workspace(id);
        });
        Self(btn)
    }
}
