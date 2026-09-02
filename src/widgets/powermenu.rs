//! Power and Session HUD widget.
//!
//! Provides a full-screen frosted overlay with session controls:
//! Shutdown, Reboot, Suspend, Lock, and Logout, with complete keyboard navigation.

use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;

use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, EventControllerKey, GestureClick, Label,
    Orientation,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    PowerOff,
    Reboot,
    Suspend,
    Lock,
    Logout,
}

impl PowerAction {
    pub fn title(self) -> &'static str {
        match self {
            Self::PowerOff => "Shut Down",
            Self::Reboot => "Restart",
            Self::Suspend => "Suspend",
            Self::Lock => "Lock",
            Self::Logout => "Log Out",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::PowerOff => "󰐥",
            Self::Reboot => "󰜉",
            Self::Suspend => "󰤄",
            Self::Lock => "󰌾",
            Self::Logout => "󰗽",
        }
    }

    pub fn shortcut(self) -> &'static str {
        match self {
            Self::PowerOff => "1",
            Self::Reboot => "2",
            Self::Suspend => "3",
            Self::Lock => "4",
            Self::Logout => "5",
        }
    }

    pub fn css_class(self) -> &'static str {
        match self {
            Self::PowerOff => "action-poweroff",
            Self::Reboot => "action-reboot",
            Self::Suspend => "action-suspend",
            Self::Lock => "action-lock",
            Self::Logout => "action-logout",
        }
    }

    pub fn execute(self) {
        match self {
            Self::PowerOff => {
                let _ = Command::new("systemctl").arg("poweroff").spawn();
            }
            Self::Reboot => {
                let _ = Command::new("systemctl").arg("reboot").spawn();
            }
            Self::Suspend => {
                let _ = Command::new("systemctl").arg("suspend").spawn();
            }
            Self::Lock => {
                let _ = Command::new("loginctl").args(["lock-session"]).spawn();
            }
            Self::Logout => {
                let _ = Command::new("hyprctl").args(["dispatch", "exit"]).spawn();
            }
        }
    }
}

const ACTIONS: [PowerAction; 5] = [
    PowerAction::PowerOff,
    PowerAction::Reboot,
    PowerAction::Suspend,
    PowerAction::Lock,
    PowerAction::Logout,
];

pub struct PowerMenuWidget {
    window: ApplicationWindow,
    selected_index: Rc<RefCell<usize>>,
    tiles: Rc<RefCell<Vec<Button>>>,
}

