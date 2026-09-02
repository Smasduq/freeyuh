//! Clock widget: centered date + time pill with a modern dashboard popover.
//!
//! Hovering the clock pill (or clicking it) opens a rich dashboard panel that
//! shows the current date and time, live weather, a calendar, and quick system
//! (CPU / memory) stats — the sort of at-a-glance panel modern desktop shells
//! provide. The panel closes automatically when the pointer leaves it.

use chrono::Local;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Calendar, EventControllerMotion, GestureClick,
    Label, LevelBar, Orientation,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::services::weather::Weather;
use crate::widgets::sysinfo;

const HIDE_DELAY: Duration = Duration::from_millis(250);

/// Handles to the live widgets inside the dashboard panel so the event reactor
/// can refresh them without rebuilding the whole panel.
#[derive(Clone)]
pub struct DashboardLabels {
    pub hero_time: Label,
    pub hero_date: Label,
    pub battery: Label,
    pub weather_icon: Label,
    pub temp: Label,
    pub condition: Label,
    pub feels: Label,
    pub location: Label,
    pub humidity: Label,
    pub wind: Label,
    pub cpu_pct: Label,
    pub cpu_bar: LevelBar,
    pub mem_pct: Label,
    pub mem_bar: LevelBar,
}

/// Creates the clock pill and its associated dashboard popup window.
///
/// Returns `(clock_pill, clock_label, dashboard_handles)`.
pub fn create(app: &Application) -> (Box, Label, DashboardLabels) {
    let container = Box::new(Orientation::Horizontal, 0);
    container.add_css_class("clock-pill");
    container.set_valign(Align::Center);
    container.set_halign(Align::Center);

    let label = Label::new(None);
    label.set_use_markup(true);
    label.add_css_class("clock-label");
    container.append(&label);

    // --- Dashboard Popup Window ---
    let dash_window = ApplicationWindow::builder().application(app).build();
    dash_window.init_layer_shell();
    dash_window.set_layer(Layer::Top);
    dash_window.set_margin(Edge::Top, 42);
    dash_window.set_keyboard_mode(KeyboardMode::None);
    dash_window.set_exclusive_zone(0);
    dash_window.add_css_class("dash-window");

    let dashboard = build_dashboard();
    dash_window.set_child(Some(&dashboard.root));

    let handles = DashboardLabels {
        hero_time: dashboard.hero_time.clone(),
        hero_date: dashboard.hero_date.clone(),
        battery: dashboard.battery.clone(),
        weather_icon: dashboard.weather_icon.clone(),
        temp: dashboard.temp.clone(),
        condition: dashboard.condition.clone(),
        feels: dashboard.feels.clone(),
        location: dashboard.location.clone(),
        humidity: dashboard.humidity.clone(),
        wind: dashboard.wind.clone(),
        cpu_pct: dashboard.cpu_pct.clone(),
        cpu_bar: dashboard.cpu_bar.clone(),
        mem_pct: dashboard.mem_pct.clone(),
        mem_bar: dashboard.mem_bar.clone(),
    };

    // Source handle for delayed hide on pointer leave.
    let hide_source = Rc::new(Cell::new(None::<glib::SourceId>));

    // Clock pill: show dashboard on hover, schedule hide on leave.
    let motion = EventControllerMotion::new();
    motion.connect_enter({
        let hide_source = hide_source.clone();
        let win = dash_window.clone();
        let dash = handles.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
            dash_window_refresh(&dash, 0);
            win.present();
        }
    });
    motion.connect_leave({
        let hide_source = hide_source.clone();
        let win = dash_window.clone();
        move |_| {
            schedule_hide(&hide_source, &win);
        }
    });
    container.add_controller(motion);

    // Dashboard window: cancel hide while hovered, hide on leave.
    let dash_motion = EventControllerMotion::new();
    dash_motion.connect_enter({
        let hide_source = hide_source.clone();
        move |_, _, _| {
            if let Some(source) = hide_source.take() {
                source.remove();
            }
        }
    });
    dash_motion.connect_leave({
        let hide_source = hide_source.clone();
        let win = dash_window.clone();
        move |_| {
            schedule_hide(&hide_source, &win);
        }
    });
    dashboard.root.add_controller(dash_motion);

    // Also support clicking to toggle.
    let gesture = GestureClick::new();
    gesture.set_button(1);
    let win = dash_window.clone();
    let dash = handles.clone();
    gesture.connect_pressed(move |_, _, _, _| {
        if win.is_visible() {
            win.hide();
        } else {
            dash_window_refresh(&dash, 0);
            win.present();
        }
    });
    container.add_controller(gesture);

    // Initial render of the dashboard hero/clock.
    dash_window_refresh(&handles, 0);

    (container, label, handles)
}

