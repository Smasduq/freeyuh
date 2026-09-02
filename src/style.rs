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
/* --- Top Bar Root --- */
.bar {
    background: rgba(11, 15, 12, 0.94);
    border-bottom: 1px solid rgba(164, 209, 180, 0.18);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.4);
    padding: 0 10px;
    color: #dee8df;
    font-family: "Adwaita Sans", "Inter", "JetBrainsMono Nerd Font", system-ui, sans-serif;
    font-size: 13px;
    min-height: 36px;
}

/* --- Workspaces Widget --- */
.workspaces {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 999px;
    padding: 4px 8px;
    margin: 4px 4px 4px 2px;
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

/* --- Active Window Widget --- */
.active-window {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 999px;
    padding: 3px 12px;
    margin: 4px 4px;
    color: #a4aea5;
    font-size: 12px;
    font-weight: 500;
}

/* --- Clock Pill & Calendar --- */
.clock-pill {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 999px;
    padding: 3px 14px;
    margin: 4px 0;
    transition: all 150ms ease;
}

.clock-pill:hover {
    background: rgba(255, 255, 255, 0.09);
    border-color: rgba(164, 209, 180, 0.25);
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

/* --- System Resource Items --- */
.sysinfo-group {
    background: transparent;
    border: none;
    padding: 0;
    margin: 0 4px;
}

.sys-item {
    background: transparent;
    border: none;
    color: #dee8df;
    padding: 0px 5px;
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

.sys-item.bat {
    color: #a3f1bd;
}

.sys-item.bat.charging {
    color: #a4d1b4;
    font-weight: bold;
}

.sys-item.bat.warning {
    color: #cec06b;
}

.sys-item.bat.critical {
    color: #fa746f;
    font-weight: bold;
}

/* --- Network Icon (Status Bar) --- */
.network-pill {
    background: transparent;
    border: none;
    border-radius: 6px;
    padding: 3px 6px;
    margin: 4px 1px;
    transition: all 150ms ease;
}

.network-pill:hover {
    background: rgba(255, 255, 255, 0.09);
}

.network-pill.connected {
    border: none;
}

.network-label {
    font-size: 14px;
    color: #9cebcc;
}

/* --- Audio Icon (Status Bar) --- */
.sys-item.audio {
    background: transparent;
    border: none;
    border-radius: 6px;
    padding: 3px 6px;
    margin: 4px 1px;
    color: #86dcce;
    font-size: 12px;
    font-weight: 500;
    transition: all 150ms ease;
}

.sys-item.audio:hover {
    background: rgba(255, 255, 255, 0.09);
}

.sys-item.audio.muted {
    color: #fa746f;
}

/* --- Notification Bell --- */
.bell {
    background: transparent;
    border: none;
    border-radius: 6px;
    color: #a4aea5;
    font-size: 14px;
    padding: 3px 6px;
    margin: 4px 2px 4px 1px;
    transition: all 150ms ease;
}

.bell:hover {
    background: rgba(255, 255, 255, 0.09);
    color: #dee8df;
}

.bell.has-unread {
    background: transparent;
    border: none;
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

/* --- Wi-Fi Connection Panel --- */
.wifi-window {
    background: transparent;
}

.wifi-dropdown {
    background: rgba(14, 20, 16, 0.96);
    border: 1px solid rgba(164, 209, 180, 0.22);
    border-radius: 18px;
    padding: 14px;
    min-width: 390px;
    min-height: 500px;
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.7);
}

.wifi-header {
    padding-bottom: 10px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    margin-bottom: 8px;
}

.wifi-title-icon {
    color: #a4d1b4;
    font-size: 18px;
}

.wifi-header-title {
    color: #ffffff;
    font-weight: bold;
    font-size: 14px;
}

.wifi-header-subtitle {
    color: #6e7870;
    font-size: 11px;
    margin-top: -1px;
}

.wifi-rescan-btn {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    color: #dee8df;
    font-size: 11px;
    font-weight: 500;
    padding: 4px 10px;
    transition: all 150ms ease;
}

.wifi-rescan-btn:hover {
    background: rgba(164, 209, 180, 0.2);
    border-color: #a4d1b4;
    color: #a4d1b4;
}

.wifi-status-label {
    color: #a4d1b4;
    font-size: 11px;
    padding: 4px 8px;
    font-weight: 500;
}

.wifi-item {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    padding: 10px 12px;
    margin: 3px 0;
    transition: all 150ms ease;
}

.wifi-item:hover {
    background: rgba(255, 255, 255, 0.07);
    border-color: rgba(164, 209, 180, 0.3);
}

.wifi-item.connected {
    background: rgba(164, 209, 180, 0.09);
    border: 1px solid rgba(164, 209, 180, 0.35);
    border-left: 4px solid #a4d1b4;
}

.wifi-item-icon {
    font-size: 15px;
}

.wifi-item-name {
    color: #ffffff;
    font-weight: 600;
    font-size: 13px;
}

.wifi-connected-badge {
    background: rgba(164, 209, 180, 0.2);
    color: #a4d1b4;
    font-size: 10px;
    font-weight: 600;
    padding: 1px 6px;
    border-radius: 4px;
}

.wifi-saved-badge {
    background: rgba(255, 255, 255, 0.08);
    color: #a4aea5;
    font-size: 10px;
    padding: 1px 6px;
    border-radius: 4px;
}

.wifi-lock-icon {
    color: #6e7870;
    font-size: 12px;
}

.wifi-item-signal {
    color: #6e7870;
    font-size: 11px;
}

.wifi-connect-btn {
    background: #a4d1b4;
    border: none;
    border-radius: 8px;
    color: #0b0f0c;
    font-weight: bold;
    font-size: 11px;
    padding: 4px 12px;
    transition: all 150ms ease;
}

.wifi-connect-btn:hover {
    background: #bbf0cb;
    box-shadow: 0 0 8px rgba(164, 209, 180, 0.35);
}

.wifi-disconnect-btn {
    background: rgba(250, 116, 111, 0.12);
    border: 1px solid rgba(250, 116, 111, 0.3);
    border-radius: 8px;
    color: #fa746f;
    font-size: 11px;
    font-weight: 500;
    padding: 4px 10px;
    transition: all 150ms ease;
}

.wifi-disconnect-btn:hover {
    background: rgba(250, 116, 111, 0.25);
}

.wifi-password-box {
    background: rgba(0, 0, 0, 0.35);
    border: 1px solid rgba(164, 209, 180, 0.18);
    border-radius: 10px;
    padding: 8px 10px;
    margin-top: 6px;
}

.wifi-password-entry {
    background: rgba(255, 255, 255, 0.07);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    color: #dee8df;
    font-size: 12px;
    padding: 5px 10px;
}

.wifi-password-entry:focus {
    border-color: #a4d1b4;
}

.wifi-join-btn {
    background: #a4d1b4;
    border: none;
    border-radius: 8px;
    color: #0b0f0c;
    font-weight: bold;
    font-size: 11px;
    padding: 5px 12px;
}

.wifi-cancel-btn {
    background: transparent;
    border: none;
    color: #6e7870;
    font-size: 12px;
    padding: 5px 8px;
    border-radius: 6px;
}

.wifi-cancel-btn:hover {
    color: #dee8df;
    background: rgba(255, 255, 255, 0.1);
}

.wifi-empty {
    padding: 40px 16px;
    color: #6e7870;
}

.wifi-empty-icon {
    font-size: 32px;
    margin-bottom: 6px;
    color: #414a43;
}

.wifi-empty-text {
    font-size: 13px;
    font-weight: 500;
}

.wifi-empty-sub {
    font-size: 11px;
    color: #6e7870;
    margin-top: 2px;
}

.wifi-scanning {
    color: #a4aea5;
    font-size: 13px;
    padding: 20px;
}
"#;
