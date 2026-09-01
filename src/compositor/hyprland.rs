//! Direct Hyprland IPC over its Unix sockets.
//!
//! This talks straight to Hyprland's `.socket.sock` (commands) and
//! `.socket2.sock` (events) so we are compatible with Hyprland >= 0.55 where
//! the Lua-based dispatcher is in use and the legacy `dispatch workspace N`
//! form no longer works.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::events::Event;

use super::{Workspace, MAX_WORKSPACES};

/// Directory holding the Hyprland IPC sockets.
fn hypr_dir() -> Option<std::path::PathBuf> {
    let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".into());
    Some(std::path::PathBuf::from(format!("{runtime}/hypr/{sig}")))
}

/// Send a raw command to the command socket and return the reply.
pub fn call(command: &str) -> Option<String> {
    let base = hypr_dir()?;
    let path = base.join(".socket.sock");
    let mut stream = UnixStream::connect(path).ok()?;
    stream.write_all(command.as_bytes()).ok()?;
    let mut reply = String::new();
    stream.read_to_string(&mut reply).ok()?;
    Some(reply)
}

/// List all workspaces known to the compositor.
pub fn workspaces() -> Vec<Workspace> {
    let out = match call("j/workspaces") {
        Some(o) => o,
        None => return Vec::new(),
    };
    let val: serde_json::Value = match serde_json::from_str(&out) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let arr = match val.as_array() {
        Some(a) => a,
        None => return Vec::new(),
    };

    arr.iter()
        .filter_map(|ws| {
            let id = ws.get("id")?.as_i64()?;
            if id <= 0 {
                return None;
            }
            let windows = ws.get("windows").and_then(|v| v.as_u64()).unwrap_or(0);
            Some(Workspace {
                id,
                has_windows: windows > 0,
            })
        })
        .collect()
}

/// The id of the currently active workspace.
pub fn active_workspace() -> i64 {
    let out = match call("activeworkspace") {
        Some(o) => o,
        None => return 1,
    };
    for line in out.lines() {
        if let Some(rest) = line.strip_prefix("workspace ID ") {
            if let Ok(n) = rest.split_whitespace().next().unwrap_or("").parse::<i64>() {
                return n;
            }
        }
    }
    1
}

/// Switch to a workspace using the Lua dispatcher (Hyprland >= 0.55).
pub fn switch_workspace(workspace_id: i64) {
    let cmd = format!("dispatch hl.dsp.focus({{ workspace = {workspace_id} }})");
    let _ = call(&cmd);
}

/// Tail the event socket and forward workspace changes to `tx` as unified
/// [`Event`]s. Reconnects automatically so the bar survives compositor
/// restarts. Blocks forever; call from a dedicated thread.
pub fn listen(tx: Sender<Event>) {
    let base = match hypr_dir() {
        Some(b) => b,
        None => return,
    };
    let path = base.join(".socket2.sock");

    // Keep reconnecting so the bar survives compositor restarts.
    loop {
        let Ok(mut stream) = UnixStream::connect(&path) else {
            std::thread::sleep(Duration::from_secs(2));
            continue;
        };
        let mut buf = [0u8; 4096];
        loop {
            let n = match stream.read(&mut buf) {
                Ok(n) if n > 0 => n,
                _ => break,
            };
            let chunk = String::from_utf8_lossy(&buf[..n]).into_owned();
            for line in chunk.lines() {
                if let Some(id) = line.strip_prefix("workspace>>") {
                    if let Ok(id) = id.trim().parse::<i64>() {
                        let _ = tx.send(Event::WorkspaceActive(id));
                    }
                } else if line.starts_with("createworkspace>>")
                    || line.starts_with("destroyworkspace>>")
                {
                    let _ = tx.send(Event::WorkspaceListChanged);
                } else if let Some(rest) = line.strip_prefix("activewindow>>") {
                    let title = if let Some((_, title)) = rest.split_once(',') {
                        let trimmed = title.trim();
                        if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
                    } else {
                        let trimmed = rest.trim();
                        if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
                    };
                    let _ = tx.send(Event::ActiveWindow(title));
                } else if line.starts_with("activewindowv2>>") {
                    let _ = tx.send(Event::ActiveWindow(active_window_title()));
                }
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

/// Query the title of the currently focused window, if any.
pub fn active_window_title() -> Option<String> {
    let out = call("j/activewindow")?;
    let val: serde_json::Value = serde_json::from_str(&out).ok()?;
    let title = val.get("title")?.as_str()?;
    let trimmed = title.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// The maximum workspace id this bar will render.
pub const fn max_workspace() -> i64 {
    MAX_WORKSPACES
}