/// Schedule a delayed hide of `win`, storing the source handle in `slot`.
fn schedule_hide(slot: &Rc<Cell<Option<glib::SourceId>>>, win: &ApplicationWindow) {
    let slot_cb = slot.clone();
    let win = win.clone();
    let source = glib::timeout_add_local(HIDE_DELAY, move || {
        slot_cb.set(None);
        win.hide();
        glib::ControlFlow::Break
    });
    slot.set(Some(source));
}

struct Dashboard {
    root: Box,
    hero_time: Label,
    hero_date: Label,
    battery: Label,
    weather_icon: Label,
    temp: Label,
    condition: Label,
    feels: Label,
    location: Label,
    humidity: Label,
    wind: Label,
    cpu_pct: Label,
    cpu_bar: LevelBar,
    mem_pct: Label,
    mem_bar: LevelBar,
}

/// Build the full dashboard panel widget tree.
fn build_dashboard() -> Dashboard {
    let root = Box::new(Orientation::Vertical, 0);
    root.add_css_class("dash-dropdown");
    root.set_width_request(320);

    // ---------- Hero / Date & Time ----------
    let hero = Box::new(Orientation::Vertical, 2);
    hero.add_css_class("dash-card");
    hero.add_css_class("dash-hero");

    let hero_time = Label::new(None);
    hero_time.set_use_markup(true);
    hero_time.add_css_class("dash-hero-time");
    hero_time.set_xalign(0.0);

    let hero_date = Label::new(None);
    hero_date.set_use_markup(true);
    hero_date.add_css_class("dash-hero-date");
    hero_date.set_xalign(0.0);

    let meta_row = Box::new(Orientation::Horizontal, 8);
    meta_row.set_valign(Align::Center);

    let today_label = Label::new(None);
    today_label.set_use_markup(true);
    today_label.add_css_class("dash-hero-today");
    today_label.set_hexpand(true);
    today_label.set_xalign(0.0);

    let battery = Label::new(None);
    battery.set_use_markup(true);
    battery.add_css_class("dash-battery");
    battery.set_valign(Align::Center);

    meta_row.append(&today_label);
    meta_row.append(&battery);

    hero.append(&hero_time);
    hero.append(&hero_date);
    hero.append(&meta_row);

    // ---------- Weather Section ----------
    let weather_section = Box::new(Orientation::Horizontal, 14);
    weather_section.add_css_class("dash-card");

    let weather_icon = Label::new(None);
    weather_icon.set_use_markup(true);
    weather_icon.add_css_class("dash-weather-icon");
    weather_icon.set_valign(Align::Center);
    weather_icon.set_xalign(0.0);

    let weather_main = Box::new(Orientation::Vertical, 0);
    weather_main.set_hexpand(true);
    weather_main.set_valign(Align::Center);

    let weather_top = Box::new(Orientation::Horizontal, 8);
    let temp = Label::new(None);
    temp.set_use_markup(true);
    temp.add_css_class("dash-weather-temp");
    let condition = Label::new(None);
    condition.set_use_markup(true);
    condition.add_css_class("dash-weather-condition");
    condition.set_valign(Align::End);
    weather_top.append(&temp);
    weather_top.append(&condition);

    let feels = Label::new(None);
    feels.set_use_markup(true);
    feels.add_css_class("dash-weather-feels");
    feels.set_xalign(0.0);

    let location = Label::new(None);
    location.set_use_markup(true);
    location.add_css_class("dash-weather-location");
    location.set_xalign(0.0);

    weather_main.append(&weather_top);
    weather_main.append(&feels);
    weather_main.append(&location);

    let weather_extra = Box::new(Orientation::Vertical, 6);
    weather_extra.set_valign(Align::Center);
    let humidity;
    let wind;
    {
        let (h_box, h_lbl) = stat_pill();
        let (w_box, w_lbl) = stat_pill();
        humidity = h_lbl;
        wind = w_lbl;
        weather_extra.append(&h_box);
        weather_extra.append(&w_box);
    }

    weather_section.append(&weather_icon);
    weather_section.append(&weather_main);
    weather_section.append(&weather_extra);

    // ---------- Quick System Stats ----------
    let sys_section = Box::new(Orientation::Vertical, 10);
    sys_section.add_css_class("dash-card");
    sys_section.add_css_class("dash-sys");

    let sys_title = Label::new(Some("SYSTEM"));
    sys_title.add_css_class("dash-section-label");
    sys_title.set_xalign(0.0);
    sys_section.append(&sys_title);

    let (cpu_label, cpu_bar) = stat_bar();
    let (mem_label, mem_bar) = stat_bar();
    sys_section.append(&cpu_label);
    sys_section.append(&cpu_bar);
    sys_section.append(&mem_label);
    sys_section.append(&mem_bar);

    // ---------- Calendar ----------
    let cal = Calendar::new();
    cal.add_css_class("dash-calendar");
    let cal_frame = Box::new(Orientation::Vertical, 0);
    cal_frame.add_css_class("dash-card");
    cal_frame.append(&cal);

    // Assemble
    root.append(&hero);
    root.append(&weather_section);
    root.append(&sys_section);
    root.append(&cal_frame);

    // Initial placeholders
    hero_time.set_markup("<b>--:--</b>");
    hero_date.set_markup("");
    today_label.set_markup("");
    battery.set_markup("");

    Dashboard {
        root,
        hero_time,
        hero_date,
        battery,
        weather_icon,
        temp,
        condition,
        feels,
        location,
        humidity,
        wind,
        cpu_pct: cpu_label,
        cpu_bar,
        mem_pct: mem_label,
        mem_bar,
    }
}

