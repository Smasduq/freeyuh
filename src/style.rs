//! Application styling: loads the bar's CSS theme via a `CssProvider`.

/// Register the bar stylesheet application-wide.
pub fn load() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(CSS);
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("no display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

const CSS: &str = r#"
/* --- Top Bar Root (Transparent Island Carrier) --- */
window.bar-window,
window {
    background: transparent;
}

.bar {
    background: transparent;
    border: none;
    box-shadow: none;
    padding: 0 4px;
    color: #c8ffd4;
    font-family: "JetBrainsMono Nerd Font", "Adwaita Sans", "Inter", monospace, system-ui;
    font-size: 13px;
    min-height: 36px;
}

/* --- Workspaces Island --- */
.workspaces {
    background: rgba(4, 8, 5, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 999px;
    padding: 4px 10px;
    margin: 3px 4px 3px 0;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6), 0 0 12px rgba(0, 230, 118, 0.04);
}

.workspaces button.ws {
    background: transparent;
    border: none;
    box-shadow: none;
    padding: 0;
    margin: 0 3px;
    min-width: 8px;
    min-height: 8px;
    border-radius: 999px;
    transition: all 200ms ease;
}

.workspaces button.ws.occupied {
    background: #1a3326;
    min-width: 8px;
    min-height: 8px;
}

.workspaces button.ws.occupied:hover {
    background: #00e676;
    box-shadow: 0 0 6px rgba(0, 230, 118, 0.5);
}

.workspaces button.ws.active {
    background: #00e676;
    min-width: 22px;
    min-height: 8px;
    border-radius: 999px;
    box-shadow: 0 0 10px rgba(0, 230, 118, 0.6), 0 0 24px rgba(0, 230, 118, 0.25);
}

.workspaces button.ws.free {
    background: transparent;
    border: 1.5px dashed rgba(0, 230, 118, 0.18);
    min-width: 8px;
    min-height: 8px;
}

.workspaces button.ws.free:hover {
    border-color: rgba(0, 230, 118, 0.5);
    background: rgba(0, 230, 118, 0.08);
}

/* --- Active Window Island --- */
.active-window {
    background: rgba(4, 8, 5, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.10);
    border-radius: 999px;
    padding: 4px 14px;
    margin: 3px 4px;
    color: #5a8a6e;
    font-size: 12px;
    font-weight: 500;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6);
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* --- Clock & Calendar Island --- */
.clock-pill {
    background: rgba(4, 8, 5, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 999px;
    padding: 4px 16px;
    margin: 3px 0;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6);
    transition: all 180ms ease;
}

.clock-pill:hover {
    background: rgba(6, 14, 9, 0.88);
    border-color: rgba(0, 230, 118, 0.32);
    box-shadow: 0 0 14px rgba(0, 230, 118, 0.12);
}

.clock-label {
    font-size: 13px;
    color: #00e676;
    font-family: "JetBrainsMono Nerd Font", monospace;
    font-weight: 600;
}

.calendar-window {
    background: transparent;
}

/* --- Modern Dashboard Panel --- */
.dash-window {
    background: transparent;
}

.dash-dropdown {
    background: rgba(4, 10, 6, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.18);
    border-radius: 22px;
    padding: 16px;
    box-shadow:
        0 28px 70px rgba(0, 0, 0, 0.85),
        0 0 40px rgba(0, 230, 118, 0.06),
        0 0 0 1px rgba(0, 230, 118, 0.04) inset;
    color: #c8ffd4;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-dropdown > * {
    margin-bottom: 12px;
}
.dash-dropdown > *:last-child {
    margin-bottom: 0;
}

.dash-card {
    background: rgba(0, 230, 118, 0.03);
    border: 1px solid rgba(0, 230, 118, 0.08);
    border-radius: 16px;
    padding: 12px 14px;
}

/* Hero: big time + date */
.dash-hero {
    background: linear-gradient(135deg, rgba(0, 230, 118, 0.10), rgba(0, 230, 118, 0.02));
    border: 1px solid rgba(0, 230, 118, 0.20);
}

.dash-hero-time {
    font-size: 40px;
    font-weight: 700;
    color: #00e676;
}

.dash-hero-date {
    font-size: 14px;
    font-weight: 600;
    color: #69ff97;
    margin-top: -2px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-hero-today {
    font-size: 11px;
    font-weight: 600;
    color: #2e4d3a;
    margin-top: 6px;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-battery {
    background: rgba(0, 230, 118, 0.08);
    border: 1px solid rgba(0, 230, 118, 0.18);
    border-radius: 999px;
    padding: 3px 10px;
    font-size: 12px;
    font-weight: 600;
    color: #00e676;
    margin-top: 6px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* Weather */
.dash-weather-icon {
    font-size: 38px;
    color: #00e676;
    min-width: 44px;
    text-shadow: 0 0 20px rgba(0, 230, 118, 0.4);
}

.dash-weather-temp {
    font-size: 30px;
    font-weight: 700;
    color: #e8fff2;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-weather-condition {
    font-size: 13px;
    font-weight: 600;
    color: #69ff97;
    padding-bottom: 4px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-weather-feels {
    font-size: 11px;
    color: #2e4d3a;
    margin-top: 2px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-weather-location {
    font-size: 11px;
    color: #3d6b52;
    margin-top: 6px;
    font-weight: 500;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-stat-pill {
    background: rgba(0, 230, 118, 0.04);
    border: 1px solid rgba(0, 230, 118, 0.10);
    border-radius: 10px;
    padding: 4px 10px;
    min-width: 92px;
}

.dash-stat-icon {
    font-size: 13px;
    color: #00e676;
}

.dash-stat-value {
    font-size: 12px;
    font-weight: 600;
    color: #c8ffd4;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* System section */
.dash-section-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 1.2px;
    color: #1e4030;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-stat-label {
    font-size: 12px;
    font-weight: 500;
    color: #4a6655;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-level {
    background: rgba(0, 230, 118, 0.06);
    border-radius: 999px;
    border: none;
    min-height: 6px;
}

.dash-level trough {
    background: rgba(0, 230, 118, 0.06);
    border-radius: 999px;
    min-height: 6px;
    border: none;
}

.dash-level highlight {
    background: #00e676;
    border-radius: 999px;
    min-height: 6px;
    box-shadow: 0 0 8px rgba(0, 230, 118, 0.5);
}

/* Calendar */
.dash-calendar {
    background: transparent;
    color: #c8ffd4;
    font-size: 12px;
    padding: 0;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.dash-calendar header {
    color: #00e676;
    font-weight: bold;
    background: transparent;
}

.dash-calendar button {
    border-radius: 8px;
    color: #4a6655;
    padding: 4px;
    background: transparent;
}

.dash-calendar button:hover {
    background: rgba(0, 230, 118, 0.12);
    color: #00e676;
}

.dash-calendar .day-number {
    border-radius: 50%;
}

.dash-calendar .day-name {
    color: #2e4d3a;
}

.dash-calendar .day-number.today {
    background: #00e676;
    color: #030805;
    font-weight: 700;
    box-shadow: 0 0 10px rgba(0, 230, 118, 0.5);
}

.dash-calendar .day-number:selected {
    background: rgba(0, 230, 118, 0.2);
    color: #00e676;
}

.dash-calendar .day-number:selected.today {
    background: #00e676;
    color: #030805;
}

/* --- System Resource Island --- */
.sysinfo-group {
    background: rgba(4, 8, 5, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.10);
    border-radius: 999px;
    padding: 4px 10px;
    margin: 3px 3px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6);
}

.sys-item {
    background: transparent;
    border: none;
    color: #4a6655;
    padding: 0px 4px;
    font-size: 12px;
    font-weight: 600;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.sys-item.cpu {
    color: #00e676;
}

.sys-item.cpu.warning {
    color: #ccff00;
}

.sys-item.cpu.critical {
    color: #ff4444;
}

.sys-item.mem {
    color: #69ff97;
}

.sys-item.mem.warning {
    color: #ccff00;
}

.sys-item.mem.critical {
    color: #ff4444;
}

/* --- Unified Quick Settings Island (Caelestia style) --- */
.quicksettings-pill {
    background: rgba(4, 8, 5, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.10);
    border-radius: 999px;
    padding: 4px 12px;
    margin: 3px 3px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6);
    transition: all 180ms ease;
}

.quicksettings-pill:hover {
    background: rgba(6, 14, 9, 0.88);
    border-color: rgba(0, 230, 118, 0.30);
    box-shadow: 0 0 12px rgba(0, 230, 118, 0.08);
}

.qs-pill-icon {
    font-size: 13px;
    margin: 0 2px;
}

.qs-pill-net {
    color: #00e676;
}

.qs-pill-bt {
    color: #69ff97;
}

.qs-pill-audio {
    color: #00e676;
    font-weight: 600;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-pill-bat {
    color: #69ff97;
    font-weight: 600;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* --- Notification Bell Island --- */
.bell {
    background: rgba(4, 8, 5, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 999px;
    padding: 4px 10px;
    margin: 3px 0 3px 3px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    color: #4a6655;
    font-size: 14px;
    transition: all 180ms ease;
}

.bell:hover {
    background: rgba(6, 14, 9, 0.88);
    border-color: rgba(0, 230, 118, 0.35);
    color: #00e676;
    box-shadow: 0 0 12px rgba(0, 230, 118, 0.18);
}

.bell.has-unread {
    color: #00e676;
    border-color: rgba(0, 230, 118, 0.3);
    box-shadow: 0 0 10px rgba(0, 230, 118, 0.2);
}

.bell-icon {
    font-size: 14px;
}

.notif-badge {
    background: #00e676;
    color: #030805;
    font-size: 10px;
    font-weight: 900;
    border-radius: 999px;
    padding: 0 5px;
    min-width: 14px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* --- Notification Center Dropdown --- */
.notif-center {
    background: transparent;
}

.notif-dropdown {
    background: rgba(4, 10, 6, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.18);
    border-radius: 20px;
    padding: 14px;
    min-width: 420px;
    min-height: 540px;
    box-shadow:
        0 24px 60px rgba(0, 0, 0, 0.85),
        0 0 40px rgba(0, 230, 118, 0.06),
        0 0 0 1px rgba(0, 230, 118, 0.04) inset;
    font-family: "JetBrainsMono Nerd Font", "Adwaita Sans", monospace;
}

.notif-header {
    padding: 5px 6px 11px 6px;
    border-bottom: 1px solid rgba(0, 230, 118, 0.12);
    margin-bottom: 8px;
}

.notif-header-title {
    font-weight: 700;
    font-size: 14px;
    color: #00e676;
    letter-spacing: 0.5px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.notif-clear-btn {
    background: rgba(0, 230, 118, 0.07);
    border: 1px solid rgba(0, 230, 118, 0.15);
    border-radius: 999px;
    padding: 4px 10px;
    color: #4a6655;
    font-size: 11px;
    font-family: "JetBrainsMono Nerd Font", monospace;
    transition: all 150ms ease;
}

.notif-clear-btn:hover {
    background: rgba(250, 82, 82, 0.14);
    border-color: rgba(250, 82, 82, 0.5);
    color: #ff6b6b;
}

.notif-empty {
    padding: 24px 16px;
    color: #1e3328;
}

.notif-empty-icon {
    font-size: 28px;
    margin-bottom: 4px;
    color: #1a2e22;
}

.notif-empty-text {
    font-size: 13px;
    font-weight: 500;
    color: #2e4d3a;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* --- Notification Keyframe Animations --- */
@keyframes toast-in {
    from {
        opacity: 0;
        transform: translateX(32px);
    }
    to {
        opacity: 1;
        transform: translateX(0);
    }
}

@keyframes toast-out {
    from {
        opacity: 1;
        transform: translateX(0);
    }
    to {
        opacity: 0;
        transform: translateX(24px);
    }
}

/* --- Notification Cards & Toasts --- */
.toast-window {
    background: transparent;
}

/* Base card (used inside the notification center history) */
.notif-toast {
    background: rgba(6, 16, 10, 0.70);
    border: 1px solid rgba(0, 230, 118, 0.14);
    border-left: 3px solid rgba(0, 230, 118, 0.6);
    border-radius: 12px;
    padding: 11px 13px;
    color: #c8ffd4;
    margin: 3px 0;
    transition: background 150ms ease, border-color 150ms ease;
    font-family: "JetBrainsMono Nerd Font", "Adwaita Sans", monospace;
}

.notif-toast:hover {
    background: rgba(8, 22, 14, 0.85);
    border-color: rgba(0, 230, 118, 0.28);
}

/* Live toast popup — Caelestia neon glass */
.toast-window .notif-toast {
    background: rgba(4, 10, 6, 0.82);
    border: 1px solid rgba(0, 230, 118, 0.22);
    border-left: 3px solid #00e676;
    border-radius: 14px;
    box-shadow:
        0 20px 50px rgba(0, 0, 0, 0.85),
        0 0 0 1px rgba(0, 230, 118, 0.06) inset,
        0 0 24px rgba(0, 230, 118, 0.10);
}

/* Critical urgency — red accent, still Caelestia dark */
.notif-toast.critical {
    border-left: 3px solid #ff4444;
    border-color: rgba(255, 68, 68, 0.28);
    box-shadow:
        0 20px 50px rgba(0, 0, 0, 0.9),
        0 0 24px rgba(255, 68, 68, 0.16);
}

/* Slide-in animation applied when toast is first shown */
.toast-window .notif-toast.toast-entering {
    animation: toast-in 280ms cubic-bezier(0.22, 1, 0.36, 1) both;
}

/* Slide-out animation applied just before the widget is removed */
.toast-window .notif-toast.toast-leaving {
    animation: toast-out 200ms ease-in both;
    pointer-events: none;
}

/* App name chip — neon green monospace tag */
.notif-app {
    font-size: 9px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #00e676;
    background: rgba(0, 230, 118, 0.10);
    border: 1px solid rgba(0, 230, 118, 0.22);
    padding: 1px 7px;
    border-radius: 4px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* Close button inside notification center cards */
.notif-card-close {
    background: transparent;
    border: none;
    color: #1e4030;
    font-size: 11px;
    padding: 1px 4px;
    border-radius: 4px;
    transition: all 120ms ease;
}

.notif-card-close:hover {
    background: rgba(255, 68, 68, 0.12);
    color: #ff6b6b;
}

/* Notification summary / title */
.notif-title {
    font-weight: 700;
    font-size: 13px;
    color: #e8fff2;
    margin-top: 4px;
    font-family: "JetBrainsMono Nerd Font", monospace;
    letter-spacing: 0.2px;
}

/* Notification body text */
.notif-body {
    font-size: 12px;
    color: #3d6b52;
    margin-top: 2px;
    line-height: 1.5;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.notif-center scrollbar {
    background: transparent;
}


/* =========================================================================
   UNIFIED QUICK SETTINGS / CONTROL CENTER PANEL (Caelestia M3)
   ========================================================================= */

.qs-window {
    background: transparent;
}

.qs-dropdown {
    background: rgba(4, 10, 6, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.18);
    border-radius: 24px;
    padding: 18px;
    min-width: 440px;
    min-height: 540px;
    box-shadow:
        0 24px 60px rgba(0, 0, 0, 0.85),
        0 0 40px rgba(0, 230, 118, 0.06),
        0 0 0 1px rgba(0, 230, 118, 0.04) inset;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-page {
    padding: 2px 1px;
}

/* --- Header Row --- */
.qs-header-row {
    padding: 2px 4px 12px 4px;
    border-bottom: 1px solid rgba(0, 230, 118, 0.12);
    margin-bottom: 8px;
}

.qs-header-title {
    font-size: 15px;
    font-weight: 700;
    color: #00e676;
    font-family: "JetBrainsMono Nerd Font", monospace;
    letter-spacing: 0.3px;
}

.qs-header-battery {
    background: rgba(0, 230, 118, 0.07);
    border: 1px solid rgba(0, 230, 118, 0.16);
    border-radius: 999px;
    padding: 3px 10px;
    font-size: 12px;
    font-weight: 600;
    color: #69ff97;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-header-battery.charging {
    background: rgba(0, 230, 118, 0.12);
    border-color: rgba(0, 230, 118, 0.3);
    color: #00e676;
    box-shadow: 0 0 8px rgba(0, 230, 118, 0.2);
}

/* --- Quick Toggle Tiles Grid --- */
.qs-tiles-container {
    margin-bottom: 2px;
}

.qs-tile {
    background: rgba(0, 230, 118, 0.05);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 16px;
    padding: 9px 10px;
    transition: all 150ms ease;
}

.qs-tile:hover {
    background: rgba(0, 230, 118, 0.10);
    border-color: rgba(0, 230, 118, 0.32);
}

.qs-tile.active {
    background: rgba(0, 230, 118, 0.18);
    border-color: rgba(0, 230, 118, 0.50);
    box-shadow: 0 0 14px rgba(0, 230, 118, 0.15);
}

.qs-tile-icon-btn {
    background: rgba(0, 230, 118, 0.06);
    border: none;
    border-radius: 999px;
    min-width: 38px;
    min-height: 38px;
    font-size: 16px;
    color: #3d6b52;
    padding: 0;
    transition: all 150ms ease;
}

.qs-tile-icon-btn:hover {
    background: rgba(0, 230, 118, 0.12);
    color: #69ff97;
}

.qs-tile.active .qs-tile-icon-btn {
    background: #00e676;
    color: #030805;
    box-shadow: 0 0 12px rgba(0, 230, 118, 0.5);
}

.qs-tile-text-btn {
    background: transparent;
    border: none;
    padding: 0 4px;
}

.qs-tile-title {
    font-size: 12px;
    font-weight: 600;
    color: #c8ffd4;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-tile-sub {
    font-size: 10px;
    font-weight: 500;
    color: #2e4d3a;
}

.qs-tile.active .qs-tile-sub {
    color: #00e676;
}

.qs-tile-arrow-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 28px;
    min-height: 28px;
    font-size: 15px;
    color: #2e4d3a;
    padding: 0;
    transition: all 150ms ease;
}

.qs-tile-arrow-btn:hover {
    background: rgba(0, 230, 118, 0.10);
    color: #69ff97;
}

/* --- Volume Slider Card --- */
.qs-slider-card {
    background: rgba(0, 230, 118, 0.05);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 16px;
    padding: 9px 12px;
    margin-top: 6px;
}

.qs-slider-mute-btn {
    background: rgba(0, 230, 118, 0.08);
    border: none;
    border-radius: 999px;
    min-width: 38px;
    min-height: 38px;
    font-size: 16px;
    color: #00e676;
    padding: 0;
    transition: all 150ms ease;
}

.qs-slider-mute-btn.muted {
    color: #ff4444;
    background: rgba(255, 68, 68, 0.10);
}

.qs-slider-mute-btn:hover {
    background: rgba(0, 230, 118, 0.16);
}

.qs-volume-scale trough {
    background: rgba(0, 230, 118, 0.08);
    border-radius: 999px;
    min-height: 6px;
    border: none;
}

.qs-volume-scale highlight {
    background: #00e676;
    border-radius: 999px;
    min-height: 6px;
    box-shadow: 0 0 8px rgba(0, 230, 118, 0.4);
}

.qs-volume-scale slider {
    background: #c8ffd4;
    border-radius: 999px;
    min-width: 14px;
    min-height: 14px;
    margin: -4px 0;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.5), 0 0 6px rgba(0, 230, 118, 0.3);
    border: none;
}

.qs-slider-pct {
    font-size: 11px;
    font-weight: 600;
    color: #2e4d3a;
    min-width: 36px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* --- Detail Navigation Bar --- */
.qs-nav-bar {
    padding-bottom: 8px;
    border-bottom: 1px solid rgba(0, 230, 118, 0.10);
    margin-bottom: 6px;
}

.qs-back-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 32px;
    min-height: 32px;

.qs-circular-controls {
    margin: 2px 0 4px;
}

.qs-circular-card {
    background: rgba(0, 230, 118, 0.04);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 18px;
    padding: 10px 8px;
}

.qs-circular-control {
    background: rgba(0, 230, 118, 0.10);
    border: 1px solid rgba(0, 230, 118, 0.18);
    border-radius: 999px;
    min-width: 92px;
    min-height: 92px;
    padding: 0;
    color: #00e676;
    transition: all 150ms ease;
}

.qs-circular-control:hover {
    background: rgba(0, 230, 118, 0.18);
    border-color: rgba(0, 230, 118, 0.45);
    box-shadow: 0 0 18px rgba(0, 230, 118, 0.16);
}

.qs-circular-icon {
    font-size: 25px;
    color: #00e676;
}

.qs-circular-value {
    font-size: 11px;
    font-weight: 700;
    color: #c8ffd4;
}

.qs-header-actions {
    margin-left: 4px;
}

.qs-header-action {
    background: rgba(0, 230, 118, 0.08);
    border: 1px solid rgba(0, 230, 118, 0.14);
    border-radius: 999px;
    min-width: 30px;
    min-height: 30px;
    padding: 0;
    color: #7ad9bc;
    font-size: 15px;
    transition: all 150ms ease;
}

.qs-header-action:hover {
    background: rgba(0, 230, 118, 0.18);
    color: #00e676;
}
    font-size: 18px;
    color: #3d6b52;
    padding: 0;
    transition: all 150ms ease;
}

.qs-back-btn:hover {
    background: rgba(0, 230, 118, 0.10);
    color: #00e676;
}

.qs-nav-title {
    font-size: 14px;
    font-weight: 700;
    color: #c8ffd4;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-rescan-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 32px;
    min-height: 32px;
    font-size: 15px;
    color: #2e4d3a;
    padding: 0;
    transition: all 150ms ease;
}

.qs-rescan-btn:hover {
    background: rgba(0, 230, 118, 0.12);
    color: #00e676;
}

/* Switch Component */
switch.qs-switch {
    background: rgba(0, 230, 118, 0.10);
    border-radius: 999px;
    border: none;
    padding: 0;
    outline: none;
    box-shadow: none;
    transition: all 200ms ease;
}

switch.qs-switch:checked {
    background: #00e676;
    box-shadow: 0 0 10px rgba(0, 230, 118, 0.4);
}

switch.qs-switch slider {
    background: #3d6b52;
    border-radius: 999px;
    margin: 2px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
    transition: all 200ms ease;
}

switch.qs-switch:checked slider {
    background: #030805;
}

/* Feedback banner */
.qs-status-banner {
    background: rgba(0, 230, 118, 0.08);
    border: 1px solid rgba(0, 230, 118, 0.22);
    border-radius: 999px;
    padding: 4px 14px;
    color: #00e676;
    font-size: 11px;
    font-weight: 600;
    margin: 4px 0;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* Detail Scrolled List Items */
.qs-list-box {
    margin-top: 2px;
}

.qs-item {
    background: rgba(0, 230, 118, 0.04);
    border: 1px solid rgba(0, 230, 118, 0.08);
    border-radius: 12px;
    padding: 8px 10px;
    margin: 2px 0;
    transition: all 150ms ease;
}

.qs-item:hover {
    background: rgba(0, 230, 118, 0.08);
    border-color: rgba(0, 230, 118, 0.22);
}

.qs-item.connected .qs-item-name {
    color: #00e676;
    font-weight: 700;
}

.qs-icon-chip {
    background: transparent;
    border: none;
    min-width: 24px;
    min-height: 24px;
}

.qs-item-icon {
    font-size: 16px;
    color: #2e4d3a;
}

.qs-item-name {
    color: #c8ffd4;
    font-weight: 500;
    font-size: 13px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-connected-icon {
    color: #00e676;
    font-size: 14px;
    font-weight: bold;
}

.qs-saved-icon {
    color: #1e4030;
    font-size: 12px;
}

.qs-paired-icon {
    color: #1e4030;
    font-size: 12px;
}

.qs-lock-icon {
    color: #1e4030;
    font-size: 12px;
}

.qs-item-signal {
    color: #2e4d3a;
    font-size: 11px;
    font-weight: 500;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-item-battery {
    color: #2e4d3a;
    font-size: 11px;
    font-weight: 500;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-connect-btn {
    background: transparent;
    border: none;
    border-radius: 8px;
    color: #2e4d3a;
    font-size: 16px;
    padding: 4px 8px;
    transition: all 150ms ease;
}

.qs-connect-btn:hover {
    background: rgba(0, 230, 118, 0.12);
    color: #00e676;
}

.qs-disconnect-btn {
    background: transparent;
    border: none;
    border-radius: 8px;
    color: #5a2020;
    font-size: 16px;
    padding: 4px 8px;
    transition: all 150ms ease;
}

.qs-disconnect-btn:hover {
    background: rgba(255, 68, 68, 0.14);
    color: #ff4444;
}

/* Auth Overlay Page */
.qs-auth-page {
    padding: 6px 12px 14px 12px;
}

.qs-auth-icon {
    font-size: 38px;
    color: #00e676;
    margin-bottom: 4px;
    text-shadow: 0 0 20px rgba(0, 230, 118, 0.5);
}

.qs-auth-ssid {
    font-size: 17px;
    font-weight: 700;
    color: #e8fff2;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-auth-sub {
    font-size: 12px;
    color: #3d6b52;
    font-weight: 500;
}

.qs-auth-input-card {
    background: rgba(0, 230, 118, 0.03);
    border: 1px solid rgba(0, 230, 118, 0.08);
    border-radius: 16px;
    padding: 12px 14px;
    margin: 6px 0;
}

.qs-auth-input-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.8px;
    color: #1e4030;
    margin-bottom: 4px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-auth-entry {
    background: rgba(0, 0, 0, 0.6);
    border: 1px solid rgba(0, 230, 118, 0.14);
    border-radius: 10px;
    color: #c8ffd4;
    font-size: 13px;
    padding: 8px 12px;
    transition: all 150ms ease;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-auth-entry:focus {
    border-color: #00e676;
    background: rgba(0, 0, 0, 0.75);
    box-shadow: 0 0 0 1px rgba(0, 230, 118, 0.4), 0 0 12px rgba(0, 230, 118, 0.12);
}

.qs-auth-status {
    font-size: 12px;
    font-weight: 600;
    padding: 4px 8px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-auth-status.error {
    color: #ff4444;
}

.qs-auth-status.connecting {
    color: #00e676;
}

.qs-auth-actions {
    margin-top: 10px;
}

.qs-auth-cancel-btn {
    background: rgba(0, 230, 118, 0.05);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 12px;
    color: #4a6655;
    font-size: 13px;
    font-weight: 600;
    padding: 10px 16px;
    transition: all 150ms ease;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-auth-cancel-btn:hover {
    background: rgba(0, 230, 118, 0.10);
    color: #c8ffd4;
}

.qs-auth-connect-btn {
    background: #00e676;
    border: none;
    border-radius: 12px;
    color: #030805;
    font-size: 13px;
    font-weight: 700;
    padding: 10px 16px;
    transition: all 150ms ease;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-auth-connect-btn:hover {
    background: #69ff97;
    box-shadow: 0 0 18px rgba(0, 230, 118, 0.5);
}

/* Empty / Scanning states */
.qs-empty {
    padding: 40px 16px;
    color: #1e4030;
}

.qs-empty-icon {
    font-size: 34px;
    margin-bottom: 8px;
    color: #152b20;
}

.qs-empty-text {
    font-size: 13px;
    font-weight: 600;
    color: #3d6b52;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-empty-sub {
    font-size: 11px;
    color: #1e4030;
    margin-top: 2px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.qs-scanning {
    color: #2e4d3a;
    font-size: 13px;
    padding: 20px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* Thin Custom Scrollbar */
.qs-scrolled-window scrollbar {
    background: transparent;
    border: none;
    min-width: 4px;
}

.qs-scrolled-window scrollbar slider {
    background: rgba(0, 230, 118, 0.12);
    border-radius: 999px;
    min-width: 4px;
    border: none;
}

.qs-scrolled-window scrollbar slider:hover {
    background: rgba(0, 230, 118, 0.30);
}

/* =========================================================================
   SPOTLIGHT APP LAUNCHER (Caelestia style)
   ========================================================================= */

.launcher-window {
    background: transparent;
}

.launcher-card {
    background: rgba(4, 10, 6, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.20);
    border-radius: 24px;
    padding: 14px 16px;
    min-width: 560px;
    box-shadow:
        0 28px 70px rgba(0, 0, 0, 0.85),
        0 0 40px rgba(0, 230, 118, 0.06),
        0 0 0 1px rgba(0, 230, 118, 0.04) inset;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-search-box {
    background: rgba(0, 230, 118, 0.05);
    border: 1px solid rgba(0, 230, 118, 0.12);
    border-radius: 16px;
    padding: 6px 14px;
    margin-bottom: 8px;
    transition: all 150ms ease;
}

.launcher-search-box:focus-within {
    background: rgba(0, 230, 118, 0.08);
    border-color: rgba(0, 230, 118, 0.45);
    box-shadow: 0 0 0 1px rgba(0, 230, 118, 0.25), 0 0 16px rgba(0, 230, 118, 0.10);
}

.launcher-search-icon {
    font-size: 18px;
    color: #00e676;
    margin-right: 4px;
}

.launcher-search-entry {
    background: transparent;
    border: none;
    color: #e8fff2;
    font-size: 15px;
    font-weight: 500;
    outline: none;
    box-shadow: none;
    padding: 4px 0;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-search-entry:focus {
    background: transparent;
    border: none;
    outline: none;
    box-shadow: none;
}

.launcher-chip {
    background: rgba(0, 230, 118, 0.08);
    border: 1px solid rgba(0, 230, 118, 0.14);
    border-radius: 6px;
    padding: 2px 6px;
    font-size: 10px;
    font-weight: 700;
    color: #2e4d3a;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-results-box {
    margin-top: 4px;
}

.launcher-item {
    background: transparent;
    border: 1px solid transparent;
    border-radius: 14px;
    padding: 8px 12px;
    margin: 2px 0;
    transition: all 120ms ease;
}

.launcher-item:hover {
    background: rgba(0, 230, 118, 0.04);
}

.launcher-item.selected {
    background: rgba(0, 230, 118, 0.10);
    border-color: rgba(0, 230, 118, 0.28);
    box-shadow: 0 0 12px rgba(0, 230, 118, 0.08);
}

.launcher-item-icon {
    margin-right: 4px;
}

.launcher-item-title {
    font-size: 13.5px;
    font-weight: 600;
    color: #c8ffd4;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-item.selected .launcher-item-title {
    color: #00e676;
    font-weight: 700;
}

.launcher-item-desc {
    font-size: 11px;
    color: #2e4d3a;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-item.selected .launcher-item-desc {
    color: #69ff97;
}

.launcher-enter-hint {
    color: #1e4030;
    font-size: 14px;
    margin-left: 8px;
    opacity: 0;
    transition: opacity 120ms ease;
}

.launcher-item.selected .launcher-enter-hint {
    color: #00e676;
    opacity: 1;
}

.launcher-footer {
    border-top: 1px solid rgba(0, 230, 118, 0.10);
    padding-top: 8px;
    margin-top: 6px;
}

.launcher-hint {
    font-size: 11px;
    color: #1e4030;
    font-weight: 500;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-count {
    font-size: 11px;
    color: #2e4d3a;
    font-weight: 600;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-empty {
    padding: 40px 16px;
    color: #1e4030;
}

.launcher-empty-icon {
    font-size: 34px;
    margin-bottom: 8px;
    color: #152b20;
}

.launcher-empty-text {
    font-size: 13px;
    font-weight: 600;
    color: #3d6b52;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.launcher-scrolled-window scrollbar {
    background: transparent;
    border: none;
    min-width: 4px;
}

.launcher-scrolled-window scrollbar slider {
    background: rgba(0, 230, 118, 0.12);
    border-radius: 999px;
    min-width: 4px;
    border: none;
}

.launcher-scrolled-window scrollbar slider:hover {
    background: rgba(0, 230, 118, 0.30);
}

/* =========================================================================
   POWER & SESSION MENU HUD
   ========================================================================= */

.powermenu-backdrop {
    background: rgba(0, 5, 2, 0.65);
}

.powermenu-card {
    background: rgba(4, 10, 6, 0.78);
    border: 1px solid rgba(0, 230, 118, 0.20);
    border-radius: 28px;
    padding: 36px 44px;
    box-shadow:
        0 36px 90px rgba(0, 0, 0, 0.85),
        0 0 50px rgba(0, 230, 118, 0.05),
        0 0 0 1px rgba(0, 230, 118, 0.04) inset;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.powermenu-title {
    font-size: 20px;
    font-weight: 700;
    color: #00e676;
    font-family: "JetBrainsMono Nerd Font", monospace;
    letter-spacing: 0.3px;
}

.powermenu-subtitle {
    font-size: 12px;
    font-weight: 500;
    color: #2e4d3a;
    margin-top: 2px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.powermenu-actions-row {
    margin-top: 10px;
}

.powermenu-tile {
    background: rgba(0, 230, 118, 0.05);
    border: 1px solid rgba(0, 230, 118, 0.10);
    border-radius: 20px;
    padding: 24px 18px;
    min-width: 106px;
    min-height: 124px;
    transition: all 180ms ease;
}

.powermenu-tile-icon {
    font-size: 34px;
    margin-bottom: 4px;
    color: #2e4d3a;
    transition: all 180ms ease;
}

.powermenu-tile-name {
    font-size: 13px;
    font-weight: 600;
    color: #4a6655;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

.powermenu-tile-key {
    font-size: 10px;
    font-weight: 700;
    color: #1e4030;
    background: rgba(0, 230, 118, 0.06);
    border-radius: 6px;
    padding: 2px 6px;
    margin-top: 4px;
    font-family: "JetBrainsMono Nerd Font", monospace;
}

/* Specific Action Color Accents */
.action-poweroff.selected,
.action-poweroff:hover {
    background: rgba(255, 68, 68, 0.14);
    border-color: rgba(255, 68, 68, 0.55);
    box-shadow: 0 0 28px rgba(255, 68, 68, 0.30);
}
.action-poweroff.selected .powermenu-tile-icon,
.action-poweroff:hover .powermenu-tile-icon {
    color: #ff4444;
}

.action-reboot.selected,
.action-reboot:hover {
    background: rgba(255, 160, 60, 0.14);
    border-color: rgba(255, 160, 60, 0.55);
    box-shadow: 0 0 28px rgba(255, 160, 60, 0.30);
}
.action-reboot.selected .powermenu-tile-icon,
.action-reboot:hover .powermenu-tile-icon {
    color: #ffa03c;
}

.action-suspend.selected,
.action-suspend:hover {
    background: rgba(0, 195, 255, 0.12);
    border-color: rgba(0, 195, 255, 0.50);
    box-shadow: 0 0 28px rgba(0, 195, 255, 0.25);
}
.action-suspend.selected .powermenu-tile-icon,
.action-suspend:hover .powermenu-tile-icon {
    color: #00c3ff;
}

.action-lock.selected,
.action-lock:hover {
    background: rgba(0, 230, 118, 0.12);
    border-color: rgba(0, 230, 118, 0.50);
    box-shadow: 0 0 28px rgba(0, 230, 118, 0.25);
}
.action-lock.selected .powermenu-tile-icon,
.action-lock:hover .powermenu-tile-icon {
    color: #00e676;
}

.action-logout.selected,
.action-logout:hover {
    background: rgba(180, 100, 255, 0.12);
    border-color: rgba(180, 100, 255, 0.50);
    box-shadow: 0 0 28px rgba(180, 100, 255, 0.25);
}
.action-logout.selected .powermenu-tile-icon,
.action-logout:hover .powermenu-tile-icon {
    color: #b464ff;
}
"#;