impl PowerMenuWidget {
    pub fn new(app: &Application) -> (Self, ApplicationWindow) {
        let window = ApplicationWindow::builder().application(app).build();

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        window.set_exclusive_zone(0);

        // Span full screen for dim overlay backdrop
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.add_css_class("powermenu-backdrop");

        let root_box = Box::new(Orientation::Vertical, 0);
        root_box.set_halign(Align::Fill);
        root_box.set_valign(Align::Fill);
        root_box.set_hexpand(true);
        root_box.set_vexpand(true);

        // Center card
        let center_box = Box::new(Orientation::Vertical, 24);
        center_box.add_css_class("powermenu-card");
        center_box.set_halign(Align::Center);
        center_box.set_valign(Align::Center);
        center_box.set_hexpand(false);
        center_box.set_vexpand(false);

        // Header (Username + Host)
        let header_box = Box::new(Orientation::Vertical, 4);
        header_box.set_halign(Align::Center);

        let user = std::env::var("USER").unwrap_or_else(|_| "User".to_string());
        let host = std::fs::read_to_string("/etc/hostname")
            .unwrap_or_else(|_| "iFreeYuh".to_string())
            .trim()
            .to_string();

        let title = Label::new(Some(&format!("{user} @ {host}")));
        title.add_css_class("powermenu-title");
        header_box.append(&title);

        let subtitle = Label::new(Some("Choose a session action or press ESC to cancel"));
        subtitle.add_css_class("powermenu-subtitle");
        header_box.append(&subtitle);

        center_box.append(&header_box);

        // Actions Row
        let actions_row = Box::new(Orientation::Horizontal, 16);
        actions_row.add_css_class("powermenu-actions-row");
        actions_row.set_halign(Align::Center);

        let tiles = Rc::new(RefCell::new(Vec::new()));
        let selected_index = Rc::new(RefCell::new(0));

        for (idx, &action) in ACTIONS.iter().enumerate() {
            let tile = Button::new();
            tile.add_css_class("powermenu-tile");
            tile.add_css_class(action.css_class());
            tile.set_cursor_from_name(Some("pointer"));

            if idx == 0 {
                tile.add_css_class("selected");
            }

            let tile_box = Box::new(Orientation::Vertical, 8);
            tile_box.set_halign(Align::Center);
            tile_box.set_valign(Align::Center);

            let icon_lbl = Label::new(Some(action.icon()));
            icon_lbl.add_css_class("powermenu-tile-icon");
            tile_box.append(&icon_lbl);

            let name_lbl = Label::new(Some(action.title()));
            name_lbl.add_css_class("powermenu-tile-name");
            tile_box.append(&name_lbl);

            let key_lbl = Label::new(Some(action.shortcut()));
            key_lbl.add_css_class("powermenu-tile-key");
            tile_box.append(&key_lbl);

            tile.set_child(Some(&tile_box));

            let win_cl = window.clone();
            tile.connect_clicked(move |_| {
                win_cl.hide();
                action.execute();
            });

            actions_row.append(&tile);
            tiles.borrow_mut().push(tile);
        }

        center_box.append(&actions_row);

        // Dismiss on clicking backdrop outside card
        let win_dismiss = window.clone();
        let bg_gesture = GestureClick::new();
        bg_gesture.connect_pressed(move |_, _, _, _| {
            win_dismiss.hide();
        });
        root_box.add_controller(bg_gesture);

        // Don't propagate clicks on center_box to root_box
        let card_gesture = GestureClick::new();
        card_gesture.connect_pressed(|g, _, _, _| {
            g.set_state(gtk4::EventSequenceState::Claimed);
        });
        center_box.add_controller(card_gesture);

        // Center the card in root
        let spacer_top = Box::new(Orientation::Vertical, 0);
        spacer_top.set_vexpand(true);
        root_box.append(&spacer_top);
        root_box.append(&center_box);
        let spacer_bottom = Box::new(Orientation::Vertical, 0);
        spacer_bottom.set_vexpand(true);
        root_box.append(&spacer_bottom);

        window.set_child(Some(&root_box));
        window.hide();

        // Keyboard navigation
        let key_controller = EventControllerKey::new();
        {
            let win_cl = window.clone();
            let tiles_cl = tiles.clone();
            let selected_cl = selected_index.clone();

            key_controller.connect_key_pressed(move |_, keyval, _, _| {
                match keyval {
                    Key::Escape => {
                        win_cl.hide();
                        glib::Propagation::Stop
                    }
                    Key::Left | Key::h | Key::H => {
                        let mut sel = selected_cl.borrow_mut();
                        if *sel == 0 {
                            *sel = ACTIONS.len() - 1;
                        } else {
                            *sel -= 1;
                        }
                        update_highlight(&tiles_cl.borrow(), *sel);
                        glib::Propagation::Stop
                    }
                    Key::Right | Key::Tab | Key::l | Key::L => {
                        let mut sel = selected_cl.borrow_mut();
                        *sel = (*sel + 1) % ACTIONS.len();
                        update_highlight(&tiles_cl.borrow(), *sel);
                        glib::Propagation::Stop
                    }
                    Key::Return | Key::KP_Enter | Key::space => {
                        let sel = *selected_cl.borrow();
                        win_cl.hide();
                        if let Some(&action) = ACTIONS.get(sel) {
                            action.execute();
                        }
                        glib::Propagation::Stop
                    }
                    Key::_1 | Key::p | Key::P => {
                        win_cl.hide();
                        PowerAction::PowerOff.execute();
                        glib::Propagation::Stop
                    }
                    Key::_2 | Key::r | Key::R => {
                        win_cl.hide();
                        PowerAction::Reboot.execute();
                        glib::Propagation::Stop
                    }
                    Key::_3 | Key::s | Key::S => {
                        win_cl.hide();
                        PowerAction::Suspend.execute();
                        glib::Propagation::Stop
                    }
                    Key::_4 | Key::k | Key::K => {
                        win_cl.hide();
                        PowerAction::Lock.execute();
                        glib::Propagation::Stop
                    }
                    Key::_5 | Key::e | Key::E | Key::q | Key::Q => {
                        win_cl.hide();
                        PowerAction::Logout.execute();
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
        }
        window.add_controller(key_controller);

        let widget = Self {
            window: window.clone(),
            selected_index,
            tiles,
        };

        (widget, window)
    }

    /// Show the power menu window.
    pub fn show(&self) {
        *self.selected_index.borrow_mut() = 0;
        update_highlight(&self.tiles.borrow(), 0);
        self.window.present();
    }

    /// Hide the power menu window.
    pub fn hide(&self) {
        self.window.hide();
    }

    /// Toggle visibility.
    pub fn toggle(&self) {
        if self.window.is_visible() {
            self.hide();
        } else {
            self.show();
        }
    }
}

fn update_highlight(tiles: &[Button], selected_idx: usize) {
    for (i, tile) in tiles.iter().enumerate() {
        if i == selected_idx {
            tile.add_css_class("selected");
        } else {
            tile.remove_css_class("selected");
        }
    }
}
