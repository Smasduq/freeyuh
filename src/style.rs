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

.calendar-dropdown {
    background: rgba(16, 23, 19, 0.96);
    border: 1px solid rgba(164, 209, 180, 0.2);
    border-radius: 16px;
    padding: 14px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
}

.calendar-header {
    color: #a4d1b4;
    font-weight: bold;
    font-size: 14px;
    padding-bottom: 4px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.calendar-widget {
    background: transparent;
    color: #dee8df;
    font-size: 12px;
}

.calendar-widget header {
    color: #dee8df;
    font-weight: bold;
}

.calendar-widget button {
    border-radius: 6px;
    color: #dee8df;
    padding: 4px;
}

.calendar-widget button:hover {
    background: rgba(164, 209, 180, 0.2);
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
    background: rgba(16, 23, 19, 0.96);
    border: 1px solid rgba(164, 209, 180, 0.2);
    border-radius: 16px;
    padding: 12px;
    min-width: 400px;
    min-height: 520px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
}

.notif-header {
    padding: 4px 6px 8px 6px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    margin-bottom: 6px;
}

.notif-header-title {
    font-weight: bold;
    font-size: 14px;
    color: #dee8df;
}

.notif-clear-btn {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 2px 8px;
    color: #a4aea5;
    font-size: 11px;
    transition: all 150ms ease;
}

.notif-clear-btn:hover {
    background: rgba(250, 116, 111, 0.2);
    border-color: #fa746f;
    color: #fa746f;
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

/* --- Notification Cards & Toasts --- */
.toast-window {
    background: transparent;
}

.notif-toast {
    background: rgba(26, 33, 28, 0.94);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    padding: 8px 12px;
    color: #dee8df;
    margin: 2px 0;
    transition: all 150ms ease;
}

.toast-window .notif-toast {
    background: rgba(16, 23, 19, 0.96);
    border: 1px solid rgba(164, 209, 180, 0.2);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
}

.notif-toast.critical {
    border-left: 3px solid #fa746f;
}

.notif-app {
    font-size: 10px;
    font-weight: bold;
    text-transform: uppercase;
    color: #a4d1b4;
    background: rgba(164, 209, 180, 0.12);
    padding: 1px 6px;
    border-radius: 6px;
}

.notif-card-close {
    background: transparent;
    border: none;
    color: #6e7870;
    font-size: 11px;
    padding: 1px 4px;
    border-radius: 4px;
}

.notif-card-close:hover {
    background: rgba(255, 255, 255, 0.12);
    color: #dee8df;
}

.notif-title {
    font-weight: bold;
    font-size: 13px;
    color: #ffffff;
    margin-top: 2px;
}

.notif-body {
    font-size: 12px;
    color: #a4aea5;
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
    background: rgba(14, 19, 16, 0.95);
    border: 1px solid rgba(164, 209, 180, 0.2);
    border-radius: 24px;
    padding: 16px;
    min-width: 380px;
    min-height: 480px;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.75), 0 0 0 1px rgba(255, 255, 255, 0.04) inset;
}

.qs-page {
    padding: 2px;
}

/* --- Header Row --- */
.qs-header-row {
    padding: 2px 4px 10px 4px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    margin-bottom: 8px;
}

.qs-header-title {
    font-size: 15px;
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
    margin-bottom: 6px;
}

.qs-tile {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 18px;
    padding: 6px 10px;
    transition: all 150ms ease;
}

.qs-tile:hover {
    background: rgba(255, 255, 255, 0.07);
    border-color: rgba(164, 209, 180, 0.25);
}

.qs-tile.active {
    background: rgba(164, 209, 180, 0.12);
    border-color: rgba(164, 209, 180, 0.35);
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
    background: #a4d1b4;
    color: #0b0f0c;
}

.qs-tile-text-btn {
    background: transparent;
    border: none;
    padding: 0 4px;
}

.qs-tile-title {
    font-size: 13px;
    font-weight: 600;
    color: #ffffff;
}

.qs-tile-sub {
    font-size: 11px;
    font-weight: 500;
    color: #8d9990;
}

.qs-tile.active .qs-tile-sub {
    color: #a4d1b4;
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
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 18px;
    padding: 8px 12px;
    margin-top: 4px;
}

.qs-slider-mute-btn {
    background: transparent;
    border: none;
    border-radius: 999px;
    min-width: 32px;
    min-height: 32px;
    font-size: 16px;
    color: #a4d1b4;
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
    background: #a4d1b4;
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
    min-width: 32px;
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
    background: transparent;
    border: none;
    border-radius: 12px;
    padding: 8px 10px;
    margin: 2px 0;
    transition: all 150ms ease;
}

.qs-item:hover {
    background: rgba(255, 255, 255, 0.06);
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
    max-width: 600px;
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
"#;
