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
    color: #dee8df;
    font-family: "Adwaita Sans", "Inter", "JetBrainsMono Nerd Font", system-ui, sans-serif;
    font-size: 13px;
    min-height: 36px;
}

/* --- Workspaces Island --- */
.workspaces {
    background: rgba(14, 19, 16, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 4px 10px;
    margin: 3px 4px 3px 0;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
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
    background: #6e7870;
    min-width: 8px;
    min-height: 8px;
}

.workspaces button.ws.occupied:hover {
    background: #dee8df;
}

.workspaces button.ws.active {
    background: #a4d1b4;
    min-width: 22px;
    min-height: 8px;
    border-radius: 999px;
    box-shadow: 0 0 8px rgba(164, 209, 180, 0.45);
}

.workspaces button.ws.free {
    background: transparent;
    border: 1.5px dashed #414a43;
    min-width: 8px;
    min-height: 8px;
}

.workspaces button.ws.free:hover {
    border-color: #a4d1b4;
    background: rgba(164, 209, 180, 0.15);
}

/* --- Active Window Island --- */
.active-window {
    background: rgba(14, 19, 16, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 4px 14px;
    margin: 3px 4px;
    color: #dee8df;
    font-size: 12px;
    font-weight: 500;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
}

/* --- Clock & Calendar Island --- */
.clock-pill {
    background: rgba(14, 19, 16, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 4px 16px;
    margin: 3px 0;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
    transition: all 180ms ease;
}

.clock-pill:hover {
    background: rgba(22, 30, 25, 0.95);
    border-color: rgba(164, 209, 180, 0.3);
}

.clock-label {
    font-size: 13px;
}

.calendar-window {
    background: transparent;
}

/* --- Modern Dashboard Panel --- */
.dash-window {
    background: transparent;
}

.dash-dropdown {
    background: rgba(12, 17, 14, 0.97);
    border: 1px solid rgba(164, 209, 180, 0.18);
    border-radius: 22px;
    padding: 16px;
    box-shadow: 0 22px 60px rgba(0, 0, 0, 0.8), 0 0 0 1px rgba(255, 255, 255, 0.04) inset;
    color: #dee8df;
}

.dash-dropdown > * {
    margin-bottom: 12px;
}
.dash-dropdown > *:last-child {
    margin-bottom: 0;
}

.dash-card {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 16px;
    padding: 12px 14px;
}

/* Hero: big time + date */
.dash-hero {
    background: linear-gradient(135deg, rgba(164, 209, 180, 0.14), rgba(164, 209, 180, 0.04));
    border: 1px solid rgba(164, 209, 180, 0.22);
}

.dash-hero-time {
    font-size: 40px;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: -1px;
}

.dash-hero-date {
    font-size: 14px;
    font-weight: 600;
    color: #a4d1b4;
    margin-top: -2px;
}

.dash-hero-today {
    font-size: 11px;
    font-weight: 600;
    color: #8d9990;
    margin-top: 6px;
    text-transform: uppercase;
    letter-spacing: 0.6px;
}

.dash-battery {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 3px 10px;
    font-size: 12px;
    font-weight: 600;
    color: #dee8df;
    margin-top: 6px;
}

/* Weather */
.dash-weather-icon {
    font-size: 38px;
    color: #a4d1b4;
    min-width: 44px;
}

.dash-weather-temp {
    font-size: 30px;
    font-weight: 700;
    color: #ffffff;
}

.dash-weather-condition {
    font-size: 13px;
    font-weight: 600;
    color: #a4d1b4;
    padding-bottom: 4px;
}

.dash-weather-feels {
    font-size: 11px;
    color: #8d9990;
    margin-top: 2px;
}

.dash-weather-location {
    font-size: 11px;
    color: #8d9990;
    margin-top: 6px;
    font-weight: 500;
}

.dash-stat-pill {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 10px;
    padding: 4px 10px;
    min-width: 92px;
}

.dash-stat-icon {
    font-size: 13px;
    color: #a4d1b4;
}

.dash-stat-value {
    font-size: 12px;
    font-weight: 600;
    color: #dee8df;
}

/* System section */
.dash-section-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 1.2px;
    color: #6e7870;
}

.dash-stat-label {
    font-size: 12px;
    font-weight: 500;
    color: #dee8df;
}

.dash-level {
    background: rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    border: none;
    min-height: 6px;
}

.dash-level trough {
    background: rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    min-height: 6px;
    border: none;
}

.dash-level highlight {
    background: #a4d1b4;
    border-radius: 999px;
    min-height: 6px;
}

/* Calendar */
.dash-calendar {
    background: transparent;
    color: #dee8df;
    font-size: 12px;
    padding: 0;
}

.dash-calendar header {
    color: #dee8df;
    font-weight: bold;
    background: transparent;
}

.dash-calendar button {
    border-radius: 8px;
    color: #dee8df;
    padding: 4px;
    background: transparent;
}

.dash-calendar button:hover {
    background: rgba(164, 209, 180, 0.18);
    color: #a4d1b4;
}

.dash-calendar .day-number {
    border-radius: 50%;
}

.dash-calendar .day-name {
    color: #8d9990;
}

.dash-calendar .day-number.today {
    background: #a4d1b4;
    color: #0b0f0c;
    font-weight: 700;
}

.dash-calendar .day-number:selected {
    background: rgba(164, 209, 180, 0.3);
    color: #a4d1b4;
}

.dash-calendar .day-number:selected.today {
    background: #a4d1b4;
    color: #0b0f0c;
}

/* --- System Resource Island --- */
.sysinfo-group {
    background: rgba(14, 19, 16, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 4px 10px;
    margin: 3px 3px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
}

.sys-item {
    background: transparent;
    border: none;
    color: #dee8df;
    padding: 0px 4px;
    font-size: 12px;
    font-weight: 500;
}

.sys-item.cpu {
    color: #a4d1b4;
}

.sys-item.cpu.warning {
    color: #cec06b;
}

.sys-item.cpu.critical {
    color: #fa746f;
}

.sys-item.mem {
    color: #7ad9bc;
}

.sys-item.mem.warning {
    color: #cec06b;
}

.sys-item.mem.critical {
    color: #fa746f;
}

/* --- Unified Quick Settings Island (GNOME / Caelestia style) --- */
.quicksettings-pill {
    background: rgba(14, 19, 16, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 4px 12px;
    margin: 3px 3px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
    transition: all 180ms ease;
}

.quicksettings-pill:hover {
    background: rgba(22, 30, 25, 0.95);
    border-color: rgba(164, 209, 180, 0.3);
}

.qs-pill-icon {
    font-size: 13px;
    margin: 0 2px;
}

.qs-pill-net {
    color: #9cebcc;
}

.qs-pill-bt {
    color: #dee8df;
}

.qs-pill-audio {
    color: #86dcce;
    font-weight: 500;
}

.qs-pill-bat {
    color: #a3f1bd;
    font-weight: 500;
}

/* --- Notification Bell Island --- */
.bell {
    background: rgba(14, 19, 16, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 4px 10px;
    margin: 3px 0 3px 3px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
    color: #a4aea5;
    font-size: 14px;
    transition: all 180ms ease;
}

.bell:hover {
    background: rgba(22, 30, 25, 0.95);
    border-color: rgba(164, 209, 180, 0.3);
    color: #dee8df;
}

.bell.has-unread {
    color: #a4d1b4;
}

.bell-icon {
    font-size: 14px;
}

.notif-badge {
    background: #a4d1b4;
    color: #141b17;
    font-size: 10px;
    font-weight: bold;
    border-radius: 999px;
    padding: 0 5px;
    min-width: 14px;
}

/* --- Notification Center Dropdown --- */
.notif-center {
    background: transparent;
}

.notif-dropdown {
    background: rgba(12, 17, 23, 0.98);
    border: 1px solid rgba(130, 207, 218, 0.24);
    border-radius: 20px;
    padding: 14px;
    min-width: 420px;
    min-height: 540px;
    box-shadow: 0 18px 45px rgba(0, 0, 0, 0.7), 0 0 0 1px rgba(255, 255, 255, 0.03) inset;
}

.notif-header {
    padding: 5px 6px 11px 6px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    margin-bottom: 8px;
}

.notif-header-title {
    font-weight: bold;
    font-size: 15px;
    color: #dee8df;
}

.notif-clear-btn {
    background: rgba(130, 207, 218, 0.1);
    border: 1px solid rgba(130, 207, 218, 0.16);
    border-radius: 999px;
    padding: 4px 10px;
    color: #a4aea5;
    font-size: 11px;
    transition: all 150ms ease;
}

.notif-clear-btn:hover {
    background: rgba(250, 116, 111, 0.16);
    border-color: rgba(250, 116, 111, 0.7);
    color: #ff9388;
}

.notif-empty {
    padding: 24px 16px;
    color: #6e7870;
}

.notif-empty-icon {
    font-size: 28px;
    margin-bottom: 4px;
    color: #414a43;
}

.notif-empty-text {
    font-size: 13px;
    font-weight: 500;
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
    background: rgba(14, 20, 17, 0.96);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-left: 4px solid #a4d1b4;
    border-radius: 14px;
    padding: 11px 13px;
    color: #dee8df;
    margin: 3px 0;
    transition: background 150ms ease, border-color 150ms ease;
}

/* Live toast popup — distinct freeyuh island look */
.toast-window .notif-toast {
    background: rgba(10, 16, 12, 0.97);
    border: 1px solid rgba(164, 209, 180, 0.22);
    border-left: 4px solid #a4d1b4;
    border-radius: 16px;
    box-shadow:
        0 16px 40px rgba(0, 0, 0, 0.75),
        0 0 0 1px rgba(255, 255, 255, 0.04) inset,
        0 0 18px rgba(164, 209, 180, 0.08);
}

/* Critical urgency — red accent */
.notif-toast.critical {
    border-left: 4px solid #fa746f;
    border-color: rgba(250, 116, 111, 0.35);
    box-shadow:
        0 16px 40px rgba(0, 0, 0, 0.75),
        0 0 18px rgba(250, 116, 111, 0.14);
}

/* Slide-in animation applied when toast is first shown */
.toast-window .notif-toast.toast-entering {
    animation: toast-in 300ms cubic-bezier(0.22, 1, 0.36, 1) both;
}

/* Slide-out animation applied just before the widget is removed */
.toast-window .notif-toast.toast-leaving {
    animation: toast-out 200ms ease-in both;
    pointer-events: none;
}

/* App name chip */
.notif-app {
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #7ad9bc;
    background: rgba(122, 217, 188, 0.14);
    border: 1px solid rgba(122, 217, 188, 0.2);
    padding: 1px 7px;
    border-radius: 6px;
}

/* Close button inside notification center cards */
.notif-card-close {
    background: transparent;
    border: none;
    color: #6e7870;
    font-size: 11px;
    padding: 1px 4px;
    border-radius: 4px;
}

.notif-card-close:hover {
    background: rgba(255, 255, 255, 0.10);
    color: #dee8df;
}

/* Notification summary / title */
.notif-title {
    font-weight: 700;
    font-size: 14px;
    color: #ffffff;
    margin-top: 3px;
}

/* Notification body text */
.notif-body {
    font-size: 12px;
    color: #8d9990;
    margin-top: 1px;
    line-height: 1.45;
}

.notif-center scrollbar {
    background: transparent;
}

/* =========================================================================
   UNIFIED QUICK SETTINGS / CONTROL CENTER PANEL (GNOME / Caelestia M3)
   ========================================================================= */

.qs-window {
    background: transparent;
}

.qs-dropdown {
    background: rgba(12, 17, 23, 0.98);
    border: 1px solid rgba(130, 207, 218, 0.24);
    border-radius: 24px;
    padding: 18px;
    min-width: 440px;
    min-height: 540px;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.78), 0 0 0 1px rgba(255, 255, 255, 0.04) inset;
}

.qs-page {
    padding: 2px 1px;
}

/* --- Header Row --- */
.qs-header-row {
    padding: 2px 4px 12px 4px;
    border-bottom: 1px solid rgba(130, 207, 218, 0.14);
    margin-bottom: 8px;
}

.qs-header-title {
    font-size: 16px;
    font-weight: 700;
    color: #ffffff;
}

.qs-header-battery {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 999px;
    padding: 3px 10px;
    font-size: 12px;
    font-weight: 600;
    color: #dee8df;
}

.qs-header-battery.charging {
    background: rgba(164, 209, 180, 0.15);
    border-color: rgba(164, 209, 180, 0.3);
    color: #a4d1b4;
}

/* --- GNOME Quick Toggle Tiles Grid --- */
.qs-tiles-container {
    margin-bottom: 2px;
}

.qs-tile {
    background: rgba(27, 39, 48, 0.86);
    border: 1px solid rgba(130, 207, 218, 0.12);
    border-radius: 16px;
    padding: 9px 10px;
    transition: all 150ms ease;
}

.qs-tile:hover {
    background: rgba(35, 52, 61, 0.96);
    border-color: rgba(130, 207, 218, 0.35);
}

.qs-tile.active {
    background: rgba(63, 116, 117, 0.34);
    border-color: rgba(130, 207, 218, 0.5);
}

.qs-tile-icon-btn {
    background: rgba(255, 255, 255, 0.06);
    border: none;
    border-radius: 999px;
    min-width: 38px;
    min-height: 38px;
    font-size: 16px;
    color: #dee8df;
    padding: 0;
    transition: all 150ms ease;
}

.qs-tile-icon-btn:hover {
    background: rgba(255, 255, 255, 0.12);
}

.qs-tile.active .qs-tile-icon-btn {
    background: #82cfd7;
    color: #0b0f0c;
}

.qs-tile-text-btn {
    background: transparent;
    border: none;
    padding: 0 4px;
}

.qs-tile-title {
    font-size: 12px;
    font-weight: 600;
    color: #ffffff;
}

.qs-tile-sub {
    font-size: 10px;
    font-weight: 500;
    color: #8d9990;
}

.qs-tile.active .qs-tile-sub {
    color: #82cfd7;
}

.qs-tile-arrow-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 28px;
    min-height: 28px;
    font-size: 15px;
    color: #8d9990;
    padding: 0;
    transition: all 150ms ease;
}

.qs-tile-arrow-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #ffffff;
}

/* --- Volume Slider Card --- */
.qs-slider-card {
    background: rgba(22, 31, 39, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    padding: 9px 12px;
    margin-top: 6px;
}

.qs-slider-mute-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 32px;
    min-height: 32px;
    font-size: 16px;
    color: #82cfd7;
    padding: 0;
    transition: all 150ms ease;
}

.qs-slider-mute-btn.muted {
    color: #fa746f;
}

.qs-slider-mute-btn:hover {
    background: rgba(255, 255, 255, 0.08);
}

.qs-volume-scale trough {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 999px;
    min-height: 6px;
    border: none;
}

.qs-volume-scale highlight {
    background: #82cfd7;
    border-radius: 999px;
    min-height: 6px;
}

.qs-volume-scale slider {
    background: #dee8df;
    border-radius: 999px;
    min-width: 14px;
    min-height: 14px;
    margin: -4px 0;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.5);
    border: none;
}

.qs-slider-pct {
    font-size: 11px;
    font-weight: 600;
    color: #8d9990;
    min-width: 36px;
    text-align: right;
}

/* --- Detail Navigation Bar --- */
.qs-nav-bar {
    padding-bottom: 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    margin-bottom: 6px;
}

.qs-back-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 32px;
    min-height: 32px;
    font-size: 18px;
    color: #dee8df;
    padding: 0;
    transition: all 150ms ease;
}