fn stat_pill() -> (Box, Label) {
    let pill = Box::new(Orientation::Horizontal, 6);
    pill.add_css_class("dash-stat-pill");
    let i = Label::new(None);
    i.set_use_markup(true);
    i.add_css_class("dash-stat-icon");
    let v = Label::new(None);
    v.set_use_markup(true);
    v.add_css_class("dash-stat-value");
    v.set_markup("--");
    pill.append(&i);
    pill.append(&v);
    (pill, v)
}

/// Build a labeled level bar row. Returns `(label, bar)`.
fn stat_bar() -> (Label, LevelBar) {
    let label = Label::new(None);
    label.set_use_markup(true);
    label.add_css_class("dash-stat-label");
    label.set_xalign(0.0);

    let bar = LevelBar::new();
    bar.set_min_value(0.0);
    bar.set_max_value(1.0);
    bar.set_value(0.0);
    bar.add_css_class("dash-level");
    bar.set_size_request(0, 6);
    (label, bar)
}

fn battery_rep() -> (String, &'static str, u8) {
    match sysinfo::battery_state() {
        Some((pct, charging)) => {
            let icon = if charging {
                "󰂄".to_string()
            } else {
                sysinfo::battery_icon(pct).to_string()
            };
            let color = if charging {
                "#a4d1b4"
            } else if pct <= 15 {
                "#fa746f"
            } else if pct <= 30 {
                "#cec06b"
            } else {
                "#a3f1bd"
            };
            (icon, color, pct)
        }
        None => ("󰇅".to_string(), "#6e7870", 100),
    }
}

/// Pick a Nerd Font glyph for a current weather condition.
fn weather_icon(condition: &str) -> &'static str {
    let c = condition.to_lowercase();
    if c.contains("sunny") || c.contains("clear") {
        "󰖙"
    } else if c.contains("partly") {
        "󰖕"
    } else if c.contains("cloud") {
        "󰖐"
    } else if c.contains("rain") || c.contains("drizzle") || c.contains("shower") {
        "󰖖"
    } else if c.contains("thunder") {
        "󰖒"
    } else if c.contains("snow") || c.contains("sleet") {
        "󰙿"
    } else if c.contains("fog") || c.contains("mist") || c.contains("haze") {
        "󰖑"
    } else {
        "󰖐"
    }
}

