mod app;
mod compositor;
mod events;
mod ipc;
mod services;
mod style;
mod widgets;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "dev.freeyuh.shell";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let first_arg = &args[1];
        if !first_arg.starts_with("--g") && !first_arg.starts_with("-g") {
            let cmd = args[1..].join(" ");
            match cmd.as_str() {
                "--help" | "-h" | "help" => {
                    println!("Freeyuh Desktop Shell Controller");
                    println!("Usage:");
                    println!("  freeyuh                             Launch shell daemon");
                    println!("  freeyuh toggle-launcher / run       Toggle Spotlight App Launcher");
                    println!("  freeyuh toggle-qs / toggle-quicksettings   Toggle Quick Settings / Control Center");
                    println!("  freeyuh toggle-notifs / notifs      Toggle Notification Center");
                    println!("  freeyuh reload / reload-style       Reload CSS style");
                    println!("  freeyuh mute / volume-mute          Toggle audio mute");
                    println!("  freeyuh volume-set <0-100>          Set audio volume level");
                    println!("  freeyuh wifi-toggle                 Toggle Wi-Fi on/off");
                    println!("  freeyuh bt-toggle                   Toggle Bluetooth on/off");
                    println!("  freeyuh brightness-set <1-100>      Set screen brightness percentage");
                    println!("  freeyuh brightness-up / down        Increase/decrease brightness (+-5%)");
                    return;
                }
                _ => {
                    match ipc::send_command(&cmd) {
                        Ok(resp) => {
                            if resp != "ok" {
                                println!("{resp}");
                            }
                        }
                        Err(e) => {
                            eprintln!("{e}");
                            std::process::exit(1);
                        }
                    }
                    return;
                }
            }
        }
    }

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| app::build(app));
    app.run_with_args::<String>(&[]);
}
