//! Spotlight App Launcher & Search HUD.
//!
//! A floating, centered glass search bar that indexes `.desktop` applications,
//! provides instant fuzzy filtering, keyboard navigation, and execution.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Entry, EventControllerKey, Image, Label,
    Orientation, ScrolledWindow,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::services::apps::{self, AppInfo};

const MAX_DISPLAY_RESULTS: usize = 30;

pub struct LauncherWidget {
    window: ApplicationWindow,
    search_entry: Entry,
    results_box: Box,
    footer_count: Label,
    all_apps: Rc<RefCell<Vec<AppInfo>>>,
    filtered_indices: Rc<RefCell<Vec<usize>>>,
    selected_index: Rc<RefCell<usize>>,
    item_rows: Rc<RefCell<Vec<Box>>>,
}

impl LauncherWidget {
    pub fn new(app: &Application) -> (Self, ApplicationWindow) {
        let window = ApplicationWindow::builder().application(app).build();

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_anchor(Edge::Top, true);
        window.set_margin(Edge::Top, 140);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        window.set_exclusive_zone(0);
        window.set_default_size(580, 480);
        window.add_css_class("launcher-window");

        let card = Box::new(Orientation::Vertical, 0);
        card.add_css_class("launcher-card");
        card.set_width_request(580);

        // --- Search Header Row ---
        let search_box = Box::new(Orientation::Horizontal, 10);
        search_box.add_css_class("launcher-search-box");
        search_box.set_valign(Align::Center);

        let search_icon = Label::new(Some("󰍉"));
        search_icon.add_css_class("launcher-search-icon");
        search_box.append(&search_icon);

        let search_entry = Entry::new();
        search_entry.set_placeholder_text(Some("Search applications or commands..."));
        search_entry.add_css_class("launcher-search-entry");
        search_entry.set_hexpand(true);
        search_box.append(&search_entry);

        let esc_chip = Label::new(Some("ESC"));
        esc_chip.add_css_class("launcher-chip");
        search_box.append(&esc_chip);

        card.append(&search_box);

        // --- Results Scrolled Container ---
        let results_box = Box::new(Orientation::Vertical, 2);
        results_box.add_css_class("launcher-results-box");
        results_box.set_vexpand(true);

        let scroll = ScrolledWindow::builder()
            .child(&results_box)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .build();
        scroll.set_min_content_height(340);
        scroll.set_max_content_height(400);
        scroll.set_vexpand(true);
        scroll.add_css_class("launcher-scrolled-window");
        card.append(&scroll);

        // --- Footer Navigation Hints ---
        let footer_box = Box::new(Orientation::Horizontal, 8);
        footer_box.add_css_class("launcher-footer");
        footer_box.set_valign(Align::Center);

        let nav_hint = Label::new(Some("↑↓ Navigate   ↵ Open"));
        nav_hint.add_css_class("launcher-hint");
        nav_hint.set_halign(Align::Start);
        nav_hint.set_hexpand(true);
        footer_box.append(&nav_hint);

        let footer_count = Label::new(None);
        footer_count.add_css_class("launcher-count");
        footer_box.append(&footer_count);

        card.append(&footer_box);
        window.set_child(Some(&card));
        window.hide();

        let all_apps = Rc::new(RefCell::new(Vec::new()));
        let filtered_indices = Rc::new(RefCell::new(Vec::new()));
        let selected_index = Rc::new(RefCell::new(0));
        let item_rows = Rc::new(RefCell::new(Vec::new()));

        let widget = Self {
            window: window.clone(),
            search_entry: search_entry.clone(),
            results_box: results_box.clone(),
            footer_count: footer_count.clone(),
            all_apps: all_apps.clone(),
            filtered_indices: filtered_indices.clone(),
            selected_index: selected_index.clone(),
            item_rows: item_rows.clone(),
        };

        // --- Search Input Filtering ---
        {
            let all_apps_cl = all_apps.clone();
            let filtered_cl = filtered_indices.clone();
            let selected_cl = selected_index.clone();
            let item_rows_cl = item_rows.clone();
            let results_box_cl = results_box.clone();
            let footer_count_cl = footer_count.clone();
            let win_cl = window.clone();

            search_entry.connect_changed(move |entry| {
                let query = entry.text().to_lowercase().trim().to_string();
                let apps = all_apps_cl.borrow();
                let mut filtered = Vec::new();

                for (idx, app) in apps.iter().enumerate() {
                    if query.is_empty() {
                        filtered.push(idx);
                    } else {
                        let name_match = app.name.to_lowercase().contains(&query);
                        let exec_match = app.exec.to_lowercase().contains(&query);
                        let comment_match = app.comment.to_lowercase().contains(&query);
                        let kw_match = app.keywords.iter().any(|k| k.contains(&query));

                        if name_match || exec_match || comment_match || kw_match {
                            filtered.push(idx);
                        }
                    }

                    if filtered.len() >= MAX_DISPLAY_RESULTS {
                        break;
                    }
                }

                *filtered_cl.borrow_mut() = filtered.clone();
                *selected_cl.borrow_mut() = 0;

                render_results(
                    &results_box_cl,
                    &apps,
                    &filtered,
                    0,
                    &item_rows_cl,
                    &footer_count_cl,
                    &win_cl,
                );
            });
        }

        // --- Keyboard Navigation Controller ---
        let key_controller = EventControllerKey::new();
        {
            let win_cl = window.clone();
            let all_apps_cl = all_apps.clone();
            let filtered_cl = filtered_indices.clone();
            let selected_cl = selected_index.clone();
            let item_rows_cl = item_rows.clone();

            key_controller.connect_key_pressed(move |_, keyval, _, _| {
                match keyval {
                    Key::Escape => {
                        win_cl.hide();
                        glib::Propagation::Stop
                    }
                    Key::Down | Key::Tab => {
                        let total = filtered_cl.borrow().len();
                        if total > 0 {
                            let mut sel = selected_cl.borrow_mut();
                            *sel = (*sel + 1) % total;
                            update_selection_highlight(&item_rows_cl.borrow(), *sel);
                        }
                        glib::Propagation::Stop
                    }
                    Key::Up => {
                        let total = filtered_cl.borrow().len();
                        if total > 0 {
                            let mut sel = selected_cl.borrow_mut();
                            if *sel == 0 {
                                *sel = total - 1;
                            } else {
                                *sel -= 1;
                            }
                            update_selection_highlight(&item_rows_cl.borrow(), *sel);
                        }
                        glib::Propagation::Stop
                    }
                    Key::Return | Key::KP_Enter => {
                        let filtered = filtered_cl.borrow();
                        let sel = *selected_cl.borrow();
                        if let Some(&app_idx) = filtered.get(sel) {
                            let apps = all_apps_cl.borrow();
                            if let Some(app) = apps.get(app_idx) {
                                let _ = apps::launch(app);
                                win_cl.hide();
                            }
                        }
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
        }
        window.add_controller(key_controller);

        (widget, window)
    }

    /// Show the launcher window and focus the search entry.
    pub fn show(&self) {
        let apps = apps::list_apps();
        *self.all_apps.borrow_mut() = apps.clone();

        let mut initial_filtered = Vec::new();
        for i in 0..apps.len().min(MAX_DISPLAY_RESULTS) {
            initial_filtered.push(i);
        }
        *self.filtered_indices.borrow_mut() = initial_filtered.clone();
        *self.selected_index.borrow_mut() = 0;

        self.search_entry.set_text("");
        render_results(
            &self.results_box,
            &apps,
            &initial_filtered,
            0,
            &self.item_rows,
            &self.footer_count,
            &self.window,
        );

        self.window.present();
        self.search_entry.grab_focus();
    }

    /// Hide the launcher window.
    pub fn hide(&self) {
        self.window.hide();
    }

    /// Toggle launcher window visibility.
    pub fn toggle(&self) {
        if self.window.is_visible() {
            self.hide();
        } else {
            self.show();
        }
    }
}

/// Render the filtered search results into `results_box`.
fn render_results(
    results_box: &Box,
    apps: &[AppInfo],
    filtered_indices: &[usize],
    selected_idx: usize,
    item_rows: &Rc<RefCell<Vec<Box>>>,
    footer_count: &Label,
    window: &ApplicationWindow,
) {
    // Clear previous widgets
    while let Some(child) = results_box.first_child() {
        results_box.remove(&child);
    }

    let mut rows = Vec::new();

    if filtered_indices.is_empty() {
        let empty_box = Box::new(Orientation::Vertical, 8);
        empty_box.add_css_class("launcher-empty");
        empty_box.set_halign(Align::Center);
        empty_box.set_valign(Align::Center);
        empty_box.set_margin_top(60);

        let empty_icon = Label::new(Some("󰍉"));
        empty_icon.add_css_class("launcher-empty-icon");
        let empty_text = Label::new(Some("No applications found"));
        empty_text.add_css_class("launcher-empty-text");

        empty_box.append(&empty_icon);
        empty_box.append(&empty_text);
        results_box.append(&empty_box);
        footer_count.set_text("0 matches");
        *item_rows.borrow_mut() = Vec::new();
        return;
    }

    footer_count.set_text(&format!("{} apps", filtered_indices.len()));

    for (pos, &app_idx) in filtered_indices.iter().enumerate() {
        if let Some(app) = apps.get(app_idx) {
            let row = Box::new(Orientation::Horizontal, 12);
            row.add_css_class("launcher-item");
            row.set_valign(Align::Center);
            row.set_cursor_from_name(Some("pointer"));

            if pos == selected_idx {
                row.add_css_class("selected");
            }

            // App Icon
            let icon_img = if !app.icon.is_empty() {
                let img = Image::from_icon_name(&app.icon);
                img.set_pixel_size(28);
                img.add_css_class("launcher-item-icon");
                img
            } else {
                let img = Image::from_icon_name("application-x-executable");
                img.set_pixel_size(28);
                img.add_css_class("launcher-item-icon");
                img
            };
            row.append(&icon_img);

            // App details (Title + Comment)
            let text_box = Box::new(Orientation::Vertical, 1);
            text_box.set_hexpand(true);
            text_box.set_valign(Align::Center);

            let title_lbl = Label::new(Some(&app.name));
            title_lbl.add_css_class("launcher-item-title");
            title_lbl.set_halign(Align::Start);
            text_box.append(&title_lbl);

            if !app.comment.is_empty() {
                let desc_lbl = Label::new(Some(&app.comment));
                desc_lbl.add_css_class("launcher-item-desc");
                desc_lbl.set_halign(Align::Start);
                desc_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                text_box.append(&desc_lbl);
            }
            row.append(&text_box);

            // Launch shortcut indicator
            let enter_hint = Label::new(Some("󰌑"));
            enter_hint.add_css_class("launcher-enter-hint");
            row.append(&enter_hint);

            // Click to launch
            let app_cl = app.clone();
            let win_cl = window.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.connect_pressed(move |_, _, _, _| {
                let _ = apps::launch(&app_cl);
                win_cl.hide();
            });
            row.add_controller(gesture);

            results_box.append(&row);
            rows.push(row);
        }
    }

    *item_rows.borrow_mut() = rows;
}

/// Update CSS selection state on result items.
fn update_selection_highlight(rows: &[Box], selected_idx: usize) {
    for (i, row) in rows.iter().enumerate() {
        if i == selected_idx {
            row.add_css_class("selected");
        } else {
            row.remove_css_class("selected");
        }
    }
}