.qs-back-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #a4d1b4;
}

.qs-nav-title {
    font-size: 14px;
    font-weight: 700;
    color: #ffffff;
}

.qs-rescan-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 32px;
    min-height: 32px;
    font-size: 15px;
    color: #8d9990;
    padding: 0;
    transition: all 150ms ease;
}

.qs-rescan-btn:hover {
    background: rgba(164, 209, 180, 0.18);
    color: #a4d1b4;
}

/* Material 3 Switch Component */
switch.qs-switch {
    background: rgba(255, 255, 255, 0.14);
    border-radius: 999px;
    border: none;
    padding: 0;
    outline: none;
    box-shadow: none;
    transition: all 200ms ease;
}

switch.qs-switch:checked {
    background: #a4d1b4;
}

switch.qs-switch slider {
    background: #dee8df;
    border-radius: 999px;
    margin: 2px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
    transition: all 200ms ease;
}

switch.qs-switch:checked slider {
    background: #0b0f0c;
}

/* Feedback banner */
.qs-status-banner {
    background: rgba(164, 209, 180, 0.12);
    border: 1px solid rgba(164, 209, 180, 0.25);
    border-radius: 999px;
    padding: 4px 14px;
    color: #a4d1b4;
    font-size: 11px;
    font-weight: 600;
    margin: 4px 0;
}

