/// Wayland compositor backends.
///
/// This module abstracts away the compositor-specific IPC so the rest of the
/// app does not need to know whether it is talking to Hyprland, wayfire, etc.
/// Currently only [hyprland] is implemented.

pub mod hyprland;

/// A workspace as reported by the compositor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Workspace {
    pub id: i64,
    pub has_windows: bool,
}

/// Container for properties shared between compositor modules.
pub const MAX_WORKSPACES: i64 = 10;
