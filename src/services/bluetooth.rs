//! Bluetooth service: queries adapter state and devices via `bluetoothctl`.

use std::process::Command;

/// State of the primary Bluetooth controller and active connections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluetoothState {
    /// Controller is powered on and at least one device is connected.
    Connected {
        name: String,
        battery: Option<u8>,
    },
    /// Controller is powered on, but no devices connected.
    Enabled {
        paired_count: usize,
    },
    /// Controller is powered off or disabled.
    Disabled,
    /// No Bluetooth adapter found on system.
    Unavailable,
}

/// A discovered or paired Bluetooth device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BluetoothDevice {
    pub mac: String,
    pub name: String,
    pub icon_type: String,
    pub is_connected: bool,
    pub is_paired: bool,
    pub battery: Option<u8>,
}

/// Query overall Bluetooth adapter state and connected device.
pub fn query() -> BluetoothState {
    if !is_enabled() {
        if !adapter_exists() {
            return BluetoothState::Unavailable;
        }
        return BluetoothState::Disabled;
    }

    let devices = get_devices();
    if let Some(connected) = devices.iter().find(|d| d.is_connected) {
        return BluetoothState::Connected {
            name: connected.name.clone(),
            battery: connected.battery,
        };
    }

    let paired_count = devices.iter().filter(|d| d.is_paired).count();
    BluetoothState::Enabled { paired_count }
}

/// Check if a Bluetooth adapter is present on the system.
pub fn adapter_exists() -> bool {
    let output = Command::new("bluetoothctl")
        .arg("show")
        .output()
        .ok();

    match output {
        Some(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.contains("Controller")
        }
        None => false,
    }
}

/// Check if Bluetooth is currently powered on.
pub fn is_enabled() -> bool {
    let output = Command::new("bluetoothctl")
        .arg("show")
        .output()
        .ok();

    if let Some(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        return stdout.lines().any(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("Powered:") && trimmed.contains("yes")
        });
    }

    false
}

/// Turn Bluetooth radio on or off via `bluetoothctl power on/off`.
pub fn set_enabled(enable: bool) -> Result<(), String> {
    let arg = if enable { "on" } else { "off" };
    let status = Command::new("bluetoothctl")
        .args(["power", arg])
        .status()
        .map_err(|e| format!("failed to execute bluetoothctl power: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("bluetoothctl power {arg} exited with {status}"))
    }
}

/// Retrieve all known (paired and discovered) devices with their detailed info.
pub fn get_devices() -> Vec<BluetoothDevice> {
    let output = match Command::new("bluetoothctl").arg("devices").output() {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => return Vec::new(),
    };

    let mut devices = Vec::new();
    for line in output.lines() {
        // Format: Device <MAC> <Name...>
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "Device" {
            let mac = parts[1].to_string();
            let name = parts[2..].join(" ");
            let info = query_device_info(&mac, &name);
            devices.push(info);
        }
    }

    // Sort: connected first, then paired, then alphabetically
    devices.sort_by(|a, b| {
        b.is_connected
            .cmp(&a.is_connected)
            .then_with(|| b.is_paired.cmp(&a.is_paired))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    devices
}

/// Query detailed information about a single device by MAC address.
fn query_device_info(mac: &str, fallback_name: &str) -> BluetoothDevice {
    let output = Command::new("bluetoothctl")
        .args(["info", mac])
        .output()
        .ok();

    let mut name = fallback_name.to_string();
    let mut icon_type = "generic".to_string();
    let mut is_connected = false;
    let mut is_paired = false;
    let mut battery = None;

    if let Some(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("Name:") {
                let n = rest.trim();
                if !n.is_empty() {
                    name = n.to_string();
                }
            } else if let Some(rest) = trimmed.strip_prefix("Alias:") {
                let a = rest.trim();
                if !a.is_empty() {
                    name = a.to_string();
                }
            } else if let Some(rest) = trimmed.strip_prefix("Icon:") {
                icon_type = rest.trim().to_string();
            } else if let Some(rest) = trimmed.strip_prefix("Connected:") {
                is_connected = rest.trim() == "yes";
            } else if let Some(rest) = trimmed.strip_prefix("Paired:") {
                is_paired = rest.trim() == "yes";
            } else if let Some(rest) = trimmed.strip_prefix("Battery Percentage:") {
                // e.g. "0x0055 (85)" or "85%"
                if let Some(pct_str) = rest.split('(').nth(1).and_then(|s| s.strip_suffix(')')) {
                    battery = pct_str.trim().parse::<u8>().ok();
                } else if let Ok(val) = rest.trim().trim_end_matches('%').parse::<u8>() {
                    battery = Some(val);
                }
            }
        }
    }

    BluetoothDevice {
        mac: mac.to_string(),
        name,
        icon_type,
        is_connected,
        is_paired,
        battery,
    }
}

/// Connect to a device via `bluetoothctl connect <mac>`.
pub fn connect_device(mac: &str) -> Result<(), String> {
    let output = Command::new("bluetoothctl")
        .args(["connect", mac])
        .output()
        .map_err(|e| format!("failed to execute connect: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() || stdout.contains("Connection successful") {
        Ok(())
    } else {
        let err_msg = if !stderr.trim().is_empty() {
            stderr.trim().to_string()
        } else {
            stdout.lines().last().unwrap_or("Connection failed").to_string()
        };
        Err(err_msg)
    }
}

/// Disconnect a device via `bluetoothctl disconnect <mac>`.
pub fn disconnect_device(mac: &str) -> Result<(), String> {
    let output = Command::new("bluetoothctl")
        .args(["disconnect", mac])
        .output()
        .map_err(|e| format!("failed to execute disconnect: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() || stdout.contains("Successful") {
        Ok(())
    } else {
        Err(stdout.lines().last().unwrap_or("Disconnect failed").to_string())
    }
}

/// Trigger device discovery via `bluetoothctl --timeout 5 scan on`.
pub fn scan_on() {
    let _ = Command::new("bluetoothctl")
        .args(["--timeout", "5", "scan", "on"])
        .spawn();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_functions_run_without_panic() {
        let _ = is_enabled();
        let _ = query();
        let _ = get_devices();
    }
}
