//! Unix Domain Socket IPC for Freeyuh.
//!
//! Enables external commands (e.g. Hyprland hotkeys or scripts) to trigger
//! shell actions like toggling the Quick Settings or Notification Center.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use crate::events::Event;

/// Returns the standard path for the Freeyuh IPC socket.
pub fn socket_path() -> PathBuf {
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        PathBuf::from(runtime_dir).join("freeyuh.sock")
    } else {
        let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
        PathBuf::from(format!("/tmp/freeyuh-{user}.sock"))
    }
}

/// Send a command to the running Freeyuh daemon.
pub fn send_command(cmd: &str) -> Result<String, String> {
    let path = socket_path();
    let mut stream = UnixStream::connect(&path)
        .map_err(|e| format!("Could not connect to Freeyuh daemon at {:?}: {}", path, e))?;

    writeln!(stream, "{}", cmd.trim())
        .map_err(|e| format!("Failed to send command: {}", e))?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(response.trim().to_string())
}

/// Spawns the IPC server thread in the daemon.
pub fn spawn_server(tx: Sender<Event>) {
    let path = socket_path();
    let _ = std::fs::remove_file(&path);

    std::thread::spawn(move || {
        let listener = match UnixListener::bind(&path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("ipc: failed to bind socket at {:?}: {}", path, e);
                return;
            }
        };

        for stream in listener.incoming().flatten() {
            let tx_cl = tx.clone();
            let mut reader = BufReader::new(stream.try_clone().unwrap_or_else(|_| stream));
            let mut line = String::new();
            if reader.read_line(&mut line).is_ok() {
                let cmd = line.trim();
                let mut response = "ok";

                match cmd {
                    "toggle-quicksettings" | "toggle-qs" | "qs" => {
                        let _ = tx_cl.send(Event::ToggleQuickSettings);
                    }
                    "toggle-notifications" | "toggle-notifs" | "notifs" => {
                        let _ = tx_cl.send(Event::ToggleNotifications);
                    }
                    "reload-style" | "reload" => {
                        let _ = tx_cl.send(Event::ReloadStyle);
                    }
                    "volume-mute" | "mute" => {
                        crate::services::audio::toggle_mute();
                        let _ = tx_cl.send(Event::AudioChanged);
                    }
                    "wifi-toggle" => {
                        let next = !crate::services::network::wifi_enabled();
                        crate::services::network::set_wifi_enabled(next);
                        let _ = tx_cl.send(Event::NetworkChanged);
                    }
                    "bluetooth-toggle" | "bt-toggle" => {
                        let next = !crate::services::bluetooth::is_enabled();
                        let _ = crate::services::bluetooth::set_enabled(next);
                        let _ = tx_cl.send(Event::BluetoothChanged);
                    }
                    other => {
                        if other.starts_with("volume-set ") {
                            if let Ok(pct) = other[11..].trim().parse::<u8>() {
                                crate::services::audio::set_volume(pct);
                                let _ = tx_cl.send(Event::AudioChanged);
                            }
                        } else {
                            response = "unknown command";
                        }
                    }
                }

                if let Ok(mut writer) = reader.into_inner().try_clone() {
                    let _ = writeln!(writer, "{response}");
                }
            }
        }
    });
}
