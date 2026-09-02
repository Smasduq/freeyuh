//! System services.
//!
//! A "service" is a background capability the bar surface can subscribe to —
//! audio, wifi, bluetooth, etc. Each service owns its state query, mutation
//! commands, and a background event producer that emits unified [`Event`]s
//! when its state changes.

pub mod apps;
pub mod audio;
pub mod bluetooth;
pub mod brightness;
pub mod network;
pub mod notifications;
pub mod weather;
