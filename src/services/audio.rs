//! Audio service, backed by the WirePlumber CLI (`wpctl`).
//!
//! The default output sink's volume and mute are queried with `wpctl
//! get-volume` and controlled with `wpctl set-volume`/`wpctl set-mute`. Change
//! events are observed with `pactl subscribe` on a background thread.

use std::process::Command;
use std::sync::mpsc::Sender;

use crate::events::Event;

/// The PipeWire object id of the default output sink.
const DEFAULT_SINK: &str = "@DEFAULT_AUDIO_SINK@";

/// The current audio state of the default output sink.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioState {
    /// Volume as a whole percentage, 0..=100.
    pub volume_percent: u8,
    /// Whether the sink is muted.
    pub muted: bool,
}

/// Run a command, returning its trimmed stdout (or `None` on failure).
fn run(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Query the current volume and mute state of the default output sink.
pub fn query() -> Option<AudioState> {
    let out = run("wpctl", &["get-volume", DEFAULT_SINK])?;

    // Output is e.g. "Volume: 0.45" or "Volume: 0.45 [MUTED]".
    let value_str = out.strip_prefix("Volume: ")?;
    let level = value_str
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f32>().ok())?
        .clamp(0.0, 1.0);

    let muted = value_str.contains("[MUTED]");

    Some(AudioState {
        volume_percent: (level * 100.0).round().clamp(0.0, 100.0) as u8,
        muted,
    })
}

/// Toggle mute on the default output sink.
pub fn toggle_mute() {
    let _ = run("wpctl", &["set-mute", DEFAULT_SINK, "toggle"]);
}

/// Get the icon glyph appropriate for the current state.
pub fn icon(state: &AudioState) -> &'static str {
    if state.muted {
        return " 󰝟";
    }
    match state.volume_percent {
        0..=30 => " 󰕿",
        31..=65 => " 󰖀",
        _ => " 󰕾",
    }
}

/// Set the output volume to an absolute percentage (0..=100).
pub fn set_volume(percent: u8) {
    let level = (percent as f32).clamp(0.0, 100.0) / 100.0;
    let arg = format!("{:.2}", level);
    let _ = run("wpctl", &["set-volume", DEFAULT_SINK, &arg]);
}

/// Change the output volume by `delta` percent. A positive value raises it and
/// a negative value lowers it.
pub fn change_volume(delta_percent: i8) {
    if delta_percent == 0 {
        return;
    }
    let sign = if delta_percent > 0 { "+" } else { "-" };
    let arg = format!("{}%{}", delta_percent.abs(), sign);
    let _ = run("wpctl", &["set-volume", DEFAULT_SINK, &arg]);
}

/// Background producer: tails `pactl subscribe` and emits
/// [`Event::AudioChanged`] whenever the default sink's volume or mute changes.
pub fn listen(tx: Sender<Event>) {
    use std::io::BufRead;

    let mut child = match Command::new("pactl")
        .arg("subscribe")
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return,
    };
    let stdout = child.stdout.take();
    match stdout {
        Some(pipe) => {
            for line in std::io::BufReader::new(pipe).lines() {
                let Ok(line) = line else { break };
                if line.starts_with("Event 'change' on sink ") {
                    let _ = tx.send(Event::AudioChanged);
                }
            }
        }
        None => {
            let _ = child.wait();
        }
    }
}
