use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::mpsc::Sender;
use std::time::Duration;

use chrono::Local;
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box, Button, Label, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use sysinfo::System;

const APP_ID: &str = "dev.freeyuh.shell";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

// ---------------------------------------------------------------------------
// Hyprland IPC (direct socket, compatible with Hyprland >= 0.55 / Lua dispatch)
// ---------------------------------------------------------------------------
const MAX_WS: i64 = 10;

fn hypr_dir() -> Option<std::path::PathBuf> {
    let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".into());
    Some(std::path::PathBuf::from(format!("{runtime}/hypr/{sig}")))
}

/// Send a raw command to the command socket and return the reply.
fn hypr_call(command: &str) -> Option<String> {
    let base = hypr_dir()?;
    let path = base.join(".socket.sock");
    let mut stream = UnixStream::connect(path).ok()?;
    stream.write_all(command.as_bytes()).ok()?;
    let mut reply = String::new();
    stream.read_to_string(&mut reply).ok()?;
    Some(reply)
}

/// List workspaces (id, has_windows) using `j/workspaces`.
fn hypr_workspaces() -> Vec<(i64, bool)> {
    let out = match hypr_call("j/workspaces") {
        Some(o) => o,
        None => return Vec::new(),
    };
    let mut result = Vec::new();
    let val: serde_json::Value = match serde_json::from_str(&out) {
        Ok(v) => v,
        Err(_) => return result,
    };
    let arr = match val.as_array() {
        Some(a) => a,
        None => return result,
    };
    for ws in arr {
        let id = ws.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
        let windows = ws.get("windows").and_then(|v| v.as_u64()).unwrap_or(0);
        if id > 0 {
            result.push((id, windows > 0));
        }
    }
    result
}

