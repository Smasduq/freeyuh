//! System info widget group: CPU, memory and battery.
//!
//! Each metric lives in its own styled pill with distinct icons, tooltips,
//! and responsive status styling.

use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};
use sysinfo::System;

/// Creates the system-info container shown on the right side.
///
/// Returned labels order: `[cpu, mem, battery]`.
pub fn create() -> (Box, Vec<Label>) {
    let container = Box::new(Orientation::Horizontal, 6);
    container.add_css_class("sysinfo-group");
    container.set_valign(gtk4::Align::Center);

    // CPU pill
    let cpu_pill = make_pill("cpu", "󰍛 --%");
    cpu_pill.set_tooltip_text(Some("CPU Usage"));

    // Memory pill
    let mem_pill = make_pill("mem", "󰘚 --");
    mem_pill.set_tooltip_text(Some("Memory Usage"));

    // Unified Battery pill (icon + percentage)
    let bat_pill = make_pill("bat", "󰁹 --%");
    bat_pill.set_tooltip_text(Some("Battery Status"));

    container.append(&cpu_pill);
    container.append(&mem_pill);
    container.append(&bat_pill);

    let labels = vec![cpu_pill, mem_pill, bat_pill];
    (container, labels)
}

/// Build a single pill: a styled label with css classes.
fn make_pill(name: &str, initial: &str) -> Label {
    let label = Label::new(Some(initial));
    label.set_use_markup(true);
    label.add_css_class("sys-item");
    label.add_css_class(name);
    label
}

/// Refresh CPU and memory pills only.
pub fn update_system(labels: &[Label]) {
    let (cpu, mem_used, mem_total) = sys_stats();

    if let Some(cpu_label) = labels.first() {
        cpu_label.set_markup(&format!("<span color=\"#a4d1b4\">󰍛</span> {cpu}%"));
        cpu_label.set_tooltip_text(Some(&format!("CPU: {cpu}%")));

        if cpu >= 85 {
            cpu_label.add_css_class("critical");
            cpu_label.remove_css_class("warning");
        } else if cpu >= 60 {
            cpu_label.add_css_class("warning");
            cpu_label.remove_css_class("critical");
        } else {
            cpu_label.remove_css_class("warning");
            cpu_label.remove_css_class("critical");
        }
    }

    if let Some(mem_label) = labels.get(1) {
        let used_gb = mem_used as f64 / 1024.0;
        let total_gb = mem_total as f64 / 1024.0;
        mem_label.set_markup(&format!("<span color=\"#7ad9bc\">󰘚</span> {used_gb:.1}G/{total_gb:.1}G"));
        let pct = if mem_total > 0 {
            (mem_used as f64 / mem_total as f64) * 100.0
        } else {
            0.0
        };
        mem_label.set_tooltip_text(Some(&format!(
            "Memory: {used_gb:.2} GiB / {total_gb:.2} GiB ({pct:.0}%)"
        )));

        if pct >= 85.0 {
            mem_label.add_css_class("critical");
            mem_label.remove_css_class("warning");
        } else if pct >= 70.0 {
            mem_label.add_css_class("warning");
            mem_label.remove_css_class("critical");
        } else {
            mem_label.remove_css_class("warning");
            mem_label.remove_css_class("critical");
        }
    }
}

/// Refresh the battery pill only.
pub fn update_battery(labels: &[Label]) {
    let battery = battery_state();

    if let Some(bat_pill) = labels.get(2) {
        match battery {
            Some((percent, charging)) => {
                let icon = if charging {
                    "󰂄"
                } else {
                    battery_icon(percent)
                };
                let color = if charging {
                    "#a4d1b4"
                } else if percent <= 15 {
                    "#fa746f"
                } else if percent <= 30 {
                    "#cec06b"
                } else {
                    "#a3f1bd"
                };
                bat_pill.set_markup(&format!("<span color=\"{color}\">{icon}</span> {percent}%"));

                let status = if charging { "Charging" } else { "Discharging" };
                bat_pill.set_tooltip_text(Some(&format!("Battery: {percent}% ({status})")));

                bat_pill.remove_css_class("charging");
                bat_pill.remove_css_class("critical");
                bat_pill.remove_css_class("warning");
                bat_pill.remove_css_class("normal");

                if charging {
                    bat_pill.add_css_class("charging");
                } else if percent <= 15 {
                    bat_pill.add_css_class("critical");
                } else if percent <= 30 {
                    bat_pill.add_css_class("warning");
                } else {
                    bat_pill.add_css_class("normal");
                }
                bat_pill.set_visible(true);
            }
            None => {
                bat_pill.set_text("󰚥 AC");
                bat_pill.set_tooltip_text(Some("Power: Connected to AC"));
                bat_pill.add_css_class("normal");
            }
        }
    }
}

/// Convenience: refresh all system pills at once (used for the initial draw).
pub fn update(labels: &[Label]) {
    update_system(labels);
    update_battery(labels);
}

/// Pick a battery icon by charge level.
fn battery_icon(percent: u8) -> &'static str {
    match percent {
        0..=10 => "󰁺",
        11..=20 => "󰁻",
        21..=30 => "󰁼",
        31..=40 => "󰁽",
        41..=50 => "󰁾",
        51..=60 => "󰁿",
        61..=70 => "󰂀",
        71..=80 => "󰂁",
        81..=90 => "󰂂",
        _ => "󰁹",
    }
}

/// Reads CPU usage and memory figures. Returns `(cpu_pct, mem_used_mib, mem_total_mib)`.
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
        (cpu, mem_used, mem_total)
    })
}

/// Returns `(percentage, charging)` aggregated across all batteries, or `None`
/// when no battery is present.
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