/// Refresh the dashboard's live data. Call on [`Event::ClockTick`],
/// [`Event::SystemTick`], and [`Event::WeatherFetched`].
pub fn refresh(dash: &DashboardLabels, weather: Option<&Weather>) {
    dash_window_refresh(dash, 0);
    if let Some(w) = weather {
        apply_weather(dash, w);
    }
}

/// Refresh the hero clock + system bars.
fn dash_window_refresh(dash: &DashboardLabels, _opts: u8) {
    let now = Local::now();
    dash.hero_time
        .set_markup(&format!("<b>{}</b>", now.format("%H:%M")));
    dash.hero_date
        .set_markup(&format!("{}", now.format("%A, %B %e")));

    let (cpu, mem_used, mem_total) = sysinfo::sys_stats();
    let mem_pct = if mem_total > 0 {
        (mem_used as f64 / mem_total as f64) * 100.0
    } else {
        0.0
    };

    dash.cpu_pct
        .set_markup(&format!("<span color=\"#a4d1b4\">󰍛</span> CPU  <b>{cpu}%</b>"));
    dash.cpu_bar.set_value((cpu as f64).clamp(0.0, 100.0) / 100.0);

    let used_gb = mem_used as f64 / 1024.0;
    let total_gb = mem_total as f64 / 1024.0;
    dash.mem_pct.set_markup(&format!(
        "<span color=\"#7ad9bc\">󰘚</span> Memory  <b>{mem_pct:.0}%</b>  <span color=\"#6e7870\">{used_gb:.1}G/{total_gb:.1}G</span>"
    ));
    dash.mem_bar.set_value((mem_pct.clamp(0.0, 100.0)) / 100.0);

    let (bat_icon, bat_color, bat_pct) = battery_rep();
    dash.battery
        .set_markup(&format!("<span color=\"{bat_color}\">{bat_icon} {bat_pct}%</span>"));
}

fn apply_weather(dash: &DashboardLabels, w: &Weather) {
    let icon = weather_icon(&w.condition);
    dash.weather_icon.set_markup(&format!("<span font=\"38\">{icon}</span>"));

    if w.city.is_empty() {
        dash.location
            .set_markup("<span color=\"#8d9990\">󰀂 Weather</span>");
    } else {
        dash.location
            .set_markup(&format!("<span color=\"#8d9990\">󰀂 {}</span>", w.city));
    }
    dash.condition.set_text(&w.condition);

    dash.temp
        .set_markup(&format!("<b>{}</b>", w.temp_c.map(|t| format!("{t}°")).unwrap_or_else(|| "--".into())));

    dash.feels.set_markup(&format!(
        "<span color=\"#8d9990\">Feels like {}</span>",
        w.feels_c.map(|v| format!("{v}°")).unwrap_or_else(|| "--".into())
    ));

    dash.humidity.set_markup(&format!(
        "<span color=\"#8d9990\">󰜌</span> {}%",
        w.humidity.map(|v| v.to_string()).unwrap_or_else(|| "--".into())
    ));
    dash.wind.set_markup(&format!(
        "<span color=\"#8d9990\">󰖊</span> {} km/h",
        w.wind_kmh.map(|v| v.to_string()).unwrap_or_else(|| "--".into())
    ));
}

/// Updates the clock pill with the current date and time.
pub fn update(label: &Label) {
    let now = Local::now();
    let date_str = now.format("%a %b %d").to_string();
    let time_str = now.format("%H:%M").to_string();
    let text = format!(
        "<span color=\"#a4d1b4\">󰃭</span> <span color=\"#a4aea5\">{date_str}</span>   <span color=\"#a4d1b4\">󱑂</span> <span weight=\"bold\" color=\"#dee8df\">{time_str}</span>"
    );
    label.set_markup(&text);
    label.set_tooltip_text(Some(&now.format("%A, %B %e, %Y").to_string()));
}
