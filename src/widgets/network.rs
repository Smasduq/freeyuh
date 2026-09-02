//! Network widget: status pill and interactive Wi-Fi connection panel.
//!
//! Hovering over or clicking the network pill opens a layer-shell Wi-Fi panel where users can:
//! - View available access points, signal strength, and security status.
//! - Connect to open, saved, or password-protected networks.
//! - Disconnect from the active network.
//! - Toggle Wi-Fi radio on/off and trigger rescans.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, EventControllerMotion, Label, Orientation,
    PasswordEntry, ScrolledWindow, Switch,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc;
use std::time::Duration;

use crate::services::network::{self, NetworkState, WifiNetwork};

const HIDE_DELAY: Duration = Duration::from_millis(250);

/// Creates the network pill button and its dropdown Wi-Fi connection panel.
///
/// Returns `(pill_button, pill_label)`.
pub fn create(app: &Application) -> (Button, Label) {
    let button = Button::new();
    button.add_css_class("network-pill");
    button.set_cursor_from_name(Some("pointer"));
    button.set_valign(Align::Center);

    let label = Label::new(None);
    label.set_use_markup(true);
    label.add_css_class("network-label");
    button.set_child(Some(&label));

    // --- Wi-Fi Dropdown Window ---
    let wifi_window = ApplicationWindow::builder().application(app).build();
    wifi_window.init_layer_shell();
    wifi_window.set_layer(Layer::Top);
    wifi_window.set_anchor(Edge::Top, true);
    wifi_window.set_anchor(Edge::Right, true);
    wifi_window.set_margin(Edge::Top, 42);
    wifi_window.set_margin(Edge::Right, 60);
    wifi_window.set_keyboard_mode(KeyboardMode::OnDemand);
    wifi_window.set_exclusive_zone(0);
    wifi_window.set_default_size(410, 530);
    wifi_window.add_css_class("wifi-window");

    let dropdown = Box::new(Orientation::Vertical, 10);
    dropdown.add_css_class("wifi-dropdown");
    dropdown.set_width_request(410);

    // --- Hero QuickSettings Tile Header ---
    let hero_card = Box::new(Orientation::Horizontal, 12);
    hero_card.add_css_class("wifi-hero-card");
    hero_card.set_valign(Align::Center);

    let icon_box = Box::new(Orientation::Horizontal, 0);
    icon_box.add_css_class("wifi-hero-icon-box");
    icon_box.set_valign(Align::Center);
    icon_box.set_halign(Align::Center);

    let title_icon = Label::new(None);
    title_icon.set_use_markup(true);
    title_icon.set_markup("<span color=\"#a4d1b4\">󰤨</span>");
    title_icon.add_css_class("wifi-hero-icon");
    icon_box.append(&title_icon);
    hero_card.append(&icon_box);

    let title_box = Box::new(Orientation::Vertical, 2);
    title_box.set_halign(Align::Start);
    title_box.set_valign(Align::Center);

    let title = Label::new(Some("Wi-Fi"));
    title.add_css_class("wifi-hero-title");
    title.set_halign(Align::Start);

    let subtitle = Label::new(Some("Wireless Connections"));
    subtitle.add_css_class("wifi-hero-subtitle");
    subtitle.set_halign(Align::Start);
    subtitle.set_ellipsize(EllipsizeMode::End);

    title_box.append(&title);
    title_box.append(&subtitle);
    hero_card.append(&title_box);

    let header_spacer = Box::new(Orientation::Horizontal, 0);
    header_spacer.set_hexpand(true);
    hero_card.append(&header_spacer);

    let rescan_btn = Button::new();
    rescan_btn.set_label("󰑐");
    rescan_btn.add_css_class("wifi-hero-btn");
    rescan_btn.set_cursor_from_name(Some("pointer"));
    rescan_btn.set_tooltip_text(Some("Scan for nearby networks"));
    hero_card.append(&rescan_btn);

    let wifi_switch = Switch::new();
    wifi_switch.add_css_class("wifi-switch");
    wifi_switch.set_active(network::wifi_enabled());
    wifi_switch.set_valign(Align::Center);
    hero_card.append(&wifi_switch);

    dropdown.append(&hero_card);

    // Status feedback banner
    let status_label = Label::new(None);
    status_label.add_css_class("wifi-status-banner");
    status_label.set_visible(false);
    status_label.set_halign(Align::Center);
    dropdown.append(&status_label);

    // Section title
    let section_header = Label::new(Some("AVAILABLE NETWORKS"));
    section_header.add_css_class("wifi-section-header");
    section_header.set_halign(Align::Start);
    dropdown.append(&section_header);

    // Network list inside ScrolledWindow
    let list_box = Box::new(Orientation::Vertical, 6);
    list_box.add_css_class("wifi-list-box");
    list_box.set_halign(Align::Fill);
    list_box.set_vexpand(true);

    let scroll = ScrolledWindow::builder()
        .child(&list_box)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .build();
    scroll.set_min_content_height(380);
    scroll.set_max_content_height(580);
    scroll.set_vexpand(true);
    scroll.add_css_class("wifi-scrolled-window");
    dropdown.append(&scroll);

    wifi_window.set_child(Some(&dropdown));
    wifi_window.hide();

    // Wire up panel refresh logic
    let reload_list = {
        let list_box = list_box.clone();
        let status_label = status_label.clone();
        let pill_label = label.clone();
        let wifi_switch = wifi_switch.clone();
        let subtitle = subtitle.clone();

        Rc::new(move || {
            let is_enabled = network::wifi_enabled();
            wifi_switch.set_active(is_enabled);

            if !is_enabled {
                subtitle.set_text("Disabled");
                clear_children(&list_box);
                let off_box = Box::new(Orientation::Vertical, 10);
                off_box.add_css_class("wifi-empty");
                off_box.set_halign(Align::Center);
                off_box.set_valign(Align::Center);
                off_box.set_vexpand(true);
                off_box.set_margin_top(80);

                let off_icon = Label::new(Some("󰤮"));
                off_icon.add_css_class("wifi-empty-icon");

                let off_text = Label::new(Some("Wi-Fi is turned off"));
                off_text.add_css_class("wifi-empty-text");

                let off_sub = Label::new(Some("Toggle the switch above to enable wireless"));
                off_sub.add_css_class("wifi-empty-sub");

                off_box.append(&off_icon);
                off_box.append(&off_text);
                off_box.append(&off_sub);
                list_box.append(&off_box);
                return;
            }

            match network::query() {
                NetworkState::Wifi { ref ssid } => {
                    subtitle.set_text(&format!("Connected · {ssid}"));
                }
                _ => {
                    subtitle.set_text("Enabled · Scanning nearby");
                }
            }

            // Show scanning indicator if list is currently empty
            if list_box.observe_children().n_items() == 0 {
                let scanning = Label::new(Some("Scanning for networks..."));
                scanning.add_css_class("wifi-scanning");
                scanning.set_margin_top(40);
                list_box.append(&scanning);
            }

            let (tx, rx) = mpsc::channel::<Vec<WifiNetwork>>();
            std::thread::spawn(move || {
                let networks = network::scan_wifi();
                let _ = tx.send(networks);
            });

            let list_box_cb = list_box.clone();
            let status_label_cb = status_label.clone();
            let pill_label_cb = pill_label.clone();
            let subtitle_cb = subtitle.clone();
            glib::timeout_add_local(Duration::from_millis(50), move || {
                match rx.try_recv() {
                    Ok(networks) => {
                        match network::query() {
                            NetworkState::Wifi { ref ssid } => {
                                subtitle_cb.set_text(&format!("Connected · {ssid}"));
                            }
                            _ => {
                                subtitle_cb.set_text(&format!("{} networks found", networks.len()));
                            }
                        }
                        populate_list(
                            &list_box_cb,
                            &networks,
                            &status_label_cb,
                            &pill_label_cb,
                            &subtitle_cb,
                        );
                        glib::ControlFlow::Break
                    }
                    Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                    Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
                }
            });
        })
    };

    // Rescan button
    {
        let reload = reload_list.clone();
        rescan_btn.connect_clicked(move |_| {
            network::rescan_wifi();
            reload();
        });
    }

    // Toggle switch
    {
        let reload = reload_list.clone();
        let pill_label = label.clone();
        wifi_switch.connect_state_set(move |_, state| {
            network::set_wifi_enabled(state);
            refresh(&pill_label);
            reload();
            glib::Propagation::Proceed
        });
    }

    // Source handle for delayed hide on pointer leave.
    let hide_source = Rc::new(Cell::new(None::<glib::SourceId>));

    // Network pill button: show wifi panel on hover, schedule hide on leave.
    let button_motion = EventControllerMotion::new();
    button_motion.connect_enter({
        let hide_source = hide_source.clone();
        let wifi_win = wifi_window.clone();
        let reload = reload_list.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
            reload();
            wifi_win.present();
        }
    });

    button_motion.connect_leave({
        let hide_source = hide_source.clone();
        let wifi_win = wifi_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let wifi_win = wifi_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                wifi_win.hide();
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    button.add_controller(button_motion);

    // Dropdown window: cancel hide while hovered, schedule hide on leave.
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
        let wifi_win = wifi_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let wifi_win = wifi_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                wifi_win.hide();
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    dropdown.add_controller(dropdown_motion);

    // Also support clicking to toggle.
    let wifi_win_click = wifi_window.clone();
    let reload_click = reload_list.clone();
    button.connect_clicked(move |_| {
        if wifi_win_click.is_visible() {
            wifi_win_click.hide();
        } else {
            reload_click();
            wifi_win_click.present();
        }
    });

    (button, label)
}

/// Populates `list_box` with Caelestia/M3-styled Wi-Fi network cards.
fn populate_list(
    list_box: &Box,
    networks: &[WifiNetwork],
    status_label: &Label,
    pill_label: &Label,
    subtitle_label: &Label,
) {
    clear_children(list_box);

    if networks.is_empty() {
        let empty_box = Box::new(Orientation::Vertical, 10);
        empty_box.add_css_class("wifi-empty");
        empty_box.set_halign(Align::Center);
        empty_box.set_valign(Align::Center);
        empty_box.set_vexpand(true);
        empty_box.set_margin_top(60);

        let empty_icon = Label::new(Some("󰤭"));
        empty_icon.add_css_class("wifi-empty-icon");

        let empty_text = Label::new(Some("No networks found"));
        empty_text.add_css_class("wifi-empty-text");

        let empty_sub = Label::new(Some("Click 󰑐 above to rescan"));
        empty_sub.add_css_class("wifi-empty-sub");

        empty_box.append(&empty_icon);
        empty_box.append(&empty_text);
        empty_box.append(&empty_sub);
        list_box.append(&empty_box);
        return;
    }

    for net in networks {
        let item_card = Box::new(Orientation::Vertical, 6);
        item_card.add_css_class("wifi-item");
        if net.is_connected {
            item_card.add_css_class("connected");
        }

        // Main content row
        let row = Box::new(Orientation::Horizontal, 12);
        row.set_valign(Align::Center);

        // Left squircle icon container
        let icon_chip = Box::new(Orientation::Horizontal, 0);
        icon_chip.add_css_class("wifi-icon-chip");
        icon_chip.set_valign(Align::Center);
        icon_chip.set_halign(Align::Center);

        let (icon_str, icon_color) = wifi_signal_info(net.signal);
        let icon_label = Label::new(None);
        icon_label.set_use_markup(true);
        icon_label.set_markup(&format!("<span color=\"{icon_color}\">{icon_str}</span>"));
        icon_label.add_css_class("wifi-item-icon");
        icon_chip.append(&icon_label);
        row.append(&icon_chip);

        // Network info column (SSID + badges)
        let info_box = Box::new(Orientation::Vertical, 3);
        info_box.set_halign(Align::Start);
        info_box.set_hexpand(true);

        let name_label = Label::new(Some(&net.ssid));
        name_label.add_css_class("wifi-item-name");
        name_label.set_halign(Align::Start);
        name_label.set_ellipsize(EllipsizeMode::End);
        info_box.append(&name_label);

        let meta_row = Box::new(Orientation::Horizontal, 6);
        meta_row.set_valign(Align::Center);

        if net.is_connected {
            let conn_badge = Label::new(Some("Connected"));
            conn_badge.add_css_class("wifi-connected-badge");
            meta_row.append(&conn_badge);
        } else if net.is_saved {
            let saved_badge = Label::new(Some("Saved"));
            saved_badge.add_css_class("wifi-saved-badge");
            meta_row.append(&saved_badge);
        }

        let sig_label = Label::new(Some(&format!("{}%", net.signal)));
        sig_label.add_css_class("wifi-item-signal");
        meta_row.append(&sig_label);

        // Lock icon if secured
        let is_secured = !net.security.is_empty() && !net.security.contains("--");
        if is_secured {
            let lock = Label::new(Some("󰌾"));
            lock.add_css_class("wifi-lock-icon");
            lock.set_tooltip_text(Some(&format!("Security: {}", net.security)));
            meta_row.append(&lock);
        }

        info_box.append(&meta_row);
        row.append(&info_box);

        // Action button
        let action_btn = Button::new();
        action_btn.set_cursor_from_name(Some("pointer"));
        action_btn.set_valign(Align::Center);

        if net.is_connected {
            action_btn.set_label("Disconnect");
            action_btn.add_css_class("wifi-disconnect-btn");

            let ssid = net.ssid.clone();
            let status_lbl = status_label.clone();
            let pill_lbl = pill_label.clone();
            let subtitle_lbl = subtitle_label.clone();
            let list_b = list_box.clone();

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
                let subtitle_lbl_cb = subtitle_lbl.clone();
                let list_b_cb = list_b.clone();
                glib::timeout_add_local(Duration::from_millis(50), move || {
                    match rx.try_recv() {
                        Ok(res) => {
                            match res {
                                Ok(_) => {
                                    status_lbl_cb.set_text("Disconnected");
                                    subtitle_lbl_cb.set_text("Disconnected");
                                    refresh(&pill_lbl_cb);
                                    let nets = network::scan_wifi();
                                    populate_list(
                                        &list_b_cb,
                                        &nets,
                                        &status_lbl_cb,
                                        &pill_lbl_cb,
                                        &subtitle_lbl_cb,
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
            row.append(&action_btn);
        } else {
            action_btn.set_label("Connect");
            action_btn.add_css_class("wifi-connect-btn");

            let password_container = Box::new(Orientation::Horizontal, 8);
            password_container.add_css_class("wifi-password-box");
            password_container.set_visible(false);

            let pass_entry = PasswordEntry::new();
            pass_entry.set_placeholder_text(Some("Password..."));
            pass_entry.add_css_class("wifi-password-entry");
            pass_entry.set_hexpand(true);
            password_container.append(&pass_entry);

            let join_btn = Button::new();
            join_btn.set_label("Join");
            join_btn.add_css_class("wifi-join-btn");
            join_btn.set_cursor_from_name(Some("pointer"));
            password_container.append(&join_btn);

            let cancel_btn = Button::new();
            cancel_btn.set_label("✕");
            cancel_btn.add_css_class("wifi-cancel-btn");
            cancel_btn.set_cursor_from_name(Some("pointer"));
            password_container.append(&cancel_btn);

            // Connect action
            let ssid = net.ssid.clone();
            let is_saved = net.is_saved;
            let status_lbl = status_label.clone();
            let pill_lbl = pill_label.clone();
            let subtitle_lbl = subtitle_label.clone();
            let list_b = list_box.clone();
            let pass_box = password_container.clone();

            action_btn.connect_clicked(move |_| {
                if is_saved || !is_secured {
                    // Direct connect
                    status_lbl.set_text(&format!("Connecting to {ssid}..."));
                    status_lbl.set_visible(true);
                    let ssid_cl = ssid.clone();
                    let (tx, rx) = mpsc::channel();

                    std::thread::spawn(move || {
                        let res = network::connect_wifi(&ssid_cl, None);
                        let _ = tx.send(res);
                    });

                    let ssid_cb = ssid.clone();
                    let status_lbl_cb = status_lbl.clone();
                    let pill_lbl_cb = pill_lbl.clone();
                    let subtitle_lbl_cb = subtitle_lbl.clone();
                    let list_b_cb = list_b.clone();
                    glib::timeout_add_local(Duration::from_millis(50), move || {
                        match rx.try_recv() {
                            Ok(res) => {
                                match res {
                                    Ok(_) => {
                                        status_lbl_cb.set_text(&format!("Connected to {ssid_cb}"));
                                        subtitle_lbl_cb.set_text(&format!("Connected · {ssid_cb}"));
                                        refresh(&pill_lbl_cb);
                                        let nets = network::scan_wifi();
                                        populate_list(
                                            &list_b_cb,
                                            &nets,
                                            &status_lbl_cb,
                                            &pill_lbl_cb,
                                            &subtitle_lbl_cb,
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
                    // Reveal password box
                    pass_box.set_visible(true);
                }
            });

            // Join with password
            let ssid_join = net.ssid.clone();
            let status_lbl_join = status_label.clone();
            let pill_lbl_join = pill_label.clone();
            let subtitle_lbl_join = subtitle_label.clone();
            let list_b_join = list_box.clone();
            let pass_entry_cl = pass_entry.clone();

            join_btn.connect_clicked(move |_| {
                let password = pass_entry_cl.text().to_string();
                status_lbl_join.set_text(&format!("Connecting to {ssid_join}..."));
                status_lbl_join.set_visible(true);

                let ssid_cl = ssid_join.clone();
                let (tx, rx) = mpsc::channel();

                std::thread::spawn(move || {
                    let res = network::connect_wifi(&ssid_cl, Some(&password));
                    let _ = tx.send(res);
                });

                let ssid_cb = ssid_join.clone();
                let status_lbl_cb = status_lbl_join.clone();
                let pill_lbl_cb = pill_lbl_join.clone();
                let subtitle_lbl_cb = subtitle_lbl_join.clone();
                let list_b_cb = list_b_join.clone();
                glib::timeout_add_local(Duration::from_millis(50), move || {
                    match rx.try_recv() {
                        Ok(res) => {
                            match res {
                                Ok(_) => {
                                    status_lbl_cb.set_text(&format!("Connected to {ssid_cb}"));
                                    subtitle_lbl_cb.set_text(&format!("Connected · {ssid_cb}"));
                                    refresh(&pill_lbl_cb);
                                    let nets = network::scan_wifi();
                                    populate_list(
                                        &list_b_cb,
                                        &nets,
                                        &status_lbl_cb,
                                        &pill_lbl_cb,
                                        &subtitle_lbl_cb,
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

            let pass_box_cancel = password_container.clone();
            cancel_btn.connect_clicked(move |_| {
                pass_box_cancel.set_visible(false);
            });

            row.append(&action_btn);
            item_card.append(&row);
            item_card.append(&password_container);
        }

        if net.is_connected {
            item_card.append(&row);
        }

        list_box.append(&item_card);
    }
}

/// Refreshes the network status icon and tooltip on the top bar.
pub fn refresh(label: &Label) {
    let parent_btn = label.parent().and_then(|p| p.downcast::<Button>().ok());

    match network::query() {
        NetworkState::Wifi { ssid } => {
            label.set_markup("<span color=\"#9cebcc\">󰤨</span>");
            label.set_tooltip_text(Some(&format!("Wi-Fi: {ssid} (Hover for Wi-Fi panel)")));

            if let Some(btn) = parent_btn {
                btn.add_css_class("connected");
                btn.remove_css_class("disconnected");
            }
        }
        NetworkState::Ethernet { name } => {
            label.set_markup("<span color=\"#86dcce\">󰈀</span>");
            label.set_tooltip_text(Some(&format!("Ethernet: {name}")));

            if let Some(btn) = parent_btn {
                btn.add_css_class("connected");
                btn.remove_css_class("disconnected");
            }
        }
        NetworkState::Disconnected => {
            label.set_markup("<span color=\"#fa746f\">󰤭</span>");
            label.set_tooltip_text(Some("Network: Disconnected (Hover to connect)"));

            if let Some(btn) = parent_btn {
                btn.remove_css_class("connected");
                btn.add_css_class("disconnected");
            }
        }
    }
}

/// Return signal glyph and color depending on percentage.
fn wifi_signal_info(signal: u8) -> (&'static str, &'static str) {
    match signal {
        80..=100 => ("󰤨", "#a4d1b4"),
        60..=79 => ("󰤥", "#9cebcc"),
        40..=59 => ("󰤢", "#7ad9bc"),
        20..=39 => ("󰤟", "#cec06b"),
        _ => ("󰤯", "#fa746f"),
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
