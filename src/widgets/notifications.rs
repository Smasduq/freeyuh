//! Notification widget: toast popups and an on-demand notification center.
//!
//! The bar acts as the notification daemon (see `services::notifications`). When a
//! notification arrives it is forwarded here as an [`Event::Notification`]; the
//! widget shows a transient toast in the top-right and logs it into a scrollable
//! center. Hovering or clicking the bell button in the bar shows the center window.

use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, Label, Orientation, ScrolledWindow,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::events::{Event, Notification, Urgency};

/// How long a normal/low toast stays visible before dismissing.
const TOAST_TIMEOUT: Duration = Duration::from_secs(5);
/// Critical toasts stay longer.
const TOAST_TIMEOUT_CRITICAL: Duration = Duration::from_secs(15);

/// The notification display: owns the toast window, the center window and the
/// shared history. Lives entirely on the main thread.
pub struct NotificationWidget {
    #[allow(dead_code)]
    toast_window: ApplicationWindow,
    toasts_list: Box,
    center_window: ApplicationWindow,
    center_dropdown: Box,
    center_list: Box,
    history: Vec<Notification>,
    bell_badge: Label,
    bell_button: Button,
    tx: Sender<Event>,
}

impl NotificationWidget {
    /// Build the toast window, the center window and the bar bell button.
    /// Returns the widget and the bell button to place in the bar.
    pub fn new(app: &Application, tx: Sender<Event>) -> (Self, Button) {
        // --- Toast window (top-right overlay, no exclusive zone) ---
        let toast_window = ApplicationWindow::builder().application(app).build();
        toast_window.init_layer_shell();
        toast_window.set_layer(Layer::Overlay);
        toast_window.set_anchor(Edge::Top, true);
        toast_window.set_anchor(Edge::Right, true);
        toast_window.set_margin(Edge::Top, 44);
        toast_window.set_margin(Edge::Right, 14);
        toast_window.set_keyboard_mode(KeyboardMode::None);
        toast_window.set_exclusive_zone(0);
        toast_window.set_default_size(-1, -1);
        toast_window.add_css_class("toast-window");

        let toasts_list = Box::new(Orientation::Vertical, 8);
        toasts_list.set_halign(Align::End);
        let toast_full = Box::new(Orientation::Horizontal, 0);
        toast_full.set_halign(Align::End);
        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        toast_full.append(&spacer);
        toast_full.append(&toasts_list);
        toasts_list.set_halign(Align::End);
        toasts_list.set_width_request(360);
        toast_window.set_child(Some(&toast_full));

        // --- Center window (history, top-right, appears on hover) ---
        let center_window = ApplicationWindow::builder().application(app).build();
        center_window.init_layer_shell();
        center_window.set_layer(Layer::Top);
        center_window.set_anchor(Edge::Top, true);
        center_window.set_anchor(Edge::Right, true);
        center_window.set_margin(Edge::Top, 44);
        center_window.set_margin(Edge::Right, 10);
        center_window.set_keyboard_mode(KeyboardMode::None);
        center_window.set_exclusive_zone(0);
        center_window.set_default_size(400, 560);
        center_window.add_css_class("notif-center");

        // Dropdown container: header + scrollable notification list.
        let dropdown = Box::new(Orientation::Vertical, 0);
        dropdown.add_css_class("notif-dropdown");
        dropdown.set_width_request(400);

        let header = Box::new(Orientation::Horizontal, 8);
        header.add_css_class("notif-header");

        let title = Label::new(Some("Notifications"));
        title.add_css_class("notif-header-title");
        title.set_halign(Align::Start);
        title.set_xalign(0.0);
        header.append(&title);

        let header_spacer = Box::new(Orientation::Horizontal, 0);
        header_spacer.set_hexpand(true);
        header.append(&header_spacer);

        let clear_btn = Button::new();
        clear_btn.set_label("󰃢 Clear");
        clear_btn.add_css_class("notif-clear-btn");
        clear_btn.set_cursor_from_name(Some("pointer"));
        let tx_clear = tx.clone();
        clear_btn.connect_clicked(move |_| {
            let _ = tx_clear.send(Event::ClearAllNotifications);
        });
        header.append(&clear_btn);

        dropdown.append(&header);

        let center_list = Box::new(Orientation::Vertical, 6);
        center_list.set_halign(Align::Fill);
        center_list.set_vexpand(true);
        let scroll = ScrolledWindow::builder()
            .child(&center_list)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .build();
        scroll.set_min_content_height(480);
        scroll.set_max_content_height(720);
        scroll.set_vexpand(true);
        dropdown.append(&scroll);
        center_list.set_width_request(380);
        center_window.set_child(Some(&dropdown));

        center_window.hide();

        // --- Bell button (in the bar) ---
        let bell = Button::new();
        bell.add_css_class("bell");
        bell.set_cursor_from_name(Some("pointer"));
        bell.set_tooltip_text(Some("Notifications"));
        bell.set_valign(Align::Center);

        let bell_box = Box::new(Orientation::Horizontal, 4);
        let bell_icon = Label::new(Some("󰂚"));
        bell_icon.add_css_class("bell-icon");
        let bell_badge = Label::new(None);
        bell_badge.add_css_class("notif-badge");
        bell_badge.set_visible(false);

        bell_box.append(&bell_icon);
        bell_box.append(&bell_badge);
        bell.set_child(Some(&bell_box));

        toast_window.hide();

        let mut widget = Self {
            toast_window,
            toasts_list,
            center_window,
            center_dropdown: dropdown,
            center_list,
            history: Vec::new(),
            bell_badge,
            bell_button: bell.clone(),
            tx,
        };

        widget.refresh_center();

        (widget, bell)
    }