/* Detail Scrolled List Items (Flat Caelestia / Backgroundless) */
.qs-list-box {
    margin-top: 2px;
}

.qs-item {
    background: rgba(27, 39, 48, 0.42);
    border: 1px solid rgba(255, 255, 255, 0.04);
    border-radius: 12px;
    padding: 8px 10px;
    margin: 2px 0;
    transition: all 150ms ease;
}

.qs-item:hover {
    background: rgba(35, 52, 61, 0.72);
    border-color: rgba(130, 207, 218, 0.2);
}

.qs-item.connected .qs-item-name {
    color: #a4d1b4;
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
}

.qs-item-name {
    color: #dee8df;
    font-weight: 500;
    font-size: 13px;
}

.qs-connected-icon {
    color: #a4d1b4;
    font-size: 14px;
    font-weight: bold;
}

.qs-saved-icon {
    color: #6e7870;
    font-size: 12px;
}

.qs-paired-icon {
    color: #6e7870;
    font-size: 12px;
}

.qs-lock-icon {
    color: #6e7870;
    font-size: 12px;
}

.qs-item-signal {
    color: #6e7870;
    font-size: 11px;
    font-weight: 500;
}

.qs-item-battery {
    color: #8d9990;
    font-size: 11px;
    font-weight: 500;
}

