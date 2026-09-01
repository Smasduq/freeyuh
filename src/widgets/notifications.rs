//! Notification widget: toast popups and an on-demand notification center.
//!
//! The bar is the notification daemon (see `services::notifications`). When a
//! notification arrives it is forwarded here as an [`Event::Notification`]; the
//! widget shows a transient toast in the top-right and logs it into a scrollable
//! center. Hovering the bell button in the bar shows the center window.

use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Box, Button, Label, Orientation, ScrolledWindow};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
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
    center_scroll: ScrolledWindow,
    center_list: Box,
    history: Vec<Notification>,
}

impl NotificationWidget {
    /// Build the toast window, the center window and the bar bell button.
    /// Returns the widget and the bell button to place in the bar.
    pub fn new(app: &Application) -> (Self, Button) {
        // --- Toast window (top-right overlay, no exclusive zone) ---
        let toast_window = ApplicationWindow::builder().application(app).build();
        toast_window.init_layer_shell();
        toast_window.set_layer(Layer::Overlay);
        toast_window.set_anchor(Edge::Top, true);
        toast_window.set_anchor(Edge::Right, true);
        toast_window.set_margin(Edge::Top, 40);
        toast_window.set_margin(Edge::Right, 12);
        toast_window.set_keyboard_mode(KeyboardMode::None);
        toast_window.set_exclusive_zone(0);
        toast_window.set_default_size(-1, -1);
        let toasts_list = Box::new(Orientation::Vertical, 8);
        toasts_list.set_halign(Align::End);
        let toast_full = Box::new(Orientation::Horizontal, 0);
        toast_full.set_halign(Align::End);
        // A spacer keeps the toasts pinned to the right edge.
        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        toast_full.append(&spacer);
        toast_full.append(&toasts_list);
        toasts_list.set_halign(Align::End);
        toasts_list.set_width_request(360);
        toast_window.set_child(Some(&toast_full));

        // --- Center window (history, top-right, appears on demand) ---
        let center_window = ApplicationWindow::builder().application(app).build();
        center_window.init_layer_shell();
        center_window.set_layer(Layer::Top);
        center_window.set_anchor(Edge::Top, true);
        center_window.set_anchor(Edge::Right, true);
        center_window.set_margin(Edge::Top, 40);
        center_window.set_margin(Edge::Right, 12);
        center_window.set_keyboard_mode(KeyboardMode::None);
        center_window.set_exclusive_zone(0);
        center_window.set_default_size(360, -1);
        center_window.add_css_class("notif-center");
        let center_list = Box::new(Orientation::Vertical, 8);
        center_list.set_halign(Align::End);
        let scroll = ScrolledWindow::builder()
            .child(&center_list)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .build();
        scroll.set_max_content_height(480);
        center_window.set_child(Some(&scroll));
        center_list.set_width_request(360);

        // Ensure the center window starts hidden.
        center_window.hide();

        // --- Bell button (in the bar) ---
        let bell = Button::new();
        bell.add_css_class("bell");
        bell.set_label(" 󰂚");
        bell.set_tooltip_text(Some("Notifications"));
        bell.set_valign(Align::Center);

        // The toast window is shown on demand when a toast appears and hidden
        // once the last toast is dismissed, so it never peeks as an empty box.
        toast_window.hide();

        (
            Self {
                toast_window,
                toasts_list,
                center_window,
                center_scroll: scroll,
                center_list,
                history: Vec::new(),
            },
            bell,
        )
    }

    /// Show the notification center window.
    pub fn show_center(&mut self) {
        self.center_window.present();
    }

    /// Hide the notification center window.
    pub fn hide_center(&mut self) {
        self.center_window.hide();
    }

    /// Access to the center scroll widget so hover wiring can attach to it.
    pub fn center_scroll(&self) -> &ScrolledWindow {
        &self.center_scroll
    }

    /// Handle a notification-related event.
    pub fn handle(&mut self, event: &Event) {
        match event {
            Event::Notification(n) => {
                self.history.push(n.clone());
                self.present_toast(n);
                self.refresh_center();
            }
            Event::NotificationClosed { id } => {
                self.history.retain(|n| n.id != *id);
                self.refresh_center();
            }
            Event::ShowNotificationCenter => self.show_center(),
            Event::HideNotificationCenter => self.hide_center(),
            _ => {}
        }
    }

    /// Show a toast card for `n`, auto-dismissing after a timeout.
    fn present_toast(&mut self, n: &Notification) {
        let card = self.make_toast(n);
        self.toasts_list.append(&card);
        self.toast_window.present();

        // Auto-dismiss after the timeout, removing this card and hiding the
        // window once it is the last toast standing.
        let toasts_list = self.toasts_list.clone();
        let toast_window = self.toast_window.clone();
        let timeout = match n.urgency {
            Urgency::Critical => TOAST_TIMEOUT_CRITICAL,
            _ => TOAST_TIMEOUT,
        };
        glib::timeout_add_local(timeout, move || {
            toasts_list.remove(&card);
            if toasts_list.observe_children().n_items() == 0 {
                toast_window.hide();
            }
            glib::ControlFlow::Break
        });
    }

    /// Build a single toast card widget.
    fn make_toast(&self, n: &Notification) -> Box {
        let title = Label::new(Some(&n.summary));
        title.add_css_class("notif-title");
        title.set_halign(Align::Start);
        title.set_xalign(0.0);

        let card = Box::new(Orientation::Vertical, 4);
        card.add_css_class("notif-toast");
        let urgency_class = match n.urgency {
            Urgency::Critical => "critical",
            _ => "normal",
        };
        card.add_css_class(urgency_class);
        card.append(&title);

        if !n.body.is_empty() {
            let body = Label::new(Some(&n.body));
            body.add_css_class("notif-body");
            body.set_halign(Align::Start);
            body.set_xalign(0.0);
            body.set_wrap(true);
            card.append(&body);
        }

        if !n.app_name.is_empty() {
            let app = Label::new(Some(&n.app_name));
            app.add_css_class("notif-app");
            app.set_halign(Align::Start);
            app.set_xalign(0.0);
            card.append(&app);
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
        for n in &self.history {
            let card = self.make_toast(n);
            card.add_css_class("history-item");
            self.center_list.append(&card);
        }
    }
}