    /// Show the notification center window.
    pub fn show_center(&mut self) {
        self.center_window.present();
    }

    /// Hide the notification center window.
    pub fn hide_center(&mut self) {
        self.center_window.hide();
    }

    /// Toggle the notification center window visibility.
    pub fn toggle_center(&mut self) {
        if self.center_window.is_visible() {
            self.hide_center();
        } else {
            self.show_center();
        }
    }

    /// Access to the center dropdown so hover wiring can attach to it.
    pub fn center_dropdown(&self) -> &gtk4::Box {
        &self.center_dropdown
    }

    /// Update the unread badge count on the bell button.
    fn update_bell(&self) {
        let count = self.history.len();
        if count == 0 {
            self.bell_badge.set_visible(false);
            self.bell_button.remove_css_class("has-unread");
        } else {
            self.bell_badge.set_text(&count.to_string());
            self.bell_badge.set_visible(true);
            self.bell_button.add_css_class("has-unread");
        }
    }

    /// Handle a notification-related event.
    pub fn handle(&mut self, event: &Event) {
        match event {
            Event::Notification(n) => {
                self.history.push(n.clone());
                self.present_toast(n);
                self.refresh_center();
                self.update_bell();
            }
            Event::NotificationClosed { id } => {
                self.history.retain(|n| n.id != *id);
                self.refresh_center();
                self.update_bell();
            }
            Event::ClearAllNotifications => {
                self.history.clear();
                self.refresh_center();
                self.update_bell();
            }
            Event::DismissNotification(id) => {
                self.history.retain(|n| n.id != *id);
                self.refresh_center();
                self.update_bell();
            }
            Event::ShowNotificationCenter => self.show_center(),
            Event::HideNotificationCenter => self.hide_center(),
            Event::ToggleNotifications => self.toggle_center(),
            _ => {}
        }
    }

