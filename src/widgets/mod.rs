//! Bar widgets: the dockable pieces that make up the bar.
//!
//! Each module owns one widget type and provides a `create()` constructor
//! plus its own refresh/update logic.

pub mod audio;
pub mod clock;
pub mod network;
pub mod notifications;
pub mod sysinfo;
pub mod window;
pub mod workspace;
