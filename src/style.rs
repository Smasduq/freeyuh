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
.bar {
    background: rgba(20, 20, 20, 0.85);
    font-size: 13px;
    color: #d8dee9;
}
.workspaces button {
    background: transparent;
    border: 2px solid #4a5568;
    border-radius: 999px;
    min-width: 9px;
    min-height: 9px;
    padding: 0;
    margin: 0 3px;
}
.workspaces button:hover {
    border-color: #ffffff;
    background: rgba(255,255,255,0.10);
}
.workspaces button.free {
    border: 2px dashed #5a6474;
    opacity: 0.75;
}
.workspaces button.occupied {
    background: #d8dee9;
    border-color: #d8dee9;
}
.workspaces button.active {
    background: #5294e2;
    border-color: #5294e2;
    border-radius: 999px;
    min-width: 13px;
    min-height: 13px;
}
.clock {
    padding: 4px 10px;
    background: rgba(255,255,255,0.06);
    border-radius: 12px;
}
.sys-item {
    font-weight: bold;
    color: #a8b2c1;
    padding: 4px 8px;
    background: rgba(255, 255, 255, 0.06);
    border-radius: 12px;
    font-size: 13px;
}
.sys-item.bat-icon {
    padding: 4px 6px;
    background: transparent;
}
.sys-item.bat {
    color: #9ece9e;
}
.sys-item.audio {
    color: #8ab4f8;
    font-size: 16px;
}
.sys-item.audio.muted {
    color: #e06c75;
}
.bell {
    background: transparent;
    border: none;
    color: #a8b2c1;
    font-size: 16px;
    padding: 2px 6px;
    border-radius: 12px;
}
.bell:hover {
    background: rgba(255, 255, 255, 0.10);
}
.notif-toast {
    background: rgba(30, 30, 30, 0.92);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 10px;
    padding: 10px 12px;
    color: #d8dee9;
}
.notif-toast.critical {
    border-left: 3px solid #e06c75;
}
.notif-title {
    font-weight: bold;
    font-size: 13px;
    color: #ffffff;
}
.notif-body {
    font-size: 12px;
    color: #c6cdd7;
}
.notif-app {
    font-size: 10px;
    color: #7f8b9b;
}
.notif-center {
    background: transparent;
}
.notif-dropdown {
    background: rgba(20, 20, 20, 0.95);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 12px;
    padding: 6px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
}
.notif-header {
    padding: 4px 8px 8px 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}
.notif-header-title {
    font-weight: bold;
    font-size: 13px;
    color: #ffffff;
}
.notif-center .notif-toast {
    background: rgba(24, 24, 24, 0.95);
}
.notif-center .history-item {
    margin: 2px 0;
}
.notif-center scrollbar {
    background: transparent;
}
"#;