    /// Show a toast card for `n`, auto-dismissing after a timeout.
    fn present_toast(&mut self, n: &Notification) {
        let card = self.make_card(n, false);
        // Trigger the slide-in keyframe as soon as the widget is in the tree.
        card.add_css_class("toast-entering");
        self.toasts_list.append(&card);
        self.toast_window.present();

        let toasts_list = self.toasts_list.clone();
        let toast_window = self.toast_window.clone();
        let timeout = match n.urgency {
            Urgency::Critical => TOAST_TIMEOUT_CRITICAL,
            _ => TOAST_TIMEOUT,
        };
        glib::timeout_add_local(timeout, move || {
            // Start the slide-out animation, then remove after it finishes.
            card.remove_css_class("toast-entering");
            card.add_css_class("toast-leaving");

            let card2 = card.clone();
            let toasts_list2 = toasts_list.clone();
            let toast_window2 = toast_window.clone();
            glib::timeout_add_local(Duration::from_millis(210), move || {
                toasts_list2.remove(&card2);
                if toasts_list2.observe_children().n_items() == 0 {
                    toast_window2.hide();
                }
                glib::ControlFlow::Break
            });

            glib::ControlFlow::Break
        });
    }

    /// Build a notification card widget (used in both toast and center).
    fn make_card(&self, n: &Notification, show_close: bool) -> Box {
        let card = Box::new(Orientation::Vertical, 4);
        card.add_css_class("notif-toast");
        let urgency_class = match n.urgency {
            Urgency::Critical => "critical",
            _ => "normal",
        };
        card.add_css_class(urgency_class);

        // Header row: app name + close button (if in history)
        let top_row = Box::new(Orientation::Horizontal, 4);
        let app_name = if n.app_name.is_empty() {
            "Notification"
        } else {
            &n.app_name
        };
        let app_label = Label::new(Some(app_name));
        app_label.add_css_class("notif-app");
        app_label.set_halign(Align::Start);
        top_row.append(&app_label);

        let top_spacer = Box::new(Orientation::Horizontal, 0);
        top_spacer.set_hexpand(true);
        top_row.append(&top_spacer);

        if show_close {
            let close_btn = Button::new();
            close_btn.set_label("✕");
            close_btn.add_css_class("notif-card-close");
            close_btn.set_cursor_from_name(Some("pointer"));
            let tx_dismiss = self.tx.clone();
            let notif_id = n.id;
            close_btn.connect_clicked(move |_| {
                let _ = tx_dismiss.send(Event::DismissNotification(notif_id));
            });
            top_row.append(&close_btn);
        }

        card.append(&top_row);

        let title = Label::new(Some(&n.summary));
        title.add_css_class("notif-title");
        title.set_halign(Align::Start);
        title.set_xalign(0.0);
        card.append(&title);

        if !n.body.is_empty() {
            let body = Label::new(Some(&n.body));
            body.add_css_class("notif-body");
            body.set_halign(Align::Start);
            body.set_xalign(0.0);
            body.set_wrap(true);
            card.append(&body);
        }

        card
    }

    /// Rebuild the center list from `history`.
    fn refresh_center(&mut self) {
        let model = self.center_list.observe_children();
        let mut to_remove = Vec::new();
        for i in 0..model.n_items() {
            if let Some(obj) = model.item(i) {
                if let Ok(w) = obj.downcast::<gtk4::Widget>() {
                    to_remove.push(w);
                }
            }
        }
        for w in to_remove {
            self.center_list.remove(&w);
        }

        if self.history.is_empty() {
            let empty_box = Box::new(Orientation::Vertical, 8);
            empty_box.add_css_class("notif-empty");
            empty_box.set_halign(Align::Center);
            empty_box.set_valign(Align::Center);
            empty_box.set_vexpand(true);
            empty_box.set_hexpand(true);
            empty_box.set_margin_top(160);
            empty_box.set_margin_bottom(160);

            let empty_icon = Label::new(Some("󰂜"));
            empty_icon.add_css_class("notif-empty-icon");

            let empty_text = Label::new(Some("No new notifications"));
            empty_text.add_css_class("notif-empty-text");

            empty_box.append(&empty_icon);
            empty_box.append(&empty_text);
            self.center_list.append(&empty_box);
        } else {
            // Render newest first.
            for n in self.history.iter().rev() {
                let card = self.make_card(n, true);
                card.add_css_class("history-item");
                self.center_list.append(&card);
            }
        }
    }
}
