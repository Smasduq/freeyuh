# iFreeYuh

A minimal, fast Wayland status bar for [Hyprland](https://hyprland.org/), written in Rust with GTK4 and `gtk4-layer-shell`.

![iFreeYuh bar screenshot](assets/screenshot.png)

---

## Features

### Left section
- **Workspace indicators** — dynamic pill/circle buttons for every open Hyprland workspace; always shows the active workspace even if empty; click to switch
- **Active window title** — live title of the focused window, ellipsized gracefully at 55 chars

### Center section
- **Clock & Calendar** — date/time pill; hover to open a GTK calendar dropdown with a 250ms leave-delay so you can move the cursor into it comfortably; click also toggles it

### Right section
- **System stats** — CPU %, memory used/total, and battery percentage with Nerd Font glyphs (󰍛 / 󰘚 / 󰁹), tooltips, and responsive styling (warning/critical states)
- **Wi-Fi panel** — hover the network icon to open an interactive connection panel:
  - Scans for nearby networks via `nmcli`
  - Connected / Saved badges with color-coded signal strength icons
  - One-click connect for saved or open networks; inline password entry for secured ones
  - Wi-Fi radio on/off toggle switch and manual rescan button
- **Audio** — volume % pill with muted state indicator; polls PipeWire/PulseAudio via `wpctl`
- **Notification bell** — hover or click to open the notification center (400 × 560 px scrollable history); dismiss individual notifications or clear all

---

## Dependencies

| Dependency | Purpose |
|---|---|
| `gtk4` | UI toolkit |
| `gtk4-layer-shell` | Wayland layer-shell protocol (places bar at top of screen) |
| `glib` | GLib bindings (timers, main loop) |
| `zbus` | D-Bus for notification daemon |
| `sysinfo` | CPU / memory / battery metrics |
| `serde` / `serde_json` | Hyprland IPC JSON parsing |
| `chrono` | Date/time formatting |
| `nmcli` *(runtime)* | Wi-Fi scanning and connection management |
| `wpctl` *(runtime)* | Audio volume / mute queries |
| Nerd Font *(runtime)* | Icon glyphs in the bar |

---

## Building

```bash
# Prerequisites: Rust (stable), GTK4 dev headers, gtk4-layer-shell dev headers
# On Arch:
sudo pacman -S gtk4 gtk4-layer-shell

cargo build --release
```

---

## Running

```bash
./target/release/ifreeyuh
```

iFreeYuh registers itself as a Wayland layer-shell surface on startup. It expects Hyprland to be running (IPC socket at `$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock`).

To autostart with Hyprland, add to `hyprland.conf`:

```ini
exec-once = /path/to/ifreeyuh
```

---

## Architecture

```
src/
├── main.rs                  # GTK Application entry point
├── app.rs                   # Bar window, layout, event reactor
├── events.rs                # Event enum + background producer spawners
├── style.rs                 # Embedded CSS stylesheet
├── compositor/
│   └── hyprland.rs          # Hyprland IPC listener & helpers
├── services/
│   ├── audio.rs             # wpctl volume/mute queries
│   ├── network.rs           # nmcli Wi-Fi scan / connect / disconnect
│   └── notifications.rs     # D-Bus org.freedesktop.Notifications daemon
└── widgets/
    ├── workspace.rs         # Workspace indicator buttons
    ├── window.rs            # Active window title label
    ├── clock.rs             # Clock pill + calendar dropdown
    ├── sysinfo.rs           # CPU / memory / battery pills
    ├── network.rs           # Wi-Fi status pill + connection panel
    ├── audio.rs             # Volume pill
    └── notifications.rs     # Bell button + notification center
```

All event producers (Hyprland IPC, clock ticks, system ticks, D-Bus) push into a single `mpsc::channel<Event>`. The main thread drains it with a `glib::timeout_add_local` poll loop and dispatches each event to the relevant widget update — no GTK widgets are ever touched from worker threads.

---

## Theme

Dark forest / sage Material 3 palette:

| Token | Value |
|---|---|
| Background | `#0b0f0c` |
| Surface | `#141b17` |
| Primary accent (mint) | `#a4d1b4` |
| Teal accent | `#7ad9bc` |
| Text | `#dee8df` |
| Muted | `#6e7870` / `#a4aea5` |

---

## License

Apache-2.0 — see [LICENSE](LICENSE).
