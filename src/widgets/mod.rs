//! Bar widgets: the dockable pieces that make up the bar.
//!
//! Each module owns one widget type and provides a `create()` constructor
//! plus its own refresh/update logic.

pub mod clock;
pub mod launcher;
pub mod notifications;
pub mod powermenu;
pub mod quicksettings;
pub mod sysinfo;
pub mod window;
pub mod workspace;