.qs-connect-btn {
    background: transparent;
    border: none;
    border-radius: 8px;
    color: #8d9990;
    font-size: 16px;
    padding: 4px 8px;
    transition: all 150ms ease;
}

.qs-connect-btn:hover {
    background: rgba(164, 209, 180, 0.18);
    color: #a4d1b4;
}

.qs-disconnect-btn {
    background: transparent;
    border: none;
    border-radius: 8px;
    color: #fa746f;
    font-size: 16px;
    padding: 4px 8px;
    transition: all 150ms ease;
}

.qs-disconnect-btn:hover {
    background: rgba(250, 116, 111, 0.18);
    color: #fa746f;
}

/* Auth Overlay Page */
.qs-auth-page {
    padding: 6px 12px 14px 12px;
}

.qs-auth-icon {
    font-size: 38px;
    color: #a4d1b4;
    margin-bottom: 4px;
}

.qs-auth-ssid {
    font-size: 17px;
    font-weight: 700;
    color: #ffffff;
}

.qs-auth-sub {
    font-size: 12px;
    color: #8d9990;
    font-weight: 500;
}

.qs-auth-input-card {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 16px;
    padding: 12px 14px;
    margin: 6px 0;
}

.qs-auth-input-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.8px;
    color: #6e7870;
    margin-bottom: 4px;
}

