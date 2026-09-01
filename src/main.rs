use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const APP_ID: &str = "dev.freeyuh.shell";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Freeyuh")
        .build();

    // Turn this into a layer-shell surface
    window.init_layer_shell();
    window.set_layer(Layer::Top);

    // Anchor it to the top, stretched across the width
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    // Reserve screen space so windows don't overlap it
    window.set_exclusive_zone(32);

    let label = Label::new(Some("freeyuh! — you are free"));
    window.set_child(Some(&label));
    window.set_default_size(-1, 32);

    window.present();
}
