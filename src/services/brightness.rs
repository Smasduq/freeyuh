//! Screen brightness control service.
//!
//! Queries current display backlight levels via `/sys/class/backlight` and
//! controls brightness using `brightnessctl`.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Returns current screen brightness as a percentage (1..=100), or `None` if
/// no backlight device is detected.
pub fn query() -> Option<u8> {
    if let Ok(entries) = fs::read_dir("/sys/class/backlight") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let (Ok(cur), Ok(max)) = (
                read_number(&path.join("brightness")),
                read_number(&path.join("max_brightness")),
            ) {
                if max > 0 {
                    let pct = ((cur as f64 / max as f64) * 100.0).round() as u8;
                    return Some(pct.clamp(1, 100));
                }
            }
        }
    }

    // Fallback to brightnessctl query
    if let Ok(output) = Command::new("brightnessctl").arg("get").output() {
        if let Ok(cur_str) = String::from_utf8(output.stdout) {
            if let Ok(cur) = cur_str.trim().parse::<u64>() {
                if let Ok(max_output) = Command::new("brightnessctl").arg("max").output() {
                    if let Ok(max_str) = String::from_utf8(max_output.stdout) {
                        if let Ok(max) = max_str.trim().parse::<u64>() {
                            if max > 0 {
                                let pct = ((cur as f64 / max as f64) * 100.0).round() as u8;
                                return Some(pct.clamp(1, 100));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Set screen brightness to an absolute percentage (1..=100).
pub fn set_brightness(percent: u8) {
    let p = percent.clamp(1, 100);
    let _ = Command::new("brightnessctl")
        .args(["set", &format!("{p}%")])
        .spawn();
}

/// Change screen brightness by `delta` percentage points.
#[allow(dead_code)]
pub fn change_brightness(delta: i8) {
    if delta == 0 {
        return;
    }
    let arg = if delta > 0 {
        format!("{delta}%+")
    } else {
        format!("{}%-", delta.abs())
    };
    let _ = Command::new("brightnessctl")
        .args(["set", &arg])
        .spawn();
}

/// Pick an icon glyph matching the brightness level.
pub fn icon(percent: u8) -> &'static str {
    match percent {
        0..=25 => "󰃞",
        26..=60 => "󰃟",
        61..=85 => "󰃝",
        _ => "󰃠",
    }
}

fn read_number(path: &Path) -> Result<u64, ()> {
    let s = fs::read_to_string(path).map_err(|_| ())?;
    s.trim().parse::<u64>().map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon() {
        assert_eq!(icon(10), "󰃞");
        assert_eq!(icon(50), "󰃟");
        assert_eq!(icon(100), "󰃠");
    }
}