.qs-auth-entry {
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 10px;
    color: #dee8df;
    font-size: 13px;
    padding: 8px 12px;
    transition: all 150ms ease;
}

.qs-auth-entry:focus {
    border-color: #a4d1b4;
    background: rgba(0, 0, 0, 0.55);
    box-shadow: 0 0 0 1px rgba(164, 209, 180, 0.5);
}

.qs-auth-status {
    font-size: 12px;
    font-weight: 600;
    padding: 4px 8px;
}

.qs-auth-status.error {
    color: #fa746f;
}

.qs-auth-status.connecting {
    color: #a4d1b4;
}

.qs-auth-actions {
    margin-top: 10px;
}

.qs-auth-cancel-btn {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    color: #dee8df;
    font-size: 13px;
    font-weight: 600;
    padding: 10px 16px;
    transition: all 150ms ease;
}

.qs-auth-cancel-btn:hover {
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
}

.qs-auth-connect-btn {
    background: #a4d1b4;
    border: none;
    border-radius: 12px;
    color: #0b0f0c;
    font-size: 13px;
    font-weight: 700;
    padding: 10px 16px;
    transition: all 150ms ease;
}

.qs-auth-connect-btn:hover {
    background: #bbf0cb;
    box-shadow: 0 0 14px rgba(164, 209, 180, 0.45);
}

