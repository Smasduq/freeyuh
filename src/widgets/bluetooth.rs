//! Bluetooth widget: status icon and interactive device management panel.
//!
//! Hovering over or clicking the Bluetooth pill opens a layer-shell panel where users can:
//! - View paired and nearby Bluetooth devices.
//! - Connect or disconnect from devices with one click.
//! - Toggle Bluetooth radio on/off and trigger device discovery.

use gtk4::pango::EllipsizeMode;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, EventControllerMotion, Label, Orientation,
    ScrolledWindow, Switch,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc;
use std::time::Duration;

use crate::services::bluetooth::{self, BluetoothDevice, BluetoothState};

const HIDE_DELAY: Duration = Duration::from_millis(250);

/// Creates the Bluetooth bar pill button and its dropdown panel.
///
/// Returns `(pill_button, pill_label)`.
pub fn create(app: &Application) -> (Button, Label) {
    let button = Button::new();
    button.add_css_class("bt-pill");
    button.set_cursor_from_name(Some("pointer"));
    button.set_valign(Align::Center);

    let label = Label::new(None);
    label.set_use_markup(true);
    label.add_css_class("bt-label");
    button.set_child(Some(&label));

    // --- Bluetooth Dropdown Window ---
    let bt_window = ApplicationWindow::builder().application(app).build();
    bt_window.init_layer_shell();
    bt_window.set_layer(Layer::Top);
    bt_window.set_anchor(Edge::Top, true);
    bt_window.set_anchor(Edge::Right, true);
    bt_window.set_margin(Edge::Top, 42);
    bt_window.set_margin(Edge::Right, 85);
    bt_window.set_keyboard_mode(KeyboardMode::OnDemand);
    bt_window.set_exclusive_zone(0);
    bt_window.set_default_size(390, 480);
    bt_window.add_css_class("bt-window");

    let dropdown = Box::new(Orientation::Vertical, 8);
    dropdown.add_css_class("bt-dropdown");
    dropdown.set_width_request(390);

    // --- Hero Header ---
    let hero_card = Box::new(Orientation::Horizontal, 12);
    hero_card.add_css_class("bt-hero-card");
    hero_card.set_valign(Align::Center);

    let icon_box = Box::new(Orientation::Horizontal, 0);
    icon_box.add_css_class("bt-hero-icon-box");
    icon_box.set_valign(Align::Center);
    icon_box.set_halign(Align::Center);

    let title_icon = Label::new(None);
    title_icon.set_use_markup(true);
    title_icon.set_markup("<span color=\"#a4d1b4\">󰂯</span>");
    title_icon.add_css_class("bt-hero-icon");
    icon_box.append(&title_icon);
    hero_card.append(&icon_box);

    let title_box = Box::new(Orientation::Vertical, 2);
    title_box.set_halign(Align::Start);
    title_box.set_valign(Align::Center);

    let title = Label::new(Some("Bluetooth"));
    title.add_css_class("bt-hero-title");
    title.set_halign(Align::Start);

    let subtitle = Label::new(Some("Wireless Devices"));
    subtitle.add_css_class("bt-hero-subtitle");
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
    rescan_btn.add_css_class("bt-hero-btn");
    rescan_btn.set_cursor_from_name(Some("pointer"));
    rescan_btn.set_tooltip_text(Some("Scan for nearby Bluetooth devices"));
    hero_card.append(&rescan_btn);

    let bt_switch = Switch::new();
    bt_switch.add_css_class("bt-switch");
    bt_switch.set_active(bluetooth::is_enabled());
    bt_switch.set_valign(Align::Center);
    hero_card.append(&bt_switch);

    dropdown.append(&hero_card);

    // Status feedback banner
    let status_label = Label::new(None);
    status_label.add_css_class("bt-status-banner");
    status_label.set_visible(false);
    status_label.set_halign(Align::Center);
    dropdown.append(&status_label);

    // Section title
    let section_header = Label::new(Some("PAIRED & NEARBY DEVICES"));
    section_header.add_css_class("bt-section-header");
    section_header.set_halign(Align::Start);
    dropdown.append(&section_header);

    // Device list inside ScrolledWindow
    let list_box = Box::new(Orientation::Vertical, 4);
    list_box.add_css_class("bt-list-box");
    list_box.set_halign(Align::Fill);
    list_box.set_vexpand(true);

    let scroll = ScrolledWindow::builder()
        .child(&list_box)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .build();
    scroll.set_min_content_height(320);
    scroll.set_max_content_height(480);
    scroll.set_vexpand(true);
    scroll.add_css_class("bt-scrolled-window");
    dropdown.append(&scroll);

    bt_window.set_child(Some(&dropdown));
    bt_window.hide();

    // Wire up panel reload logic
    let reload_list = {
        let list_box = list_box.clone();
        let status_label = status_label.clone();
        let pill_label = label.clone();
        let bt_switch = bt_switch.clone();
        let subtitle = subtitle.clone();

        Rc::new(move || {
            let is_enabled = bluetooth::is_enabled();
            bt_switch.set_active(is_enabled);

            if !is_enabled {
                subtitle.set_text("Disabled");
                clear_children(&list_box);
                let off_box = Box::new(Orientation::Vertical, 10);
                off_box.add_css_class("bt-empty");
                off_box.set_halign(Align::Center);
                off_box.set_valign(Align::Center);
                off_box.set_vexpand(true);
                off_box.set_margin_top(60);

                let off_icon = Label::new(Some("󰂲"));
                off_icon.add_css_class("bt-empty-icon");

                let off_text = Label::new(Some("Bluetooth is turned off"));
                off_text.add_css_class("bt-empty-text");

                let off_sub = Label::new(Some("Toggle the switch above to enable Bluetooth"));
                off_sub.add_css_class("bt-empty-sub");

                off_box.append(&off_icon);
                off_box.append(&off_text);
                off_box.append(&off_sub);
                list_box.append(&off_box);
                return;
            }

            match bluetooth::query() {
                BluetoothState::Connected { ref name, .. } => {
                    subtitle.set_text(&format!("Connected · {name}"));
                }
                BluetoothState::Enabled { paired_count } => {
                    if paired_count > 0 {
                        subtitle.set_text(&format!("{paired_count} paired devices"));
                    } else {
                        subtitle.set_text("Ready to pair");
                    }
                }
                _ => {
                    subtitle.set_text("Bluetooth Enabled");
                }
            }

            let (tx, rx) = mpsc::channel::<Vec<BluetoothDevice>>();
            std::thread::spawn(move || {
                let devices = bluetooth::get_devices();
                let _ = tx.send(devices);
            });

            let list_box_cb = list_box.clone();
            let status_label_cb = status_label.clone();
            let pill_label_cb = pill_label.clone();
            let subtitle_cb = subtitle.clone();

            glib::timeout_add_local(Duration::from_millis(50), move || {
                match rx.try_recv() {
                    Ok(devices) => {
                        populate_list(
                            &list_box_cb,
                            &devices,
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
        let status_lbl = status_label.clone();
        rescan_btn.connect_clicked(move |_| {
            status_lbl.set_text("Scanning for devices...");
            status_lbl.set_visible(true);
            bluetooth::scan_on();
            reload();
        });
    }

    // Toggle switch
    {
        let reload = reload_list.clone();
        let pill_label = label.clone();
        bt_switch.connect_state_set(move |_, state| {
            let _ = bluetooth::set_enabled(state);
            refresh(&pill_label);
            reload();
            glib::Propagation::Proceed
        });
    }

    // Source handle for delayed hide on pointer leave.
    let hide_source = Rc::new(Cell::new(None::<glib::SourceId>));

    // Bluetooth pill button: show panel on hover, schedule hide on leave.
    let button_motion = EventControllerMotion::new();
    button_motion.connect_enter({
        let hide_source = hide_source.clone();
        let bt_win = bt_window.clone();
        let reload = reload_list.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
            reload();
            bt_win.present();
        }
    });

    button_motion.connect_leave({
        let hide_source = hide_source.clone();
        let bt_win = bt_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let bt_win = bt_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                bt_win.hide();
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
        let bt_win = bt_window.clone();
        move |_| {
            let hide_source_cb = hide_source.clone();
            let bt_win = bt_win.clone();
            let source = glib::timeout_add_local(HIDE_DELAY, move || {
                hide_source_cb.set(None);
                bt_win.hide();
                glib::ControlFlow::Break
            });
            hide_source.set(Some(source));
        }
    });
    dropdown.add_controller(dropdown_motion);

    // Also support clicking to toggle.
    let bt_win_click = bt_window.clone();
    let reload_click = reload_list.clone();
    button.connect_clicked(move |_| {
        if bt_win_click.is_visible() {
            bt_win_click.hide();
        } else {
            reload_click();
            bt_win_click.present();
        }
    });

    (button, label)
}

/// Populates `list_box` with backgroundless Bluetooth device items.
fn populate_list(
    list_box: &Box,
    devices: &[BluetoothDevice],
    status_label: &Label,
    pill_label: &Label,
    subtitle_label: &Label,
) {
    clear_children(list_box);

    if devices.is_empty() {
        let empty_box = Box::new(Orientation::Vertical, 10);
        empty_box.add_css_class("bt-empty");
        empty_box.set_halign(Align::Center);
        empty_box.set_valign(Align::Center);
        empty_box.set_vexpand(true);
        empty_box.set_margin_top(40);

        let empty_icon = Label::new(Some("󰂲"));
        empty_icon.add_css_class("bt-empty-icon");

        let empty_text = Label::new(Some("No Bluetooth devices found"));
        empty_text.add_css_class("bt-empty-text");

        let empty_sub = Label::new(Some("Click 󰑐 above to scan for devices"));
        empty_sub.add_css_class("bt-empty-sub");

        empty_box.append(&empty_icon);
        empty_box.append(&empty_text);
        empty_box.append(&empty_sub);
        list_box.append(&empty_box);
        return;
    }

    for dev in devices {
        let item_card = Box::new(Orientation::Horizontal, 12);
        item_card.add_css_class("bt-item");
        item_card.set_valign(Align::Center);
        if dev.is_connected {
            item_card.add_css_class("connected");
        }

        // Left device icon
        let icon_chip = Box::new(Orientation::Horizontal, 0);
        icon_chip.add_css_class("bt-icon-chip");
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
        icon_label.add_css_class("bt-item-icon");
        icon_chip.append(&icon_label);
        item_card.append(&icon_chip);

        // Device info column
        let info_box = Box::new(Orientation::Vertical, 2);
        info_box.set_halign(Align::Start);
        info_box.set_hexpand(true);

        let name_label = Label::new(Some(&dev.name));
        name_label.add_css_class("bt-item-name");
        name_label.set_halign(Align::Start);
        name_label.set_ellipsize(EllipsizeMode::End);
        info_box.append(&name_label);

        let meta_row = Box::new(Orientation::Horizontal, 6);
        meta_row.set_valign(Align::Center);

        if dev.is_connected {
            let conn_icon = Label::new(Some("󰄬"));
            conn_icon.add_css_class("bt-connected-icon");
            conn_icon.set_tooltip_text(Some("Connected"));
            meta_row.append(&conn_icon);

            if let Some(bat) = dev.battery {
                let bat_label = Label::new(Some(&format!("{bat}%")));
                bat_label.add_css_class("bt-item-battery");
                meta_row.append(&bat_label);
            }
        } else if dev.is_paired {
            let paired_icon = Label::new(Some("󰋑"));
            paired_icon.add_css_class("bt-paired-icon");
            paired_icon.set_tooltip_text(Some("Paired Device"));
            meta_row.append(&paired_icon);
        }

        info_box.append(&meta_row);
        item_card.append(&info_box);

        // Action button (icon only)
        let action_btn = Button::new();
        action_btn.set_cursor_from_name(Some("pointer"));
        action_btn.set_valign(Align::Center);

        let mac = dev.mac.clone();
        let name = dev.name.clone();
        let is_connected = dev.is_connected;
        let status_lbl = status_label.clone();
        let pill_lbl = pill_label.clone();
        let subtitle_lbl = subtitle_label.clone();
        let list_b = list_box.clone();

        if is_connected {
            action_btn.set_label("󰚥");
            action_btn.add_css_class("bt-disconnect-btn");
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
                let subtitle_lbl_cb = subtitle_lbl.clone();
                let list_b_cb = list_b.clone();

                glib::timeout_add_local(Duration::from_millis(50), move || {
                    match rx.try_recv() {
                        Ok(res) => {
                            match res {
                                Ok(_) => {
                                    status_lbl_cb.set_visible(false);
                                    subtitle_lbl_cb.set_text("Disconnected");
                                    refresh(&pill_lbl_cb);
                                    let devs = bluetooth::get_devices();
                                    populate_list(
                                        &list_b_cb,
                                        &devs,
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
        } else {
            action_btn.set_label("󰅂");
            action_btn.add_css_class("bt-connect-btn");
            action_btn.set_tooltip_text(Some("Connect"));

            action_btn.connect_clicked(move |_| {
                status_lbl.set_text(&format!("Connecting to {name}..."));
                status_lbl.set_visible(true);
                let mac_cl = mac.clone();
                let name_cl = name.clone();
                let (tx, rx) = mpsc::channel();

                std::thread::spawn(move || {
                    let res = bluetooth::connect_device(&mac_cl);
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
                                    status_lbl_cb.set_visible(false);
                                    subtitle_lbl_cb.set_text(&format!("Connected · {name_cl}"));
                                    refresh(&pill_lbl_cb);
                                    let devs = bluetooth::get_devices();
                                    populate_list(
                                        &list_b_cb,
                                        &devs,
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
        }

        item_card.append(&action_btn);
        list_box.append(&item_card);
    }
}

/// Refreshes the Bluetooth status icon on the top bar.
pub fn refresh(label: &Label) {
    match bluetooth::query() {
        BluetoothState::Connected { name, battery } => {
            let bat_str = battery.map(|b| format!(" {b}%")).unwrap_or_default();
            label.set_markup("<span color=\"#a4d1b4\">󰂯</span>");
            label.set_tooltip_text(Some(&format!("Bluetooth: Connected to {name}{bat_str}")));
        }
        BluetoothState::Enabled { paired_count } => {
            label.set_markup("<span color=\"#dee8df\">󰂯</span>");
            label.set_tooltip_text(Some(&format!("Bluetooth: On ({paired_count} paired)")));
        }
        BluetoothState::Disabled => {
            label.set_markup("<span color=\"#6e7870\">󰂲</span>");
            label.set_tooltip_text(Some("Bluetooth: Off (Hover to enable)"));
        }
        BluetoothState::Unavailable => {
            label.set_markup("<span color=\"#414a43\">󰂲</span>");
            label.set_tooltip_text(Some("Bluetooth: No adapter found"));
        }
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
