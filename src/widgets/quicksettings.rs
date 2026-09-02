//! Unified GNOME-style Quick Settings widget and dropdown panel.
//!
//! Groups Network (Wi-Fi), Bluetooth, and Sound (Audio volume slider & mute)
//! into a single unified bar pill and an interactive multi-page Control Center panel.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, EventControllerMotion, Label, Orientation,
    PasswordEntry, Scale, ScrolledWindow, Stack, StackTransitionType, Switch,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc;
use std::time::Duration;

use crate::services::audio;
use crate::services::bluetooth::{self, BluetoothDevice, BluetoothState};
use crate::services::network::{self, NetworkState, WifiNetwork};
use crate::widgets::sysinfo;

const HIDE_DELAY: Duration = Duration::from_millis(250);

/// Handles to labels inside the unified Quick Settings bar pill.
pub struct QuickSettingsLabels {
    pub net_icon: Label,
    pub bt_icon: Label,
    pub audio_label: Label,
    pub bat_icon: Label,
}

/// Creates the unified Quick Settings pill button on the bar and its Control Center panel.
///
/// Returns `(pill_button, pill_labels, qs_window, reload_fn)`.
pub fn create(app: &Application) -> (Button, QuickSettingsLabels, ApplicationWindow, Rc<dyn Fn()>) {
    let button = Button::new();
    button.add_css_class("quicksettings-pill");
    button.set_cursor_from_name(Some("pointer"));
    button.set_valign(Align::Center);

    let pill_box = Box::new(Orientation::Horizontal, 6);
    pill_box.set_valign(Align::Center);

    let net_icon = Label::new(None);
    net_icon.set_use_markup(true);
    net_icon.add_css_class("qs-pill-icon");
    net_icon.add_css_class("qs-pill-net");

    let bt_icon = Label::new(None);
    bt_icon.set_use_markup(true);
    bt_icon.add_css_class("qs-pill-icon");
    bt_icon.add_css_class("qs-pill-bt");

    let audio_label = Label::new(None);
    audio_label.set_use_markup(true);
    audio_label.add_css_class("qs-pill-icon");
    audio_label.add_css_class("qs-pill-audio");

    let bat_icon = Label::new(None);
    bat_icon.set_use_markup(true);
    bat_icon.add_css_class("qs-pill-icon");
    bat_icon.add_css_class("qs-pill-bat");

    pill_box.append(&net_icon);
    pill_box.append(&bt_icon);
    pill_box.append(&audio_label);
    pill_box.append(&bat_icon);
    button.set_child(Some(&pill_box));

    // --- Control Center Layer Shell Window ---
    let qs_window = ApplicationWindow::builder().application(app).build();
    qs_window.init_layer_shell();
    qs_window.set_layer(Layer::Top);
    qs_window.set_anchor(Edge::Top, true);
    qs_window.set_anchor(Edge::Right, true);
    qs_window.set_margin(Edge::Top, 42);
    qs_window.set_margin(Edge::Right, 60);
    qs_window.set_keyboard_mode(KeyboardMode::OnDemand);
    qs_window.set_exclusive_zone(0);
    qs_window.set_default_size(440, 540);
    qs_window.add_css_class("qs-window");

    let dropdown = Box::new(Orientation::Vertical, 0);
    dropdown.add_css_class("qs-dropdown");
    dropdown.set_width_request(440);

    // Multi-page navigation stack
    let stack = Stack::new();
    stack.set_transition_type(StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(220);
    stack.set_vexpand(true);

    // =========================================================================
    // PAGE 1: "main" (GNOME Control Center Overview)
    // =========================================================================
    let main_page = Box::new(Orientation::Vertical, 14);
    main_page.add_css_class("qs-page");
    main_page.add_css_class("qs-main-page");

    // Header title with Battery status badge
    let header_box = Box::new(Orientation::Horizontal, 8);
    header_box.add_css_class("qs-header-row");
    header_box.set_valign(Align::Center);

    let header_title = Label::new(Some("Quick Settings"));
    header_title.add_css_class("qs-header-title");
    header_title.set_halign(Align::Start);
    header_title.set_hexpand(true);
    header_box.append(&header_title);

    let header_battery = Label::new(None);
    header_battery.set_use_markup(true);
    header_battery.add_css_class("qs-header-battery");
    header_battery.set_valign(Align::Center);
    header_box.append(&header_battery);

    main_page.append(&header_box);

    // Quick Tiles Grid (Wi-Fi & Bluetooth side-by-side or stacked tiles)
    let tiles_box = Box::new(Orientation::Horizontal, 10);
    tiles_box.add_css_class("qs-tiles-container");
    tiles_box.set_homogeneous(true);

    // --- Wi-Fi Tile ---
    let wifi_tile = Box::new(Orientation::Horizontal, 10);
    wifi_tile.add_css_class("qs-tile");
    wifi_tile.set_valign(Align::Center);
    wifi_tile.set_hexpand(true);

    let wifi_toggle_btn = Button::new();
    wifi_toggle_btn.add_css_class("qs-tile-icon-btn");
    wifi_toggle_btn.set_cursor_from_name(Some("pointer"));
    let wifi_tile_icon = Label::new(None);
    wifi_tile_icon.set_use_markup(true);
    wifi_tile_icon.set_markup("󰤨");
    wifi_toggle_btn.set_child(Some(&wifi_tile_icon));
    wifi_tile.append(&wifi_toggle_btn);

    let wifi_text_btn = Button::new();
    wifi_text_btn.add_css_class("qs-tile-text-btn");
    wifi_text_btn.set_hexpand(true);
    wifi_text_btn.set_cursor_from_name(Some("pointer"));

    let wifi_text_box = Box::new(Orientation::Vertical, 1);
    wifi_text_box.set_halign(Align::Start);
    let wifi_tile_title = Label::new(Some("Wi-Fi"));
    wifi_tile_title.add_css_class("qs-tile-title");
    wifi_tile_title.set_halign(Align::Start);
    let wifi_tile_sub = Label::new(Some("Not Connected"));
    wifi_tile_sub.add_css_class("qs-tile-sub");
    wifi_tile_sub.set_halign(Align::Start);
    wifi_tile_sub.set_ellipsize(EllipsizeMode::End);
    wifi_text_box.append(&wifi_tile_title);
    wifi_text_box.append(&wifi_tile_sub);
    wifi_text_btn.set_child(Some(&wifi_text_box));
    wifi_tile.append(&wifi_text_btn);

    let wifi_arrow_btn = Button::new();
    wifi_arrow_btn.set_label("󰅂");
    wifi_arrow_btn.add_css_class("qs-tile-arrow-btn");
    wifi_arrow_btn.set_cursor_from_name(Some("pointer"));
    wifi_tile.append(&wifi_arrow_btn);

    tiles_box.append(&wifi_tile);

    // --- Bluetooth Tile ---
    let bt_tile = Box::new(Orientation::Horizontal, 10);
    bt_tile.add_css_class("qs-tile");
    bt_tile.set_valign(Align::Center);
    bt_tile.set_hexpand(true);

    let bt_toggle_btn = Button::new();
    bt_toggle_btn.add_css_class("qs-tile-icon-btn");
    bt_toggle_btn.set_cursor_from_name(Some("pointer"));
    let bt_tile_icon = Label::new(None);
    bt_tile_icon.set_use_markup(true);
    bt_tile_icon.set_markup("󰂯");
    bt_toggle_btn.set_child(Some(&bt_tile_icon));
    bt_tile.append(&bt_toggle_btn);

    let bt_text_btn = Button::new();
    bt_text_btn.add_css_class("qs-tile-text-btn");
    bt_text_btn.set_hexpand(true);
    bt_text_btn.set_cursor_from_name(Some("pointer"));

    let bt_text_box = Box::new(Orientation::Vertical, 1);
    bt_text_box.set_halign(Align::Start);
    let bt_tile_title = Label::new(Some("Bluetooth"));
    bt_tile_title.add_css_class("qs-tile-title");
    bt_tile_title.set_halign(Align::Start);
    let bt_tile_sub = Label::new(Some("Disabled"));
    bt_tile_sub.add_css_class("qs-tile-sub");
    bt_tile_sub.set_halign(Align::Start);
    bt_tile_sub.set_ellipsize(EllipsizeMode::End);
    bt_text_box.append(&bt_tile_title);
    bt_text_box.append(&bt_tile_sub);
    bt_text_btn.set_child(Some(&bt_text_box));
    bt_tile.append(&bt_text_btn);

    let bt_arrow_btn = Button::new();
    bt_arrow_btn.set_label("󰅂");
    bt_arrow_btn.add_css_class("qs-tile-arrow-btn");
    bt_arrow_btn.set_cursor_from_name(Some("pointer"));
    bt_tile.append(&bt_arrow_btn);

    tiles_box.append(&bt_tile);
    main_page.append(&tiles_box);

    // --- Audio Volume Slider Card ---
    let volume_card = Box::new(Orientation::Horizontal, 10);
    volume_card.add_css_class("qs-slider-card");
    volume_card.set_valign(Align::Center);

    let mute_btn = Button::new();
    mute_btn.add_css_class("qs-slider-mute-btn");
    mute_btn.set_cursor_from_name(Some("pointer"));
    let mute_icon = Label::new(None);
    mute_icon.set_use_markup(true);
    mute_icon.set_markup("󰕾");
    mute_btn.set_child(Some(&mute_icon));
    volume_card.append(&mute_btn);

    let volume_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    volume_scale.add_css_class("qs-volume-scale");
    volume_scale.set_hexpand(true);
    volume_scale.set_draw_value(false);
    if let Some(state) = audio::query() {
        volume_scale.set_value(state.volume_percent as f64);
    }
    volume_card.append(&volume_scale);

    let volume_pct_label = Label::new(Some("80%"));
    volume_pct_label.add_css_class("qs-slider-pct");
    volume_card.append(&volume_pct_label);

    main_page.append(&volume_card);

    // --- Screen Brightness Slider Card ---
    let bright_card = Box::new(Orientation::Horizontal, 10);
    bright_card.add_css_class("qs-slider-card");
    bright_card.set_valign(Align::Center);

    let bright_icon_btn = Box::new(Orientation::Horizontal, 0);
    bright_icon_btn.add_css_class("qs-slider-mute-btn");
    let bright_icon_lbl = Label::new(Some("󰃠"));
    bright_icon_lbl.add_css_class("qs-slider-icon");
    bright_icon_btn.append(&bright_icon_lbl);
    bright_card.append(&bright_icon_btn);

    let bright_scale = Scale::with_range(Orientation::Horizontal, 1.0, 100.0, 1.0);
    bright_scale.add_css_class("qs-volume-scale");
    bright_scale.add_css_class("qs-brightness-scale");
    bright_scale.set_hexpand(true);
    bright_scale.set_draw_value(false);
    let initial_b = crate::services::brightness::query().unwrap_or(80);
    bright_scale.set_value(initial_b as f64);
    bright_icon_lbl.set_text(crate::services::brightness::icon(initial_b));
    bright_card.append(&bright_scale);

    let bright_pct_label = Label::new(Some(&format!("{initial_b}%")));
    bright_pct_label.add_css_class("qs-slider-pct");
    bright_card.append(&bright_pct_label);

    main_page.append(&bright_card);
    stack.add_named(&main_page, Some("main"));

    // =========================================================================
    // PAGE 2: "wifi" (Wi-Fi Networks Detail View)
    // =========================================================================
    let wifi_page = Box::new(Orientation::Vertical, 8);
    wifi_page.add_css_class("qs-page");
    wifi_page.add_css_class("qs-detail-page");

    // Detail nav header
    let wifi_nav = Box::new(Orientation::Horizontal, 10);
    wifi_nav.add_css_class("qs-nav-bar");
    wifi_nav.set_valign(Align::Center);

    let wifi_back_btn = Button::new();
    wifi_back_btn.set_label("󰁍");
    wifi_back_btn.add_css_class("qs-back-btn");
    wifi_back_btn.set_cursor_from_name(Some("pointer"));
    wifi_nav.append(&wifi_back_btn);

    let wifi_nav_title = Label::new(Some("Wi-Fi Networks"));
    wifi_nav_title.add_css_class("qs-nav-title");
    wifi_nav_title.set_halign(Align::Start);
    wifi_nav_title.set_hexpand(true);
    wifi_nav.append(&wifi_nav_title);

    let wifi_rescan_btn = Button::new();
    wifi_rescan_btn.set_label("󰑐");
    wifi_rescan_btn.add_css_class("qs-rescan-btn");
    wifi_rescan_btn.set_cursor_from_name(Some("pointer"));
    wifi_rescan_btn.set_tooltip_text(Some("Scan for nearby networks"));
    wifi_nav.append(&wifi_rescan_btn);

    let wifi_switch = Switch::new();
    wifi_switch.add_css_class("qs-switch");
    wifi_switch.set_active(network::wifi_enabled());
    wifi_switch.set_valign(Align::Center);
    wifi_nav.append(&wifi_switch);

    wifi_page.append(&wifi_nav);

    let wifi_status_label = Label::new(None);
    wifi_status_label.add_css_class("qs-status-banner");
    wifi_status_label.set_visible(false);
    wifi_status_label.set_halign(Align::Center);
    wifi_page.append(&wifi_status_label);

    let wifi_list_box = Box::new(Orientation::Vertical, 4);
    wifi_list_box.add_css_class("qs-list-box");
    wifi_list_box.set_halign(Align::Fill);
    wifi_list_box.set_vexpand(true);

    let wifi_scroll = ScrolledWindow::builder()
        .child(&wifi_list_box)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .build();
    wifi_scroll.set_min_content_height(340);
    wifi_scroll.set_max_content_height(480);
    wifi_scroll.set_vexpand(true);
    wifi_scroll.add_css_class("qs-scrolled-window");
    wifi_page.append(&wifi_scroll);

    stack.add_named(&wifi_page, Some("wifi"));

    // =========================================================================
    // PAGE 3: "wifi-auth" (Password Overlay)
    // =========================================================================
    let auth_page = Box::new(Orientation::Vertical, 14);
    auth_page.add_css_class("qs-page");
    auth_page.add_css_class("qs-auth-page");
    auth_page.set_vexpand(true);

    let auth_nav = Box::new(Orientation::Horizontal, 10);
    auth_nav.add_css_class("qs-nav-bar");
    auth_nav.set_valign(Align::Center);

    let auth_back_btn = Button::new();
    auth_back_btn.set_label("󰁍");
    auth_back_btn.add_css_class("qs-back-btn");
    auth_back_btn.set_cursor_from_name(Some("pointer"));
    auth_nav.append(&auth_back_btn);

    let auth_nav_title = Label::new(Some("Join Network"));
    auth_nav_title.add_css_class("qs-nav-title");
    auth_nav_title.set_halign(Align::Start);
    auth_nav_title.set_hexpand(true);
    auth_nav.append(&auth_nav_title);

    auth_page.append(&auth_nav);

    let auth_hero = Box::new(Orientation::Vertical, 8);
    auth_hero.set_halign(Align::Center);
    auth_hero.set_valign(Align::Center);
    auth_hero.set_margin_top(16);
    auth_hero.set_margin_bottom(12);

    let auth_icon = Label::new(None);
    auth_icon.set_use_markup(true);
    auth_icon.set_markup("<span font=\"36\" color=\"#a4d1b4\">󰤨</span>");
    auth_icon.add_css_class("qs-auth-icon");
    auth_hero.append(&auth_icon);

    let auth_ssid_label = Label::new(None);
    auth_ssid_label.add_css_class("qs-auth-ssid");
    auth_ssid_label.set_ellipsize(EllipsizeMode::End);
    auth_hero.append(&auth_ssid_label);

    let auth_sub = Label::new(Some("Enter network password to connect"));
    auth_sub.add_css_class("qs-auth-sub");
    auth_hero.append(&auth_sub);
    auth_page.append(&auth_hero);

    let auth_card = Box::new(Orientation::Vertical, 6);
    auth_card.add_css_class("qs-auth-input-card");

    let auth_input_lbl = Label::new(Some("PASSWORD"));
    auth_input_lbl.add_css_class("qs-auth-input-label");
    auth_input_lbl.set_halign(Align::Start);
    auth_card.append(&auth_input_lbl);

    let auth_pass_entry = PasswordEntry::new();
    auth_pass_entry.set_placeholder_text(Some("Enter password..."));
    auth_pass_entry.set_show_peek_icon(true);
    auth_pass_entry.add_css_class("qs-auth-entry");
    auth_card.append(&auth_pass_entry);
    auth_page.append(&auth_card);

    let auth_status = Label::new(None);
    auth_status.add_css_class("qs-auth-status");
    auth_status.set_visible(false);
    auth_status.set_halign(Align::Center);
    auth_page.append(&auth_status);

    let auth_actions = Box::new(Orientation::Horizontal, 10);
    auth_actions.add_css_class("qs-auth-actions");
    auth_actions.set_halign(Align::Fill);
    auth_actions.set_margin_top(12);

    let auth_cancel_btn = Button::new();
    auth_cancel_btn.set_label("Cancel");
    auth_cancel_btn.add_css_class("qs-auth-cancel-btn");
    auth_cancel_btn.set_hexpand(true);
    auth_cancel_btn.set_cursor_from_name(Some("pointer"));
    auth_actions.append(&auth_cancel_btn);

    let auth_connect_btn = Button::new();
    auth_connect_btn.set_label("Connect");
    auth_connect_btn.add_css_class("qs-auth-connect-btn");
    auth_connect_btn.set_hexpand(true);
    auth_connect_btn.set_cursor_from_name(Some("pointer"));
    auth_actions.append(&auth_connect_btn);
    auth_page.append(&auth_actions);

    stack.add_named(&auth_page, Some("wifi-auth"));

    // =========================================================================
    // PAGE 4: "bt" (Bluetooth Devices Detail View)
    // =========================================================================
    let bt_page = Box::new(Orientation::Vertical, 8);
    bt_page.add_css_class("qs-page");
    bt_page.add_css_class("qs-detail-page");

    let bt_nav = Box::new(Orientation::Horizontal, 10);
    bt_nav.add_css_class("qs-nav-bar");
    bt_nav.set_valign(Align::Center);

    let bt_back_btn = Button::new();
    bt_back_btn.set_label("󰁍");
    bt_back_btn.add_css_class("qs-back-btn");
    bt_back_btn.set_cursor_from_name(Some("pointer"));
    bt_nav.append(&bt_back_btn);

    let bt_nav_title = Label::new(Some("Bluetooth Devices"));
    bt_nav_title.add_css_class("qs-nav-title");
    bt_nav_title.set_halign(Align::Start);
    bt_nav_title.set_hexpand(true);
    bt_nav.append(&bt_nav_title);

    let bt_rescan_btn = Button::new();
    bt_rescan_btn.set_label("󰑐");
    bt_rescan_btn.add_css_class("qs-rescan-btn");
    bt_rescan_btn.set_cursor_from_name(Some("pointer"));
    bt_rescan_btn.set_tooltip_text(Some("Scan for nearby Bluetooth devices"));
    bt_nav.append(&bt_rescan_btn);

    let bt_switch = Switch::new();
    bt_switch.add_css_class("qs-switch");
    bt_switch.set_active(bluetooth::is_enabled());
    bt_switch.set_valign(Align::Center);
    bt_nav.append(&bt_switch);

    bt_page.append(&bt_nav);

    let bt_status_label = Label::new(None);
    bt_status_label.add_css_class("qs-status-banner");
    bt_status_label.set_visible(false);
    bt_status_label.set_halign(Align::Center);
    bt_page.append(&bt_status_label);

    let bt_list_box = Box::new(Orientation::Vertical, 4);
    bt_list_box.add_css_class("qs-list-box");
    bt_list_box.set_halign(Align::Fill);
    bt_list_box.set_vexpand(true);

    let bt_scroll = ScrolledWindow::builder()
        .child(&bt_list_box)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .build();
    bt_scroll.set_min_content_height(340);
    bt_scroll.set_max_content_height(480);
    bt_scroll.set_vexpand(true);
    bt_scroll.add_css_class("qs-scrolled-window");
    bt_page.append(&bt_scroll);

    stack.add_named(&bt_page, Some("bt"));

    dropdown.append(&stack);
    qs_window.set_child(Some(&dropdown));
    qs_window.hide();

    // =========================================================================
    // STATE & NAVIGATION WIRING
    // =========================================================================
    let target_wifi_ssid = Rc::new(RefCell::new(String::new()));

    // Overview tile clicks -> switch page
    {
        let stack_wifi = stack.clone();
        wifi_text_btn.connect_clicked(move |_| {
            stack_wifi.set_visible_child_name("wifi");
        });
        let stack_wifi2 = stack.clone();
        wifi_arrow_btn.connect_clicked(move |_| {
            stack_wifi2.set_visible_child_name("wifi");
        });
    }
    {
        let stack_bt = stack.clone();
        bt_text_btn.connect_clicked(move |_| {
            stack_bt.set_visible_child_name("bt");
        });
        let stack_bt2 = stack.clone();
        bt_arrow_btn.connect_clicked(move |_| {
            stack_bt2.set_visible_child_name("bt");
        });
    }

    // Detail Back buttons -> return to "main"
    {
        let stack_back = stack.clone();
        wifi_back_btn.connect_clicked(move |_| {
            stack_back.set_visible_child_name("main");
        });
    }
    {
        let stack_back = stack.clone();
        bt_back_btn.connect_clicked(move |_| {
            stack_back.set_visible_child_name("main");
        });
    }
    {
        let stack_back = stack.clone();
        let auth_status_cl = auth_status.clone();
        let auth_pass_cl = auth_pass_entry.clone();
        auth_back_btn.connect_clicked(move |_| {
            auth_status_cl.set_visible(false);
            auth_pass_cl.set_text("");
            stack_back.set_visible_child_name("wifi");
        });

        let stack_back2 = stack.clone();
        let auth_status_cl2 = auth_status.clone();
        let auth_pass_cl2 = auth_pass_entry.clone();
        auth_cancel_btn.connect_clicked(move |_| {
            auth_status_cl2.set_visible(false);
            auth_pass_cl2.set_text("");
            stack_back2.set_visible_child_name("wifi");
        });
    }

    // Shared labels handle for bar pill
    let pill_labels = QuickSettingsLabels {
        net_icon: net_icon.clone(),
        bt_icon: bt_icon.clone(),
        audio_label: audio_label.clone(),
        bat_icon: bat_icon.clone(),
    };

    // --- Overview Toggles & Slider Logic ---
    // Wi-Fi Quick Toggle
    {
        let pill_net = net_icon.clone();
        let wifi_sub = wifi_tile_sub.clone();
        let wifi_icon = wifi_tile_icon.clone();
        let wifi_sw = wifi_switch.clone();
        let wifi_tile_cl = wifi_tile.clone();

        wifi_toggle_btn.connect_clicked(move |_| {
            let next = !network::wifi_enabled();
            network::set_wifi_enabled(next);
            wifi_sw.set_active(next);
            update_overview_wifi(&pill_net, &wifi_sub, &wifi_icon, &wifi_tile_cl);
        });
    }

    // Bluetooth Quick Toggle
    {
        let pill_bt = bt_icon.clone();
        let bt_sub = bt_tile_sub.clone();
        let bt_icon_lbl = bt_tile_icon.clone();
        let bt_sw = bt_switch.clone();
        let bt_tile_cl = bt_tile.clone();

        bt_toggle_btn.connect_clicked(move |_| {
            let next = !bluetooth::is_enabled();
            let _ = bluetooth::set_enabled(next);
            bt_sw.set_active(next);
            update_overview_bt(&pill_bt, &bt_sub, &bt_icon_lbl, &bt_tile_cl);
        });
    }

    // Volume Slider & Mute
    {
        let pill_aud = audio_label.clone();
        let mute_ic = mute_icon.clone();
        let mute_btn_cl = mute_btn.clone();
        let vol_pct = volume_pct_label.clone();
        let vol_scale = volume_scale.clone();

        volume_scale.connect_value_changed(move |scale| {
            let val = scale.value().round() as u8;
            vol_pct.set_text(&format!("{val}%"));
            audio::set_volume(val);
            update_overview_audio(&pill_aud, &mute_ic, &mute_btn_cl, &vol_pct, &vol_scale, false);
        });
    }
    {
        let pill_aud = audio_label.clone();
        let mute_ic = mute_icon.clone();
        let mute_btn_cl = mute_btn.clone();
        let vol_pct = volume_pct_label.clone();
        let vol_scale = volume_scale.clone();

        mute_btn.connect_clicked(move |_| {
            audio::toggle_mute();
            update_overview_audio(&pill_aud, &mute_ic, &mute_btn_cl, &vol_pct, &vol_scale, true);
        });
    }

    // --- Brightness Slider Signal Wiring ---
    {
        let b_icon = bright_icon_lbl.clone();
        let b_pct = bright_pct_label.clone();
        bright_scale.connect_value_changed(move |scale| {
            let val = scale.value().round() as u8;
            crate::services::brightness::set_brightness(val);
            b_pct.set_text(&format!("{val}%"));
            b_icon.set_text(crate::services::brightness::icon(val));
        });
    }

    // --- Wi-Fi Detail Logic & Refresh ---
    let reload_wifi = {
        let list_box = wifi_list_box.clone();
        let status_lbl = wifi_status_label.clone();
        let pill_net = net_icon.clone();
        let wifi_sub = wifi_tile_sub.clone();
        let wifi_icon = wifi_tile_icon.clone();
        let wifi_tile_cl = wifi_tile.clone();
        let wifi_sw = wifi_switch.clone();
        let stack = stack.clone();
        let auth_pass = auth_pass_entry.clone();
        let auth_stat = auth_status.clone();
        let target_ssid = target_wifi_ssid.clone();

        Rc::new(move || {
            let is_enabled = network::wifi_enabled();
            wifi_sw.set_active(is_enabled);
            update_overview_wifi(&pill_net, &wifi_sub, &wifi_icon, &wifi_tile_cl);

            if !is_enabled {
                clear_children(&list_box);
                let off_box = Box::new(Orientation::Vertical, 10);
                off_box.add_css_class("qs-empty");
                off_box.set_halign(Align::Center);
                off_box.set_valign(Align::Center);
                off_box.set_vexpand(true);
                off_box.set_margin_top(60);

                let off_icon = Label::new(Some("󰤮"));
                off_icon.add_css_class("qs-empty-icon");
                let off_text = Label::new(Some("Wi-Fi is turned off"));
                off_text.add_css_class("qs-empty-text");
                let off_sub = Label::new(Some("Toggle switch to enable wireless"));
                off_sub.add_css_class("qs-empty-sub");

                off_box.append(&off_icon);
                off_box.append(&off_text);
                off_box.append(&off_sub);
                list_box.append(&off_box);
                return;
            }

            if list_box.observe_children().n_items() == 0 {
                let scanning = Label::new(Some("Scanning for networks..."));
                scanning.add_css_class("qs-scanning");
                scanning.set_margin_top(40);
                list_box.append(&scanning);
            }

            let (tx, rx) = mpsc::channel::<Vec<WifiNetwork>>();
            std::thread::spawn(move || {
                let nets = network::scan_wifi();
                let _ = tx.send(nets);
            });

            let list_box_cb = list_box.clone();
            let status_lbl_cb = status_lbl.clone();
            let pill_net_cb = pill_net.clone();
            let wifi_sub_cb = wifi_sub.clone();
            let wifi_icon_cb = wifi_icon.clone();
            let wifi_tile_cb = wifi_tile_cl.clone();
            let stack_cb = stack.clone();
            let auth_pass_cb = auth_pass.clone();
            let auth_stat_cb = auth_stat.clone();
            let target_ssid_cb = target_ssid.clone();

            glib::timeout_add_local(Duration::from_millis(50), move || {
                match rx.try_recv() {
                    Ok(nets) => {
                        update_overview_wifi(&pill_net_cb, &wifi_sub_cb, &wifi_icon_cb, &wifi_tile_cb);
                        populate_wifi_list(
                            &list_box_cb,
                            &nets,
                            &status_lbl_cb,
                            &pill_net_cb,
                            &wifi_sub_cb,
                            &wifi_icon_cb,
                            &wifi_tile_cb,
                            &stack_cb,
                            &auth_pass_cb,
                            &auth_stat_cb,
                            &target_ssid_cb,
                        );
                        glib::ControlFlow::Break
                    }
                    Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                    Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                }
            });
        })
    };

    // Wi-Fi Detail Rescan & Switch
    {
        let reload = reload_wifi.clone();
        wifi_rescan_btn.connect_clicked(move |_| {
            network::rescan_wifi();
            reload();
        });
    }
    {
        let reload = reload_wifi.clone();
        wifi_switch.connect_state_set(move |_, state| {
            network::set_wifi_enabled(state);
            reload();
            glib::Propagation::Proceed
        });
    }

    // --- Bluetooth Detail Logic & Refresh ---
    let reload_bt = {
        let list_box = bt_list_box.clone();
        let status_lbl = bt_status_label.clone();
        let pill_bt = bt_icon.clone();
        let bt_sub = bt_tile_sub.clone();
        let bt_icon_lbl = bt_tile_icon.clone();
        let bt_tile_cl = bt_tile.clone();
        let bt_sw = bt_switch.clone();

        Rc::new(move || {
            let is_enabled = bluetooth::is_enabled();
            bt_sw.set_active(is_enabled);
            update_overview_bt(&pill_bt, &bt_sub, &bt_icon_lbl, &bt_tile_cl);

            if !is_enabled {
                clear_children(&list_box);
                let off_box = Box::new(Orientation::Vertical, 10);
                off_box.add_css_class("qs-empty");
                off_box.set_halign(Align::Center);
                off_box.set_valign(Align::Center);
                off_box.set_vexpand(true);
                off_box.set_margin_top(60);

                let off_icon = Label::new(Some("󰂲"));
                off_icon.add_css_class("qs-empty-icon");
                let off_text = Label::new(Some("Bluetooth is turned off"));
                off_text.add_css_class("qs-empty-text");
                let off_sub = Label::new(Some("Toggle switch to enable Bluetooth"));
                off_sub.add_css_class("qs-empty-sub");

                off_box.append(&off_icon);
                off_box.append(&off_text);
                off_box.append(&off_sub);
                list_box.append(&off_box);
                return;
            }

            let (tx, rx) = mpsc::channel::<Vec<BluetoothDevice>>();
            std::thread::spawn(move || {
                let devs = bluetooth::get_devices();
                let _ = tx.send(devs);
            });

            let list_box_cb = list_box.clone();
            let status_lbl_cb = status_lbl.clone();
            let pill_bt_cb = pill_bt.clone();
            let bt_sub_cb = bt_sub.clone();
            let bt_icon_cb = bt_icon_lbl.clone();
            let bt_tile_cb = bt_tile_cl.clone();

            glib::timeout_add_local(Duration::from_millis(50), move || {
                match rx.try_recv() {
                    Ok(devs) => {
                        update_overview_bt(&pill_bt_cb, &bt_sub_cb, &bt_icon_cb, &bt_tile_cb);
                        populate_bt_list(
                            &list_box_cb,
                            &devs,
                            &status_lbl_cb,
                            &pill_bt_cb,
                            &bt_sub_cb,
                            &bt_icon_cb,
                            &bt_tile_cb,
                        );
                        glib::ControlFlow::Break
                    }
                    Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                    Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                }
            });
        })
    };

    // Bluetooth Detail Rescan & Switch
    {
        let reload = reload_bt.clone();
        let status_lbl = bt_status_label.clone();
        bt_rescan_btn.connect_clicked(move |_| {
            status_lbl.set_text("Scanning for devices...");
            status_lbl.set_visible(true);
            bluetooth::scan_on();
            reload();
        });
    }
    {
        let reload = reload_bt.clone();
        bt_switch.connect_state_set(move |_, state| {
            let _ = bluetooth::set_enabled(state);
            reload();
            glib::Propagation::Proceed
        });
    }

    // --- Wi-Fi Auth Connect Handler ---
    {
        let target_ssid_cl = target_wifi_ssid.clone();
        let auth_pass_cl = auth_pass_entry.clone();
        let auth_status_cl = auth_status.clone();
        let stack_cl = stack.clone();
        let reload_w = reload_wifi.clone();

        let handle_auth_connect = Rc::new(move || {
            let ssid = target_ssid_cl.borrow().clone();
            let password = auth_pass_cl.text().to_string();
            if ssid.is_empty() {
                return;
            }

            auth_status_cl.set_text(&format!("Connecting to {ssid}..."));
            auth_status_cl.remove_css_class("error");
            auth_status_cl.add_css_class("connecting");
            auth_status_cl.set_visible(true);

            let (tx, rx) = mpsc::channel();
            let ssid_thread = ssid.clone();
            std::thread::spawn(move || {
                let res = network::connect_wifi(&ssid_thread, Some(&password));
                let _ = tx.send(res);
            });

            let auth_status_cb = auth_status_cl.clone();
            let stack_cb = stack_cl.clone();
            let auth_pass_cb = auth_pass_cl.clone();
            let reload_cb = reload_w.clone();

            glib::timeout_add_local(Duration::from_millis(50), move || {
                match rx.try_recv() {
                    Ok(res) => {
                        match res {
                            Ok(_) => {
                                auth_status_cb.set_visible(false);
                                auth_pass_cb.set_text("");
                                stack_cb.set_visible_child_name("wifi");
                                reload_cb();
                            }
                            Err(e) => {
                                auth_status_cb.set_text(&format!("Failed: {e}"));
                                auth_status_cb.remove_css_class("connecting");
                                auth_status_cb.add_css_class("error");
                                auth_status_cb.set_visible(true);
                            }
                        }
                        glib::ControlFlow::Break
                    }
                    Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                    Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                }
            });
        });

        let conn_act = handle_auth_connect.clone();
        auth_connect_btn.connect_clicked(move |_| {
            conn_act();
        });
        let conn_ent = handle_auth_connect.clone();
        auth_pass_entry.connect_activate(move |_| {
            conn_ent();
        });
    }

    // --- Hover & Click Show/Hide Handling ---
    let hide_source = Rc::new(Cell::new(None::<glib::SourceId>));
    let reload_all = {
        let reload_w = reload_wifi.clone();
        let reload_b = reload_bt.clone();
        let pill_aud = audio_label.clone();
        let mute_ic = mute_icon.clone();
        let mute_btn_cl = mute_btn.clone();
        let vol_pct = volume_pct_label.clone();
        let vol_scale = volume_scale.clone();
        let pill_bat = bat_icon.clone();
        let header_bat = header_battery.clone();
        let b_scale = bright_scale.clone();
        let b_pct = bright_pct_label.clone();
        let b_icon = bright_icon_lbl.clone();
        Rc::new(move || {
            reload_w();
            reload_b();
            update_overview_audio(&pill_aud, &mute_ic, &mute_btn_cl, &vol_pct, &vol_scale, true);
            update_overview_battery(&pill_bat, &header_bat);
            if let Some(cur_b) = crate::services::brightness::query() {
                b_scale.set_value(cur_b as f64);
                b_pct.set_text(&format!("{cur_b}%"));
                b_icon.set_text(crate::services::brightness::icon(cur_b));
            }
        })
    };

    let button_motion = EventControllerMotion::new();
    button_motion.connect_enter({
        let hide_source = hide_source.clone();
        let qs_win = qs_window.clone();
        let reload = reload_all.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
            reload();
            qs_win.present();
        }
    });

    button_motion.connect_leave({
        let hide_source = hide_source.clone();
        let qs_win = qs_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let qs_win = qs_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                qs_win.hide();
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    button.add_controller(button_motion);

    let dropdown_motion = EventControllerMotion::new();
    dropdown_motion.connect_enter({
        let hide_source = hide_source.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
        }
    });

    dropdown_motion.connect_leave({
        let hide_source = hide_source.clone();
        let qs_win = qs_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let qs_win = qs_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                qs_win.hide();
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    dropdown.add_controller(dropdown_motion);

    let qs_win_click = qs_window.clone();
    let reload_click = reload_all.clone();
    button.connect_clicked(move |_| {
        if qs_win_click.is_visible() {
            qs_win_click.hide();
        } else {
            reload_click();
            qs_win_click.present();
        }
    });

    // Initial render
    reload_all();

    (button, pill_labels, qs_window, reload_all)
}

/// Toggle visibility of the Quick Settings Control Center window.
pub fn toggle(window: &ApplicationWindow, reload: &Rc<dyn Fn()>) {
    if window.is_visible() {
        window.hide();
    } else {
        reload();
        window.present();
    }
}

/// Update Wi-Fi overview tile and top bar pill icon.
fn update_overview_wifi(pill_label: &Label, sub_label: &Label, tile_icon: &Label, tile: &Box) {
    if !network::wifi_enabled() {
        pill_label.set_markup("<span color=\"#6e7870\">󰤮</span>");
        tile_icon.set_markup("<span color=\"#6e7870\">󰤮</span>");
        sub_label.set_text("Off");
        tile.remove_css_class("active");
        return;
    }

    match network::query() {
        NetworkState::Wifi { ref ssid } => {
            pill_label.set_markup("<span color=\"#a4d1b4\">󰤨</span>");
            tile_icon.set_markup("<span color=\"#0b0f0c\">󰤨</span>");
            sub_label.set_text(ssid);
            tile.add_css_class("active");
        }
        _ => {
            pill_label.set_markup("<span color=\"#dee8df\">󰤨</span>");
            tile_icon.set_markup("<span color=\"#a4d1b4\">󰤨</span>");
            sub_label.set_text("Not Connected");
            tile.remove_css_class("active");
        }
    }
}

/// Update Bluetooth overview tile and top bar pill icon.
fn update_overview_bt(pill_label: &Label, sub_label: &Label, tile_icon: &Label, tile: &Box) {
    if !bluetooth::is_enabled() {
        pill_label.set_markup("<span color=\"#6e7870\">󰂲</span>");
        tile_icon.set_markup("<span color=\"#6e7870\">󰂲</span>");
        sub_label.set_text("Off");
        tile.remove_css_class("active");
        return;
    }

    match bluetooth::query() {
        BluetoothState::Connected { ref name, .. } => {
            pill_label.set_markup("<span color=\"#a4d1b4\">󰂯</span>");
            tile_icon.set_markup("<span color=\"#0b0f0c\">󰂯</span>");
            sub_label.set_text(name);
            tile.add_css_class("active");
        }
        BluetoothState::Enabled { paired_count } => {
            pill_label.set_markup("<span color=\"#dee8df\">󰂯</span>");
            tile_icon.set_markup("<span color=\"#a4d1b4\">󰂯</span>");
            if paired_count > 0 {
                sub_label.set_text(&format!("{paired_count} paired"));
            } else {
                sub_label.set_text("On");
            }
            tile.remove_css_class("active");
        }
        _ => {
            pill_label.set_markup("<span color=\"#6e7870\">󰂲</span>");
            tile_icon.set_markup("<span color=\"#6e7870\">󰂲</span>");
            sub_label.set_text("Off");
            tile.remove_css_class("active");
        }
    }
}

/// Update Audio slider, percentage, mute button, and top bar pill.
fn update_overview_audio(
    pill_label: &Label,
    mute_icon: &Label,
    mute_btn: &Button,
    pct_label: &Label,
    scale: &Scale,
    sync_scale: bool,
) {
    if let Some(state) = audio::query() {
        if sync_scale {
            scale.set_value(state.volume_percent as f64);
        }
        pct_label.set_text(&format!("{}%", state.volume_percent));

        if state.muted {
            pill_label.set_markup("<span color=\"#fa746f\">󰝟</span>");
            mute_icon.set_markup("<span color=\"#fa746f\">󰝟</span>");
            mute_btn.add_css_class("muted");
        } else {
            let icon_str = match state.volume_percent {
                0..=30 => "󰕿",
                31..=65 => "󰖀",
                _ => "󰕾",
            };
            pill_label.set_markup(&format!("<span color=\"#86dcce\">{icon_str} {}%</span>", state.volume_percent));
            mute_icon.set_markup(&format!("<span color=\"#a4d1b4\">{icon_str}</span>"));
            mute_btn.remove_css_class("muted");
        }
    }
}

/// Refresh all components for incoming external background events.
pub fn refresh_network(labels: &QuickSettingsLabels) {
    if !network::wifi_enabled() {
        labels.net_icon.set_markup("<span color=\"#6e7870\">󰤮</span>");
    } else if let NetworkState::Wifi { .. } = network::query() {
        labels.net_icon.set_markup("<span color=\"#a4d1b4\">󰤨</span>");
    } else {
        labels.net_icon.set_markup("<span color=\"#dee8df\">󰤨</span>");
    }
}

pub fn refresh_bluetooth(labels: &QuickSettingsLabels) {
    if !bluetooth::is_enabled() {
        labels.bt_icon.set_markup("<span color=\"#6e7870\">󰂲</span>");
    } else if let BluetoothState::Connected { .. } = bluetooth::query() {
        labels.bt_icon.set_markup("<span color=\"#a4d1b4\">󰂯</span>");
    } else {
        labels.bt_icon.set_markup("<span color=\"#dee8df\">󰂯</span>");
    }
}

pub fn refresh_audio(labels: &QuickSettingsLabels) {
    if let Some(state) = audio::query() {
        if state.muted {
            labels.audio_label.set_markup("<span color=\"#fa746f\">󰝟</span>");
        } else {
            let icon_str = match state.volume_percent {
                0..=30 => "󰕿",
                31..=65 => "󰖀",
                _ => "󰕾",
            };
            labels.audio_label.set_markup(&format!("<span color=\"#86dcce\">{icon_str} {}%</span>", state.volume_percent));
        }
    }
}

pub fn refresh_battery(labels: &QuickSettingsLabels) {
    if let Some((percent, charging)) = sysinfo::battery_state() {
        let icon = if charging {
            "󰂄"
        } else {
            sysinfo::battery_icon(percent)
        };
        let color = if charging {
            "#a4d1b4"
        } else if percent <= 15 {
            "#fa746f"
        } else if percent <= 30 {
            "#cec06b"
        } else {
            "#a3f1bd"
        };
        labels.bat_icon.set_markup(&format!("<span color=\"{color}\">{icon} {percent}%</span>"));
        let status = if charging { "Charging" } else { "Discharging" };
        labels.bat_icon.set_tooltip_text(Some(&format!("Battery: {percent}% ({status})")));
    } else {
        labels.bat_icon.set_markup("");
        labels.bat_icon.set_tooltip_text(None);
    }
}

fn update_overview_battery(pill_bat: &Label, header_bat: &Label) {
    if let Some((percent, charging)) = sysinfo::battery_state() {
        let icon = if charging {
            "󰂄"
        } else {
            sysinfo::battery_icon(percent)
        };
        let color = if charging {
            "#a4d1b4"
        } else if percent <= 15 {
            "#fa746f"
        } else if percent <= 30 {
            "#cec06b"
        } else {
            "#a3f1bd"
        };
        pill_bat.set_markup(&format!("<span color=\"{color}\">{icon} {percent}%</span>"));
        let status = if charging { "Charging" } else { "Discharging" };
        pill_bat.set_tooltip_text(Some(&format!("Battery: {percent}% ({status})")));

        if charging {
            header_bat.add_css_class("charging");
            header_bat.set_markup(&format!("<span color=\"#a4d1b4\">󰂄</span> {percent}%"));
        } else {
            header_bat.remove_css_class("charging");
            header_bat.set_markup(&format!("<span color=\"{color}\">{icon}</span> {percent}%"));
        }
        header_bat.set_tooltip_text(Some(&format!("Battery: {percent}% ({status})")));
        header_bat.set_visible(true);
    } else {
        pill_bat.set_markup("");
        pill_bat.set_tooltip_text(None);
        header_bat.set_visible(false);
    }
}

/// Populates `list_box` with flat Caelestia/M3 Wi-Fi items.
fn populate_wifi_list(
    list_box: &Box,
    networks: &[WifiNetwork],
    status_label: &Label,
    pill_label: &Label,
    wifi_sub: &Label,
    wifi_icon: &Label,
    wifi_tile: &Box,
    stack: &Stack,
    auth_pass_entry: &PasswordEntry,
    auth_status: &Label,
    target_ssid: &Rc<RefCell<String>>,
) {
    clear_children(list_box);

    if networks.is_empty() {
        let empty_box = Box::new(Orientation::Vertical, 10);
        empty_box.add_css_class("qs-empty");
        empty_box.set_halign(Align::Center);
        empty_box.set_valign(Align::Center);
        empty_box.set_vexpand(true);
        empty_box.set_margin_top(40);

        let empty_icon = Label::new(Some("󰤭"));
        empty_icon.add_css_class("qs-empty-icon");
        let empty_text = Label::new(Some("No networks found"));
        empty_text.add_css_class("qs-empty-text");
        let empty_sub = Label::new(Some("Click 󰑐 above to rescan"));
        empty_sub.add_css_class("qs-empty-sub");

        empty_box.append(&empty_icon);
        empty_box.append(&empty_text);
        empty_box.append(&empty_sub);
        list_box.append(&empty_box);
        return;
    }

    for net in networks {
        let item_card = Box::new(Orientation::Horizontal, 12);
        item_card.add_css_class("qs-item");
        item_card.set_valign(Align::Center);
        if net.is_connected {
            item_card.add_css_class("connected");
        }

        let icon_chip = Box::new(Orientation::Horizontal, 0);
        icon_chip.add_css_class("qs-icon-chip");
        icon_chip.set_valign(Align::Center);
        icon_chip.set_halign(Align::Center);

        let (icon_str, icon_color) = wifi_signal_info(net.signal);
        let icon_label = Label::new(None);
        icon_label.set_use_markup(true);
        icon_label.set_markup(&format!("<span color=\"{icon_color}\">{icon_str}</span>"));
        icon_label.add_css_class("qs-item-icon");
        icon_chip.append(&icon_label);
        item_card.append(&icon_chip);

        let info_box = Box::new(Orientation::Vertical, 2);
        info_box.set_halign(Align::Start);
        info_box.set_hexpand(true);

        let name_label = Label::new(Some(&net.ssid));
        name_label.add_css_class("qs-item-name");
        name_label.set_halign(Align::Start);
        name_label.set_ellipsize(EllipsizeMode::End);
        info_box.append(&name_label);

        let meta_row = Box::new(Orientation::Horizontal, 6);
        meta_row.set_valign(Align::Center);

        if net.is_connected {
            let conn_icon = Label::new(Some("󰄬"));
            conn_icon.add_css_class("qs-connected-icon");
            conn_icon.set_tooltip_text(Some("Active Connection"));
            meta_row.append(&conn_icon);
        } else if net.is_saved {
            let saved_icon = Label::new(Some("󰋑"));
            saved_icon.add_css_class("qs-saved-icon");
            saved_icon.set_tooltip_text(Some("Saved Network"));
            meta_row.append(&saved_icon);
        }

        let sig_label = Label::new(Some(&format!("{}%", net.signal)));
        sig_label.add_css_class("qs-item-signal");
        meta_row.append(&sig_label);

        let is_secured = !net.security.is_empty() && !net.security.contains("--");
        if is_secured {
            let lock = Label::new(Some("󰌾"));
            lock.add_css_class("qs-lock-icon");
            lock.set_tooltip_text(Some(&format!("Security: {}", net.security)));
            meta_row.append(&lock);
        }

        info_box.append(&meta_row);
        item_card.append(&info_box);

        let action_btn = Button::new();
        action_btn.set_cursor_from_name(Some("pointer"));
        action_btn.set_valign(Align::Center);

        let ssid = net.ssid.clone();
        let status_lbl = status_label.clone();
        let pill_lbl = pill_label.clone();
        let w_sub = wifi_sub.clone();
        let w_icon = wifi_icon.clone();
        let w_tile = wifi_tile.clone();
        let list_b = list_box.clone();
        let stack_cl = stack.clone();
        let auth_pass_cl = auth_pass_entry.clone();
        let auth_status_cl = auth_status.clone();
        let target_ssid_cl = target_ssid.clone();

        if net.is_connected {
            action_btn.set_label("󰚥");
            action_btn.add_css_class("qs-disconnect-btn");
            action_btn.set_tooltip_text(Some("Disconnect"));

            action_btn.connect_clicked(move |_| {
                status_lbl.set_text(&format!("Disconnecting from {ssid}..."));
                status_lbl.set_visible(true);
                let ssid_cl = ssid.clone();
                let (tx, rx) = mpsc::channel();

                std::thread::spawn(move || {
                    let res = network::disconnect_wifi(&ssid_cl);
                    let _ = tx.send(res);
                });

                let status_lbl_cb = status_lbl.clone();
                let pill_lbl_cb = pill_lbl.clone();
                let w_sub_cb = w_sub.clone();
                let w_icon_cb = w_icon.clone();
                let w_tile_cb = w_tile.clone();
                let list_b_cb = list_b.clone();
                let stack_cb = stack_cl.clone();
                let auth_pass_cb = auth_pass_cl.clone();
                let auth_status_cb = auth_status_cl.clone();
                let target_ssid_cb = target_ssid_cl.clone();

                glib::timeout_add_local(Duration::from_millis(50), move || {
                    match rx.try_recv() {
                        Ok(res) => {
                            match res {
                                Ok(_) => {
                                    status_lbl_cb.set_visible(false);
                                    update_overview_wifi(&pill_lbl_cb, &w_sub_cb, &w_icon_cb, &w_tile_cb);
                                    let nets = network::scan_wifi();
                                    populate_wifi_list(
                                        &list_b_cb,
                                        &nets,
                                        &status_lbl_cb,
                                        &pill_lbl_cb,
                                        &w_sub_cb,
                                        &w_icon_cb,
                                        &w_tile_cb,
                                        &stack_cb,
                                        &auth_pass_cb,
                                        &auth_status_cb,
                                        &target_ssid_cb,
                                    );
                                }
                                Err(e) => {
                                    status_lbl_cb.set_text(&format!("Error: {e}"));
                                }
                            }
                            glib::ControlFlow::Break
                        }
                        Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                        Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                    }
                });
            });
        } else {
            action_btn.set_label("󰅂");
            action_btn.add_css_class("qs-connect-btn");
            action_btn.set_tooltip_text(Some("Connect"));

            let is_saved = net.is_saved;

            action_btn.connect_clicked(move |_| {
                if is_saved || !is_secured {
                    status_lbl.set_text(&format!("Connecting to {ssid}..."));
                    status_lbl.set_visible(true);
                    let ssid_cl = ssid.clone();
                    let (tx, rx) = mpsc::channel();

                    std::thread::spawn(move || {
                        let res = network::connect_wifi(&ssid_cl, None);
                        let _ = tx.send(res);
                    });

                    let status_lbl_cb = status_lbl.clone();
                    let pill_lbl_cb = pill_lbl.clone();
                    let w_sub_cb = w_sub.clone();
                    let w_icon_cb = w_icon.clone();
                    let w_tile_cb = w_tile.clone();
                    let list_b_cb = list_b.clone();
                    let stack_cb = stack_cl.clone();
                    let auth_pass_cb = auth_pass_cl.clone();
                    let auth_status_cb = auth_status_cl.clone();
                    let target_ssid_cb = target_ssid_cl.clone();

                    glib::timeout_add_local(Duration::from_millis(50), move || {
                        match rx.try_recv() {
                            Ok(res) => {
                                match res {
                                    Ok(_) => {
                                        status_lbl_cb.set_visible(false);
                                        update_overview_wifi(&pill_lbl_cb, &w_sub_cb, &w_icon_cb, &w_tile_cb);
                                        let nets = network::scan_wifi();
                                        populate_wifi_list(
                                            &list_b_cb,
                                            &nets,
                                            &status_lbl_cb,
                                            &pill_lbl_cb,
                                            &w_sub_cb,
                                            &w_icon_cb,
                                            &w_tile_cb,
                                            &stack_cb,
                                            &auth_pass_cb,
                                            &auth_status_cb,
                                            &target_ssid_cb,
                                        );
                                    }
                                    Err(e) => {
                                        status_lbl_cb.set_text(&format!("Failed: {e}"));
                                    }
                                }
                                glib::ControlFlow::Break
                            }
                            Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                            Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                        }
                    });
                } else {
                    *target_ssid_cl.borrow_mut() = ssid.clone();
                    auth_pass_cl.set_text("");
                    auth_status_cl.set_visible(false);

                    if let Some(auth_view_w) = stack_cl.child_by_name("wifi-auth") {
                        if let Ok(auth_box) = auth_view_w.downcast::<Box>() {
                            let model = auth_box.observe_children();
                            for i in 0..model.n_items() {
                                if let Some(obj) = model.item(i) {
                                    if let Ok(b) = obj.downcast::<Box>() {
                                        let b_children = b.observe_children();
                                        for j in 0..b_children.n_items() {
                                            if let Some(child_obj) = b_children.item(j) {
                                                if let Ok(lbl) = child_obj.downcast::<Label>() {
                                                    if lbl.has_css_class("qs-auth-ssid") {
                                                        lbl.set_text(&ssid);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    stack_cl.set_visible_child_name("wifi-auth");
                    auth_pass_cl.grab_focus();
                }
            });
        }

        item_card.append(&action_btn);
        list_box.append(&item_card);
    }
}

/// Populates `list_box` with backgroundless Bluetooth device items.
fn populate_bt_list(
    list_box: &Box,
    devices: &[BluetoothDevice],
    status_label: &Label,
    pill_label: &Label,
    bt_sub: &Label,
    bt_icon: &Label,
    bt_tile: &Box,
) {
    clear_children(list_box);

    if devices.is_empty() {
        let empty_box = Box::new(Orientation::Vertical, 10);
        empty_box.add_css_class("qs-empty");
        empty_box.set_halign(Align::Center);
        empty_box.set_valign(Align::Center);
        empty_box.set_vexpand(true);
        empty_box.set_margin_top(40);

        let empty_icon = Label::new(Some("󰂲"));
        empty_icon.add_css_class("qs-empty-icon");
        let empty_text = Label::new(Some("No Bluetooth devices found"));
        empty_text.add_css_class("qs-empty-text");
        let empty_sub = Label::new(Some("Click 󰑐 above to scan"));
        empty_sub.add_css_class("qs-empty-sub");

        empty_box.append(&empty_icon);
        empty_box.append(&empty_text);
        empty_box.append(&empty_sub);
        list_box.append(&empty_box);
        return;
    }

    for dev in devices {
        let item_card = Box::new(Orientation::Horizontal, 12);
        item_card.add_css_class("qs-item");
        item_card.set_valign(Align::Center);
        if dev.is_connected {
            item_card.add_css_class("connected");
        }

        let icon_chip = Box::new(Orientation::Horizontal, 0);
        icon_chip.add_css_class("qs-icon-chip");
        icon_chip.set_valign(Align::Center);
        icon_chip.set_halign(Align::Center);

        let icon_str = match dev.icon_type.as_str() {
            "audio-headphones" | "audio-headset" | "audio-card" => "󰋋",
            "input-mouse" => "󰍽",
            "input-keyboard" => "󰌌",
            "input-gaming" => "󰊴",
            "phone" => "󰏲",
            _ => "󰂯",
        };

        let icon_color = if dev.is_connected {
            "#a4d1b4"
        } else if dev.is_paired {
            "#9cebcc"
        } else {
            "#6e7870"
        };

        let icon_label = Label::new(None);
        icon_label.set_use_markup(true);
        icon_label.set_markup(&format!("<span color=\"{icon_color}\">{icon_str}</span>"));
        icon_label.add_css_class("qs-item-icon");
        icon_chip.append(&icon_label);
        item_card.append(&icon_chip);

        let info_box = Box::new(Orientation::Vertical, 2);
        info_box.set_halign(Align::Start);
        info_box.set_hexpand(true);

        let name_label = Label::new(Some(&dev.name));
        name_label.add_css_class("qs-item-name");
        name_label.set_halign(Align::Start);
        name_label.set_ellipsize(EllipsizeMode::End);
        info_box.append(&name_label);

        let meta_row = Box::new(Orientation::Horizontal, 6);
        meta_row.set_valign(Align::Center);

        if dev.is_connected {
            let conn_icon = Label::new(Some("󰄬"));
            conn_icon.add_css_class("qs-connected-icon");
            conn_icon.set_tooltip_text(Some("Connected"));
            meta_row.append(&conn_icon);

            if let Some(bat) = dev.battery {
                let bat_label = Label::new(Some(&format!("{bat}%")));
                bat_label.add_css_class("qs-item-battery");
                meta_row.append(&bat_label);
            }
        } else if dev.is_paired {
            let paired_icon = Label::new(Some("󰋑"));
            paired_icon.add_css_class("qs-paired-icon");
            paired_icon.set_tooltip_text(Some("Paired Device"));
            meta_row.append(&paired_icon);
        }

        info_box.append(&meta_row);
        item_card.append(&info_box);

        let action_btn = Button::new();
        action_btn.set_cursor_from_name(Some("pointer"));
        action_btn.set_valign(Align::Center);

        let mac = dev.mac.clone();
        let name = dev.name.clone();
        let is_connected = dev.is_connected;
        let status_lbl = status_label.clone();
        let pill_lbl = pill_label.clone();
        let b_sub = bt_sub.clone();
        let b_icon = bt_icon.clone();
        let b_tile = bt_tile.clone();
        let list_b = list_box.clone();

        if is_connected {
            action_btn.set_label("󰚥");
            action_btn.add_css_class("qs-disconnect-btn");
            action_btn.set_tooltip_text(Some("Disconnect"));

            action_btn.connect_clicked(move |_| {
                status_lbl.set_text(&format!("Disconnecting from {name}..."));
                status_lbl.set_visible(true);
                let mac_cl = mac.clone();
                let (tx, rx) = mpsc::channel();

                std::thread::spawn(move || {
                    let res = bluetooth::disconnect_device(&mac_cl);
                    let _ = tx.send(res);
                });

                let status_lbl_cb = status_lbl.clone();
                let pill_lbl_cb = pill_lbl.clone();
                let b_sub_cb = b_sub.clone();
                let b_icon_cb = b_icon.clone();
                let b_tile_cb = b_tile.clone();
                let list_b_cb = list_b.clone();

                glib::timeout_add_local(Duration::from_millis(50), move || {
                    match rx.try_recv() {
                        Ok(res) => {
                            match res {
                                Ok(_) => {
                                    status_lbl_cb.set_visible(false);
                                    update_overview_bt(&pill_lbl_cb, &b_sub_cb, &b_icon_cb, &b_tile_cb);
                                    let devs = bluetooth::get_devices();
                                    populate_bt_list(
                                        &list_b_cb,
                                        &devs,
                                        &status_lbl_cb,
                                        &pill_lbl_cb,
                                        &b_sub_cb,
                                        &b_icon_cb,
                                        &b_tile_cb,
                                    );
                                }
                                Err(e) => {
                                    status_lbl_cb.set_text(&format!("Failed: {e}"));
                                }
                            }
                            glib::ControlFlow::Break
                        }
                        Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                        Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                    }
                });
            });
        } else {
            action_btn.set_label("󰅂");
            action_btn.add_css_class("qs-connect-btn");
            action_btn.set_tooltip_text(Some("Connect"));

            action_btn.connect_clicked(move |_| {
                status_lbl.set_text(&format!("Connecting to {name}..."));
                status_lbl.set_visible(true);
                let mac_cl = mac.clone();
                let (tx, rx) = mpsc::channel();

                std::thread::spawn(move || {
                    let res = bluetooth::connect_device(&mac_cl);
                    let _ = tx.send(res);
                });

                let status_lbl_cb = status_lbl.clone();
                let pill_lbl_cb = pill_lbl.clone();
                let b_sub_cb = b_sub.clone();
                let b_icon_cb = b_icon.clone();
                let b_tile_cb = b_tile.clone();
                let list_b_cb = list_b.clone();

                glib::timeout_add_local(Duration::from_millis(50), move || {
                    match rx.try_recv() {
                        Ok(res) => {
                            match res {
                                Ok(_) => {
                                    status_lbl_cb.set_visible(false);
                                    update_overview_bt(&pill_lbl_cb, &b_sub_cb, &b_icon_cb, &b_tile_cb);
                                    let devs = bluetooth::get_devices();
                                    populate_bt_list(
                                        &list_b_cb,
                                        &devs,
                                        &status_lbl_cb,
                                        &pill_lbl_cb,
                                        &b_sub_cb,
                                        &b_icon_cb,
                                        &b_tile_cb,
                                    );
                                }
                                Err(e) => {
                                    status_lbl_cb.set_text(&format!("Failed: {e}"));
                                }
                            }
                            glib::ControlFlow::Break
                        }
                        Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                        Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                    }
                });
            });
        }

        item_card.append(&action_btn);
        list_box.append(&item_card);
    }
}

/// Returns signal glyph and color.
fn wifi_signal_info(signal: u8) -> (&'static str, &'static str) {
    match signal {
        75..=100 => ("󰤨", "#a4d1b4"),
        50..=74 => ("󰤥", "#9cebcc"),
        25..=49 => ("󰤢", "#f1c27d"),
        1..=24 => ("󰤟", "#fa746f"),
        _ => ("󰤯", "#6e7870"),
    }
}

/// Helper to remove all children from a box.
fn clear_children(container: &Box) {
    let model = container.observe_children();
    let mut to_remove = Vec::new();
    for i in 0..model.n_items() {
        if let Some(obj) = model.item(i) {
            if let Ok(w) = obj.downcast::<gtk4::Widget>() {
                to_remove.push(w);
            }
        }
    }
    for w in to_remove {
        container.remove(&w);
    }
}
