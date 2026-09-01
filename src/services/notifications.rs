//! Notification service.
//!
//! Owns `org.freedesktop.Notifications` on the session bus and implements the
//! freedesktop notification spec, so this bar is the notification daemon.
//!
//! Because a desktop shell (e.g. a Quickshell config) may already own the
//! name, the connection is requested with `REPLACE_EXISTING` so the bar takes
//! over and becomes the single notification daemon. Incoming notifications are
//! forwarded into the unified event bus, where the toast popup and
//! notification-center widgets subscribe.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc::Sender};

use zbus::{interface, Connection, NameFlag};

use crate::events::{Event, Notification, Urgency};

const SERVICE_NAME: &str = "org.freedesktop.Notifications";
const OBJECT_PATH: &str = "/org/freedesktop/Notifications";

/// Shared mutable state behind the D-Bus interface. The interface object must
/// be `Send`-friendly, so all mutable state lives behind an `Arc<Mutex<..>>`.
#[derive(Clone, Default)]
struct Store {
    /// Per-connection incrementing notification id counter.
    next_id: Arc<Mutex<u32>>,
    /// Active notifications keyed by their assigned id.
    active: Arc<Mutex<HashMap<u32, Notification>>>,
}

/// The notification daemon, exposed over D-Bus.
pub struct NotificationService {
    tx: Sender<Event>,
    store: Store,
    // Keep the service a "services::notifications" façade for callers.
    _private: (),
}

impl NotificationService {
    fn new(tx: Sender<Event>) -> Self {
        Self {
            tx,
            store: Store::default(),
            _private: (),
        }
    }

    fn next_id(&self) -> u32 {
        let mut guard = self.store.next_id.lock().unwrap();
        *guard += 1;
        *guard
    }
}

impl NotificationService {
    fn send(&self, event: Event) {
        let _ = self.tx.send(event);
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationService {
    /// Called by clients (e.g. `notify-send`) to post a notification.
    async fn notify(
        &self,
        #[zbus(signature = "s")] app_name: String,
        #[zbus(signature = "u")] replaces_id: u32,
        #[zbus(signature = "s")] _app_icon: String,
        #[zbus(signature = "s")] summary: String,
        #[zbus(signature = "s")] body: String,
        #[zbus(signature = "as")] _actions: Vec<String>,
        #[zbus(signature = "a{sv}")] hints: zbus::zvariant::OwnedValue,
        #[zbus(signature = "i")] _expire_timeout: i32,
    ) -> u32 {
        // A `replaces_id` cherry-picked from an earlier app request bumps the
        // same notification rather than creating a new one.
        let mut id = if replaces_id != 0 {
            replaces_id
        } else {
            self.next_id()
        };

        let urgency = parse_urgency(&hints);

        let notif = Notification {
            id,
            app_name,
            summary,
            body,
            urgency,
        };

        // Store (id -> notification) so CloseNotification can resolve it.
        {
            let mut active = self.store.active.lock().unwrap();
            if replaces_id != 0 {
                // Re-replacing keeps the slot; but keep id unique by using the
                // stored id if present.
                if let Some(existing) = active.get(&id) {
                    id = existing.id;
                }
                active.insert(id, notif.clone());
            } else {
                active.insert(id, notif.clone());
            }
        }

        self.send(Event::Notification(notif));
        id
    }

    /// Called by the origin client or other apps to close a notification.
    async fn close_notification(&self, id: u32) {
        if self.store.active.lock().unwrap().remove(&id).is_some() {
            self.send(Event::NotificationClosed { id });
        }
    }

    /// Advertised capabilities.
    async fn get_capabilities(&self) -> Vec<String> {
        vec![
            "body".into(),
            "body-markup".into(),
            "icon-static".into(),
            "actions".into(),
            "persistence".into(),
        ]
    }

    /// Identifies this daemon to clients.
    async fn get_server_information(&self) -> (String, String, String, String) {
        (
            "freeyuh".into(),
            "freeyuh".into(),
            env!("CARGO_PKG_VERSION").into(),
            "1.2".into(),
        )
    }
}

/// Parse the `urgency` hint (0=low, 1=normal, 2=critical, default normal).
fn parse_urgency(hints: &zbus::zvariant::OwnedValue) -> Urgency {
    // hints serialize as a dictionary (`a{sv}`). Look up "urgency".
    if let Ok(dict) = hints.try_clone().downcast::<HashMap<String, zbus::zvariant::OwnedValue>>() {
        if let Some(v) = dict.get("urgency") {
            if let Ok(byte) = v.try_clone().downcast::<u8>() {
                return match byte {
                    0 => Urgency::Low,
                    2 => Urgency::Critical,
                    _ => Urgency::Normal,
                };
            }
        }
    }
    Urgency::Normal
}

/// Spawn the notification daemon on a dedicated thread with its own async
/// runtime. Takes over the `org.freedesktop.Notifications` name if another
/// process already owns it.
pub fn spawn(tx: Sender<Event>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build notification runtime");
        rt.block_on(async move {
            let conn = match Connection::session().await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("notifications: no session bus: {e}");
                    return;
                }
            };
            if let Err(e) = conn
                .request_name(SERVICE_NAME, NameFlag::ReplaceExisting | NameFlag::AllowReplacement)
                .await
            {
                eprintln!("notifications: could not acquire {SERVICE_NAME}: {e}");
                return;
            }

            let service = NotificationService::new(tx);
            if let Err(e) = conn
                .object_server()
                .at(OBJECT_PATH, service)
                .await
            {
                eprintln!("notifications: failed to register service: {e}");
                return;
            }

            // Keep the daemon alive indefinitely.
            std::future::pending::<()>().await;
        });
    });
}