/* Empty / Scanning states */
.qs-empty {
    padding: 40px 16px;
    color: #6e7870;
}

.qs-empty-icon {
    font-size: 34px;
    margin-bottom: 8px;
    color: #414a43;
}

.qs-empty-text {
    font-size: 13px;
    font-weight: 600;
    color: #dee8df;
}

.qs-empty-sub {
    font-size: 11px;
    color: #6e7870;
    margin-top: 2px;
}

.qs-scanning {
    color: #a4aea5;
    font-size: 13px;
    padding: 20px;
}

/* Thin Custom Scrollbar */
.qs-scrolled-window scrollbar {
    background: transparent;
    border: none;
    min-width: 4px;
}

.qs-scrolled-window scrollbar slider {
    background: rgba(255, 255, 255, 0.12);
    border-radius: 999px;
    min-width: 4px;
    border: none;
}

.qs-scrolled-window scrollbar slider:hover {
    background: rgba(164, 209, 180, 0.35);
}

/* =========================================================================
   SPOTLIGHT APP LAUNCHER (Caelestia / macOS Spotlight style)
   ========================================================================= */

.launcher-window {
    background: transparent;
}

.launcher-card {
    background: rgba(14, 19, 16, 0.96);
    border: 1px solid rgba(164, 209, 180, 0.25);
    border-radius: 24px;
    padding: 14px 16px;
    min-width: 560px;
    box-shadow: 0 24px 60px rgba(0, 0, 0, 0.85), 0 0 0 1px rgba(255, 255, 255, 0.05) inset;
}

.launcher-search-box {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    padding: 6px 14px;
    margin-bottom: 8px;
    transition: all 150ms ease;
}

.launcher-search-box:focus-within {
    background: rgba(255, 255, 255, 0.07);
    border-color: rgba(164, 209, 180, 0.4);
    box-shadow: 0 0 0 1px rgba(164, 209, 180, 0.3);
}

.launcher-search-icon {
    font-size: 18px;
    color: #a4d1b4;
    margin-right: 4px;
}

.launcher-search-entry {
    background: transparent;
    border: none;
    color: #ffffff;
    font-size: 15px;
    font-weight: 500;
    outline: none;
    box-shadow: none;
    padding: 4px 0;
}

.launcher-search-entry:focus {
    background: transparent;
    border: none;
    outline: none;
    box-shadow: none;
}

.launcher-chip {
    background: rgba(255, 255, 255, 0.08);
    border-radius: 6px;
    padding: 2px 6px;
    font-size: 10px;
    font-weight: 700;
    color: #8d9990;
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
    background: rgba(255, 255, 255, 0.05);
}

.launcher-item.selected {
    background: rgba(164, 209, 180, 0.14);
    border-color: rgba(164, 209, 180, 0.35);
}

.launcher-item-icon {
    margin-right: 4px;
}

.launcher-item-title {
    font-size: 13.5px;
    font-weight: 600;
    color: #dee8df;
}

.launcher-item.selected .launcher-item-title {
    color: #ffffff;
    font-weight: 700;
}

.launcher-item-desc {
    font-size: 11px;
    color: #6e7870;
}

.launcher-item.selected .launcher-item-desc {
    color: #a4d1b4;
}

.launcher-enter-hint {
    color: #6e7870;
    font-size: 14px;
    margin-left: 8px;
    opacity: 0;
    transition: opacity 120ms ease;
}

.launcher-item.selected .launcher-enter-hint {
    color: #a4d1b4;
    opacity: 1;
}

.launcher-footer {
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    padding-top: 8px;
    margin-top: 6px;
}

.launcher-hint {
    font-size: 11px;
    color: #6e7870;
    font-weight: 500;
}

.launcher-count {
    font-size: 11px;
    color: #8d9990;
    font-weight: 600;
}

.launcher-empty {
    padding: 40px 16px;
    color: #6e7870;
}

