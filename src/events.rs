//! Unified event bus.
//!
//! Every change the bar cares about — compositor workspace changes, clock
//! rollover, and system metric updates — is represented as an [`Event`] and
//! pushed onto a single channel. A single main-thread reactor in `app.rs`
//! drains the channel and dispatches each event to the right widget update.
//!
//! Producers run on background threads and emit events only when something
//! actually changed, so widgets update on demand rather than on a fixed timer.

use std::sync::mpsc::Sender;

/// A single change the bar should react to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The active workspace changed to `id`.
    WorkspaceActive(i64),
    /// The set of workspaces changed (created/destroyed/rearranged).
    WorkspaceListChanged,
    /// The clock's displayed minute rolled over.
    ClockTick,
    /// CPU/memory figures should be re-read.
    SystemTick,
    /// Battery percentage or charging state changed.
    BatteryChanged,
    /// The default audio sink's volume or mute changed.
    AudioChanged,
    /// A new notification arrived.
    Notification(Notification),
    /// An active notification was dismissed.
    NotificationClosed { id: u32 },
    /// The title of the currently focused window changed.
    ActiveWindow(Option<String>),
    /// Network connection state or SSID changed.
    NetworkChanged,
    /// Show the notification center window.
    ShowNotificationCenter,
    /// Hide the notification center window.
    HideNotificationCenter,
    /// Clear all notifications from history.
    ClearAllNotifications,
    /// Dismiss a specific notification from history.
    DismissNotification(u32),
}

/// A single notification received from the D-Bus notification daemon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub id: u32,
    /// Name of the sending application (its `.desktop` id or process name).
    pub app_name: String,
    /// The notification title.
    pub summary: String,
    /// The notification body text.
    pub body: String,
    /// Urgency level reported by the sender.
    pub urgency: Urgency,
}

/// Urgency level of a notification, per the freedesktop spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

/// A producer for [`Event`]s: a handle to the background thread whose events
/// are sent on a shared channel.
pub struct EventProducer {
    _thread: Option<std::thread::JoinHandle<()>>,
}

impl EventProducer {
    fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            _thread: Some(std::thread::spawn(f)),
        }
    }

    /// A handle that owns no thread (used for producers that manage their own
    /// lifecycle, e.g. the notification daemon).
    fn none() -> Self {
        Self { _thread: None }
    }
}

/// Spawn the Hyprland event listener that emits workspace events.
pub fn spawn_hyprland(tx: Sender<Event>) -> EventProducer {
    EventProducer::spawn(move || {
        crate::compositor::hyprland::listen(tx);
    })
}

/// Spawn a producer that emits [`Event::ClockTick`] whenever the displayed
/// minute changes and [`Event::SystemTick`] on a coarse interval.
pub fn spawn_tickers(tx: Sender<Event>) -> EventProducer {
    EventProducer::spawn(move || {
        use chrono::Local;
        let mut last_minute = String::new();
        let mut samples: u64 = 0;
        loop {
            let now = Local::now();
            let minute = now.format("%Y%m%d%H%M").to_string();
            if minute != last_minute {
                last_minute = minute;
                let _ = tx.send(Event::ClockTick);
            }
            // System figures change frequently but not constantly; sample on
            // a modest interval (every 4th second).
            samples += 1;
            if samples % 4 == 0 {
                let _ = tx.send(Event::SystemTick);
            }
            if samples % 6 == 0 {
                let _ = tx.send(Event::NetworkChanged);
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    })
}

/// Spawn a producer that watches the power supply and emits
/// [`Event::BatteryChanged`] when the percentage or charging state changes.
pub fn spawn_battery(tx: Sender<Event>) -> EventProducer {
    EventProducer::spawn(move || {
        let mut last: Option<(u8, bool)> = None;
        loop {
            let current = crate::widgets::sysinfo::battery_state();
            if current != last {
                last = current;
                let _ = tx.send(Event::BatteryChanged);
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    })
}

/// Spawn the audio service producer that emits [`Event::AudioChanged`] on any
/// volume/mute change on the default output sink.
pub fn spawn_audio(tx: Sender<Event>) -> EventProducer {
    EventProducer::spawn(move || {
        crate::services::audio::listen(tx);
    })
}

/// Spawn the notification daemon, which owns `org.freedesktop.Notifications`
/// on the session bus and emits [`Event::Notification`] /
/// [`Event::NotificationClosed`] as notifications arrive.
pub fn spawn_notifications(tx: Sender<Event>) -> EventProducer {
    crate::services::notifications::spawn(tx);
    // The daemon manages its own thread; return a no-op handle so the caller
    // keeps a uniform producer list.
    EventProducer::none()
}
