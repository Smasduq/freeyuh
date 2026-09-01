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
    font-size: 12px;
}
.sys-item.bat-icon {
    padding: 4px 6px;
    background: transparent;
}
.sys-item.bat {
    color: #9ece9e;
}
"#;