.launcher-empty-icon {
    font-size: 34px;
    margin-bottom: 8px;
    color: #414a43;
}

.launcher-empty-text {
    font-size: 13px;
    font-weight: 600;
    color: #dee8df;
}

.launcher-scrolled-window scrollbar {
    background: transparent;
    border: none;
    min-width: 4px;
}

.launcher-scrolled-window scrollbar slider {
    background: rgba(255, 255, 255, 0.12);
    border-radius: 999px;
    min-width: 4px;
    border: none;
}

.launcher-scrolled-window scrollbar slider:hover {
    background: rgba(164, 209, 180, 0.35);
}

/* =========================================================================
   POWER & SESSION MENU HUD
   ========================================================================= */

.powermenu-backdrop {
    background: rgba(4, 7, 5, 0.72);
}

.powermenu-card {
    background: rgba(14, 19, 16, 0.96);
    border: 1px solid rgba(164, 209, 180, 0.25);
    border-radius: 28px;
    padding: 36px 44px;
    box-shadow: 0 32px 80px rgba(0, 0, 0, 0.9), 0 0 0 1px rgba(255, 255, 255, 0.05) inset;
}

.powermenu-title {
    font-size: 20px;
    font-weight: 700;
    color: #dee8df;
}

.powermenu-subtitle {
    font-size: 12px;
    font-weight: 500;
    color: #6e7870;
    margin-top: 2px;
}

.powermenu-actions-row {
    margin-top: 10px;
}

.powermenu-tile {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 20px;
    padding: 24px 18px;
    min-width: 106px;
    min-height: 124px;
    transition: all 180ms ease;
}

.powermenu-tile-icon {
    font-size: 34px;
    margin-bottom: 4px;
    color: #8d9990;
    transition: all 180ms ease;
}

.powermenu-tile-name {
    font-size: 13px;
    font-weight: 600;
    color: #dee8df;
}

.powermenu-tile-key {
    font-size: 10px;
    font-weight: 700;
    color: #6e7870;
    background: rgba(255, 255, 255, 0.06);
    border-radius: 6px;
    padding: 2px 6px;
    margin-top: 4px;
}

/* Specific Action Color Accents */
.action-poweroff.selected,
.action-poweroff:hover {
    background: rgba(250, 116, 111, 0.18);
    border-color: rgba(250, 116, 111, 0.6);
    box-shadow: 0 0 24px rgba(250, 116, 111, 0.35);
}
.action-poweroff.selected .powermenu-tile-icon,
.action-poweroff:hover .powermenu-tile-icon {
    color: #fa746f;
}

.action-reboot.selected,
.action-reboot:hover {
    background: rgba(240, 178, 122, 0.18);
    border-color: rgba(240, 178, 122, 0.6);
    box-shadow: 0 0 24px rgba(240, 178, 122, 0.35);
}
.action-reboot.selected .powermenu-tile-icon,
.action-reboot:hover .powermenu-tile-icon {
    color: #f0b27a;
}

.action-suspend.selected,
.action-suspend:hover {
    background: rgba(133, 193, 233, 0.18);
    border-color: rgba(133, 193, 233, 0.6);
    box-shadow: 0 0 24px rgba(133, 193, 233, 0.35);
}
.action-suspend.selected .powermenu-tile-icon,
.action-suspend:hover .powermenu-tile-icon {
    color: #85c1e9;
}

.action-lock.selected,
.action-lock:hover {
    background: rgba(164, 209, 180, 0.18);
    border-color: rgba(164, 209, 180, 0.6);
    box-shadow: 0 0 24px rgba(164, 209, 180, 0.35);
}
.action-lock.selected .powermenu-tile-icon,
.action-lock:hover .powermenu-tile-icon {
    color: #a4d1b4;
}

.action-logout.selected,
.action-logout:hover {
    background: rgba(187, 143, 206, 0.18);
    border-color: rgba(187, 143, 206, 0.6);
    box-shadow: 0 0 24px rgba(187, 143, 206, 0.35);
}
.action-logout.selected .powermenu-tile-icon,
.action-logout:hover .powermenu-tile-icon {
    color: #bb8fce;
}
"#;
