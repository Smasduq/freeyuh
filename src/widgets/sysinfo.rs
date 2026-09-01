//! System info widget group: CPU, memory and battery.
//!
//! Each metric lives in its own labelled pill so the system section reads
//! clearly instead of being one mixed blob of text.

use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};
use sysinfo::System;

/// Creates the system-info container shown on the right side, holding one
/// labelled pill per metric (CPU, memory, battery).
///
/// The battery is split into two pills so the icon and the percentage are not
/// visually mashed together. Returned labels order:
/// `[cpu, mem, battery-icon, battery-percent]`.
pub fn create() -> (Box, Vec<Label>) {
    let container = Box::new(Orientation::Horizontal, 2);
    container.add_css_class("sysinfo");
    container.set_halign(gtk4::Align::End);
    container.set_valign(gtk4::Align::Center);
    container.set_margin_end(10);

    // CPU
    let cpu_pill = make_pill("cpu", "  --%");
    // Memory
    let mem_pill = make_pill("mem", "  --");
    // Battery icon (no value)
    let bat_icon = make_pill("bat-icon", " ");
    // Battery percentage
    let bat_pct = make_pill("bat", " --%");

    container.append(&cpu_pill);
    container.append(&mem_pill);
    container.append(&bat_icon);
    container.append(&bat_pct);

    let labels = vec![cpu_pill, mem_pill, bat_icon, bat_pct];
    (container, labels)
}

/// Build a single pill: a small caption with a value label.
fn make_pill(name: &str, initial: &str) -> Label {
    let label = Label::new(Some(initial));
    label.add_css_class("sys-item");
    label.add_css_class(name);
    label
}

/// Refresh CPU and memory pills only.
///
/// `labels` order must match `create()`: CPU, memory, battery-icon, battery-pct.
pub fn update_system(labels: &[Label]) {
    let (cpu, mem_used, mem_total) = sys_stats();

    if let Some(cpu_label) = labels.first() {
        cpu_label.set_text(&format!("  {cpu:>3}%"));
    }
    if let Some(mem_label) = labels.get(1) {
        mem_label.set_text(&format!("  {mem_used}MB/{mem_total}MB"));
    }
}

/// Refresh the battery icon and percentage pills only (called on
/// [`crate::events::Event::BatteryChanged`]).
pub fn update_battery(labels: &[Label]) {
    let battery = battery_state();

    if let Some(bat_icon) = labels.get(2) {
        bat_icon.set_text(&battery_icon_label(battery));
    }
    if let Some(bat_pct) = labels.get(3) {
        bat_pct.set_text(&battery_pct_label(battery));
    }
}

/// Convenience: refresh all system pills at once (used for the initial draw).
pub fn update(labels: &[Label]) {
    update_system(labels);
    update_battery(labels);
}

/// Render the battery icon pill, or "--" when unknown.
fn battery_icon_label(battery: Option<(u8, bool)>) -> String {
    match battery {
        Some((percent, charging)) => {
            if charging {
                " ".to_string()
            } else {
                format!(" {}", battery_icon(percent))
            }
        }
        None => " ".to_string(),
    }
}

/// Render the battery percentage pill, or "--" when unknown.
fn battery_pct_label(battery: Option<(u8, bool)>) -> String {
    match battery {
        Some((percent, _)) => format!(" {percent}%"),
        None => " --%".to_string(),
    }
}

/// Pick a battery icon by charge level.
fn battery_icon(percent: u8) -> &'static str {
    match percent {
        0..=20 => "",
        21..=40 => "",
        41..=60 => "",
        61..=80 => "",
        _ => "",
    }
}

/// Reads CPU usage and memory figures. The `System` handle lives in a
/// thread-local because `refresh_*` requires `&mut self` and this widget is
/// only ever refreshed from the main thread.
fn sys_stats() -> (u8, u64, u64) {
    thread_local! {
        static SYS: std::cell::RefCell<System> = std::cell::RefCell::new(System::new());
    }
    SYS.with(|sys| {
        let mut sys = sys.borrow_mut();
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        let cpu = sys.global_cpu_usage() as u8;
        let mem_total = sys.total_memory() / 1024 / 1024; // MiB
        let mem_used = sys.used_memory() / 1024 / 1024; // MiB
        (cpu, mem_total, mem_used)
    })
}

/// Returns `(percentage, charging)` aggregated across all batteries, or `None`
/// when no battery is present. Exposed for the event bus so it can detect
/// changes without re-reading the UI.
pub fn battery_state() -> Option<(u8, bool)> {
    let mut now: u64 = 0;
    let mut full: u64 = 0;
    let mut charging = false;

    let power_supplies = std::fs::read_dir("/sys/class/power_supply").ok()?;

    for entry in power_supplies.flatten() {
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_none_or(|n| !n.starts_with("BAT"))
        {
            continue;
        }
        let typ = std::fs::read_to_string(path.join("type")).unwrap_or_default();
        if typ.trim() != "Battery" {
            continue;
        }

        let Ok(n) = std::fs::read_to_string(path.join("energy_now")) else {
            continue;
        };
        let Ok(f) = std::fs::read_to_string(path.join("energy_full")) else {
            continue;
        };
        let Ok(n) = n.trim().parse::<u64>() else { continue };
        let Ok(f) = f.trim().parse::<u64>() else { continue };

        now += n;
        full += f;

        let status = std::fs::read_to_string(path.join("status")).unwrap_or_default();
        if status.trim() == "Charging" {
            charging = true;
        }
    }

    if full == 0 {
        return None;
    }

    let percent = ((now as f64 / full as f64) * 100.0).round() as u8;
    Some((percent, charging))
}