/// The active workspace id via `activeworkspace` (last line "workspace ID n").
fn hypr_active_workspace() -> i64 {
    let out = match hypr_call("activeworkspace") {
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
fn hypr_switch(workspace_id: i64) {
    let cmd = format!("dispatch hl.dsp.focus({{ workspace = {workspace_id} }})");
    let _ = hypr_call(&cmd);
}

/// Spawn a background thread that tails the event socket and forwards
/// workspace-related events to `tx`. Events are strings:
///   "active <id>"     - workspace changed
///   "reload"          - a workspace was created/destroyed
fn hypr_event_listener(tx: Sender<String>) {
    std::thread::spawn(move || {
        let base = match hypr_dir() {
            Some(b) => b,
            None => return,
        };
        let path = base.join(".socket2.sock");

        // Keep reconnecting so the bar stays alive across compositor restarts.
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
                    if line.starts_with("workspace>>") {
                        let id = line.trim_start_matches("workspace>>");
                        let _ = tx.send(format!("active {}", id));
                    } else if line.starts_with("createworkspace>>")
                        || line.starts_with("destroyworkspace>>")
                    {
                        let _ = tx.send("reload".to_string());
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

// ---------------------------------------------------------------------------
// GTK UI
// ---------------------------------------------------------------------------
struct WorkspaceButton {
    button: Button,
}

impl WorkspaceButton {
    fn new(id: i64, has_windows: bool, active: bool, free: bool) -> Self {
        let btn = Button::new();
        btn.add_css_class("ws");
        if free {
            btn.add_css_class("free");
        } else if has_windows {
            btn.add_css_class("occupied");
        }
        if active {
            btn.add_css_class("active");
        }
        let target = id;
        btn.connect_clicked(move |_| {
            hypr_switch(target);
        });
        Self { button: btn }
    }
}

fn build_ui(app: &Application) {
    let bar_height = 34;

    let window = ApplicationWindow::builder().application(app).build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(bar_height);
    window.set_default_size(-1, bar_height);

    let root = Box::new(Orientation::Horizontal, 0);
    root.add_css_class("bar");
    root.set_halign(Align::Fill);
    window.set_child(Some(&root));

    // ---- Left: workspaces ----
    let workspaces_box = Box::new(Orientation::Horizontal, 4);
    workspaces_box.add_css_class("workspaces");
    workspaces_box.set_halign(Align::Start);
    workspaces_box.set_valign(Align::Center);
    workspaces_box.set_margin_start(10);

    // ---- Center: clock ----
    let clock_label = Label::new(Some(""));
    clock_label.add_css_class("clock");
    clock_label.set_margin_top(6);
    clock_label.set_margin_bottom(6);

    let center = Box::new(Orientation::Horizontal, 0);
    center.set_halign(Align::Center);
    center.set_valign(Align::Center);
    center.append(&clock_label);

    // ---- Right: system info ----
    let sys_label = Label::new(Some(""));
    sys_label.add_css_class("sysinfo");
    sys_label.set_margin_top(6);
    sys_label.set_margin_bottom(6);

    let right = Box::new(Orientation::Horizontal, 12);
    right.set_halign(Align::End);
    right.set_valign(Align::Center);
    right.set_margin_end(10);
    right.append(&sys_label);

    root.append(&workspaces_box);
    root.append(&center);
    root.append(&right);

    center.set_hexpand(true);
    workspaces_box.set_hexpand(true);
    right.set_hexpand(true);

    load_css();

    // Initial render (best-effort; never panics)
    refresh_workspaces(&workspaces_box);

    // ---- Clock + system info refresh ----
    let clock2 = clock_label.clone();
    let sys2 = sys_label.clone();
    glib::timeout_add_local(Duration::from_millis(1000), move || {
        update_clock(&clock2);
        update_sysinfo(&sys2);
        glib::ControlFlow::Continue
    });

    // ---- Hyprland events -> main thread ----
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let wb_main = workspaces_box.clone();
    glib::timeout_add_local(Duration::from_millis(300), move || {
        loop {
            match rx.try_recv() {
                Ok(msg) => {
                    // Rebuild on any workspace change so that occupied
                    // workspaces and the free slot always reflect reality.
                    if msg.starts_with("active ") || msg == "reload" {
                        refresh_workspaces(&wb_main);
                    }
                }
                Err(_) => break,
            }
        }
        glib::ControlFlow::Continue
    });

    hypr_event_listener(tx);

    window.present();
}

/// (Re)build the workspace list. Shows only workspaces that currently have
/// windows, plus one "free" circle at the end that opens the next workspace.
fn refresh_workspaces(container: &gtk4::Box) {
    clear_children(container);

    let workspaces = hypr_workspaces();
    let active = hypr_active_workspace();

    // Only workspace ids that currently have windows.
    let mut ids: Vec<i64> = workspaces
        .iter()
        .filter(|(_, has_windows)| *has_windows)
        .map(|(id, _)| *id)
        .collect();
    ids.sort_unstable();
    ids.dedup();

    for &id in &ids {
        let ws = WorkspaceButton::new(id, true, id == active, false);
        container.append(&ws.button);
    }

    // Free slot: the next workspace after the highest one with windows.
    let next_id = ids.last().map(|last| last + 1).unwrap_or(1);
    if next_id <= MAX_WS {
        let free = WorkspaceButton::new(next_id, false, false, true);
        container.append(&free.button);
    }
}

fn clear_children(container: &gtk4::Box) {
    // Collect first, then remove. Removing by index while iterating shifts
    // the remaining children and leaves some behind (causing duplicates).
    let model = container.observe_children();
    let mut to_remove = Vec::new();
    for i in 0..model.n_items() {
        if let Some(obj) = model.item(i) {
            if let Ok(w) = obj.downcast::<gtk4::Widget>() {
                to_remove.push(w);
            }
        }
    }
    for w in to_remove {
        container.remove(&w);
    }
}

fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(CSS);
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("no display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn update_clock(label: &Label) {
    let now = Local::now();
    label.set_text(&now.format("%a %b %d  %H:%M").to_string());
}

fn update_sysinfo(label: &Label) {
    thread_local! {
        static SYS: std::cell::RefCell<System> = std::cell::RefCell::new(System::new());
    }

    let (cpu, mem_used, mem_total) = SYS.with(|sys| {
        let mut sys = sys.borrow_mut();
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        let cpu = sys.global_cpu_usage() as u8;
        let mem_total = sys.total_memory() / 1024 / 1024; // MiB
        let mem_used = sys.used_memory() / 1024 / 1024; // MiB
        (cpu, mem_total, mem_used)
    });

    let battery = battery_info();
    let bat_str = match battery {
        Some((percent, charging)) => {
            let icon = if charging { " " } else { " " };
            format!("{icon} {percent}%")
        }
        None => String::new(),
    };

    let text = format!("  {cpu:>3}%    {mem_used}MB/{mem_total}MB{bat_str}");
    label.set_text(&text);
}

fn battery_info() -> Option<(u8, bool)> {
    for bat in ["BAT0", "BAT1", "BAT2"] {
        let base = format!("/sys/class/power_supply/{bat}");
        let capacity = std::fs::read_to_string(format!("{base}/capacity"))
            .ok()?
            .trim()
            .parse()
            .ok()?;
        let status = std::fs::read_to_string(format!("{base}/status")).unwrap_or_default();
        let charging = status.trim() == "Charging";
        return Some((capacity, charging));
    }
    None
}

const CSS: &str = r#"
.bar {
    background: rgba(20, 20, 20, 0.85);
    font-size: 13px;
    color: #d8dee9;
}
.workspaces button {
    background: transparent;
    border: 2px solid #4a5568;
    border-radius: 999px;
    min-width: 9px;
    min-height: 9px;
    padding: 0;
    margin: 0 3px;
}
.workspaces button:hover {
    border-color: #ffffff;
    background: rgba(255,255,255,0.10);
}
.workspaces button.free {
    border: 2px dashed #5a6474;
    opacity: 0.75;
}
.workspaces button.occupied {
    background: #d8dee9;
    border-color: #d8dee9;
}
.workspaces button.active {
    background: #5294e2;
    border-color: #5294e2;
    border-radius: 999px;
    min-width: 13px;
    min-height: 13px;
}
.clock {
    padding: 4px 10px;
    background: rgba(255,255,255,0.06);
    border-radius: 12px;
}
.sysinfo {
    text-shadow: 0 1px 2px rgba(0,0,0,0.4);
    font-weight: bold;
    color: #a8b2c1;
}
"#;
