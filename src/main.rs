mod app;
mod compositor;
mod events;
mod style;
mod widgets;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "dev.freeyuh.shell";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| app::build(app));
    app.run();
}
