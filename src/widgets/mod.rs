//! Bar widgets: the dockable pieces that make up the bar.
//!
//! Each module owns one widget type and provides a `create()` constructor
//! plus its own refresh/update logic.

pub mod clock;
pub mod sysinfo;
pub mod workspace;
