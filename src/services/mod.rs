//! System services.
//!
//! A "service" is a background capability the bar surface can subscribe to —
//! audio, wifi, bluetooth, etc. Each service owns its state query, mutation
//! commands, and a background event producer that emits unified [`Event`]s
//! when its state changes.

pub mod audio;
pub mod notifications;
