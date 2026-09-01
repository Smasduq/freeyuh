//! Network service: queries the active connection state and controls Wi-Fi networks.

use std::collections::HashSet;
use std::process::Command;

/// The current state of the active network connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkState {
    Wifi { ssid: String },
    Ethernet { name: String },
    Disconnected,
}

/// A scanned Wi-Fi access point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub security: String,
    pub is_connected: bool,
    pub is_saved: bool,
}

/// Query the current network status.
pub fn query() -> NetworkState {
    let out = Command::new("nmcli")
        .args(["-t", "-f", "TYPE,STATE,CONNECTION", "dev"])
        .output()
        .ok();

    if let Some(out) = out {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 && parts[1] == "connected" {
                    let conn_type = parts[0];
                    let conn_name = parts[2].trim();
                    if conn_type == "wifi" {
                        return NetworkState::Wifi {
                            ssid: conn_name.to_string(),
                        };
                    } else if conn_type == "ethernet" {
                        return NetworkState::Ethernet {
                            name: conn_name.to_string(),
                        };
                    }
                }
            }
        }
    }

    NetworkState::Disconnected
}

/// Check whether Wi-Fi radio is enabled.
pub fn wifi_enabled() -> bool {
    let out = Command::new("nmcli")
        .args(["radio", "wifi"])
        .output()
        .ok();
    if let Some(out) = out {
        String::from_utf8_lossy(&out.stdout).trim() == "enabled"
    } else {
        false
    }
}

/// Enable or disable Wi-Fi radio.
pub fn set_wifi_enabled(enable: bool) {
    let state = if enable { "on" } else { "off" };
    let _ = Command::new("nmcli").args(["radio", "wifi", state]).output();
}

/// Trigger an asynchronous Wi-Fi rescan.
pub fn rescan_wifi() {
    std::thread::spawn(|| {
        let _ = Command::new("nmcli")
            .args(["dev", "wifi", "rescan"])
            .output();
    });
}

/// Fetch list of saved Wi-Fi connections.
fn saved_connections() -> HashSet<String> {
    let mut set = HashSet::new();
    let out = Command::new("nmcli")
        .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
        .output()
        .ok();

    if let Some(out) = out {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if let Some((name, conn_type)) = line.split_once(':') {
                if conn_type.contains("wireless") {
                    set.insert(name.trim().to_string());
                }
            }
        }
    }
    set
}

/// Scan nearby Wi-Fi networks and return a sorted list.
pub fn scan_wifi() -> Vec<WifiNetwork> {
    let saved = saved_connections();

    let out = match Command::new("nmcli")
        .args(["-t", "-f", "IN-USE,SSID,SIGNAL,SECURITY", "dev", "wifi", "list"])
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => return Vec::new(),
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let mut map: std::collections::HashMap<String, WifiNetwork> = std::collections::HashMap::new();

    for line in text.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            continue;
        }

        let in_use = parts[0].trim() == "*";
        let ssid = parts[1].trim().to_string();
        if ssid.is_empty() {
            continue; // Ignore hidden networks
        }

        let signal = parts[2].trim().parse::<u8>().unwrap_or(0);
        let security = parts[3..].join(":").trim().to_string();
        let is_saved = saved.contains(&ssid);

        let entry = WifiNetwork {
            ssid: ssid.clone(),
            signal,
            security,
            is_connected: in_use,
            is_saved,
        };

        match map.get(&ssid) {
            Some(existing) => {
                // Prefer connected, then higher signal
                if entry.is_connected || (!existing.is_connected && entry.signal > existing.signal) {
                    map.insert(ssid, entry);
                }
            }
            None => {
                map.insert(ssid, entry);
            }
        }
    }

    let mut list: Vec<WifiNetwork> = map.into_values().collect();
    // Sort: connected first, then by signal descending, then by name.
    list.sort_by(|a, b| {
        b.is_connected
            .cmp(&a.is_connected)
            .then_with(|| b.signal.cmp(&a.signal))
            .then_with(|| a.ssid.cmp(&b.ssid))
    });

    list
}

/// Connect to a Wi-Fi network by SSID, optionally with a password.
pub fn connect_wifi(ssid: &str, password: Option<&str>) -> Result<(), String> {
    let mut cmd = Command::new("nmcli");
    cmd.args(["dev", "wifi", "connect", ssid]);
    if let Some(pass) = password {
        if !pass.is_empty() {
            cmd.args(["password", pass]);
        }
    }

    let out = cmd.output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if err.is_empty() {
            let stdout_err = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Err(stdout_err)
        } else {
            Err(err)
        }
    }
}

/// Disconnect from a Wi-Fi network.
pub fn disconnect_wifi(ssid: &str) -> Result<(), String> {
    let out = Command::new("nmcli")
        .args(["con", "down", "id", ssid])
        .output()
        .map_err(|e| e.to_string())?;

    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_wifi_runs_without_panic() {
        let networks = scan_wifi();
        // Even if empty or not, it should parse cleanly without panic
        for net in &networks {
            assert!(!net.ssid.is_empty());
        }
    }

    #[test]
    fn test_wifi_enabled_runs_without_panic() {
        let _ = wifi_enabled();
    }
}
