//! Notification service.
//!
//! Owns `org.freedesktop.Notifications` on the session bus and implements the
//! freedesktop notification spec, so this bar is the notification daemon.
//!
//! Because a desktop shell (e.g. a Quickshell config) may already own the
//! name, the connection is built with `replace_existing_names(true)` so the
//! bar takes over and becomes the single notification daemon. Incoming
//! notifications are forwarded into the unified event bus, where the toast
//! popup and notification-center widgets subscribe.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc::Sender};

use zbus::zvariant::OwnedValue;
use zbus::interface;

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
}

impl NotificationService {
    fn new(tx: Sender<Event>) -> Self {
        Self {
            tx,
            store: Store::default(),
        }
    }

    fn next_id(&self) -> u32 {
        let mut guard = self.store.next_id.lock().unwrap();
        *guard += 1;
        *guard
    }

    fn send(&self, event: Event) {
        let _ = self.tx.send(event);
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationService {
    /// Called by clients (e.g. `notify-send`) to post a notification.
    async fn notify(
        &self,
        app_name: String,
        replaces_id: u32,
        _app_icon: String,
        summary: String,
        body: String,
        _actions: Vec<String>,
        hints: HashMap<String, OwnedValue>,
        _expire_timeout: i32,
    ) -> u32 {
        let id = if replaces_id != 0 {
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

        {
            let mut active = self.store.active.lock().unwrap();
            active.insert(id, notif.clone());
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
            "iFreeYuh".into(),
            "iFreeYuh".into(),
            env!("CARGO_PKG_VERSION").into(),
            "1.2".into(),
        )
    }
}

/// Parse the `urgency` hint (0=low, 1=normal, 2=critical, default normal).
fn parse_urgency(hints: &HashMap<String, OwnedValue>) -> Urgency {
    match hints.get("urgency") {
        Some(value) => match u8::try_from(value.clone()) {
            Ok(0) => Urgency::Low,
            Ok(2) => Urgency::Critical,
            _ => Urgency::Normal,
        },
        None => Urgency::Normal,
    }
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
            let service = NotificationService::new(tx);
            let builder = zbus::connection::Builder::session().ok();
            let Some(builder) = builder else {
                eprintln!("notifications: no session bus");
                return;
            };
            let builder = match builder.name(SERVICE_NAME).and_then(|b| {
                b.replace_existing_names(true)
                    .serve_at(OBJECT_PATH, service)
            }) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("notifications: could not configure service: {e}");
                    return;
                }
            };
            match builder.build().await {
                Ok(_conn) => {
                    // Keep the daemon alive indefinitely.
                    std::future::pending::<()>().await;
                }
                Err(e) => eprintln!("notifications: could not acquire {SERVICE_NAME}: {e}"),
            }
        });
    });
}
