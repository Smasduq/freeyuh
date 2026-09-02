mod app;
mod compositor;
mod events;
mod ipc;
mod services;
mod style;
mod widgets;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "dev.ifreeyuh.shell";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let first_arg = &args[1];
        if !first_arg.starts_with("--g") && !first_arg.starts_with("-g") {
            let cmd = args[1..].join(" ");
            match cmd.as_str() {
                "--help" | "-h" | "help" => {
                    println!("iFreeYuh Desktop Shell Controller");
                    println!("Usage:");
                    println!("  ifreeyuh                             Launch shell daemon");
                    println!("  ifreeyuh toggle-launcher / run       Toggle Spotlight App Launcher");
                    println!("  ifreeyuh toggle-powermenu / power    Toggle Power & Session Menu");
                    println!("  ifreeyuh toggle-qs / toggle-quicksettings   Toggle Quick Settings / Control Center");
                    println!("  ifreeyuh toggle-notifs / notifs      Toggle Notification Center");
                    println!("  ifreeyuh reload / reload-style       Reload CSS style");
                    println!("  ifreeyuh mute / volume-mute          Toggle audio mute");
                    println!("  ifreeyuh volume-set <0-100>          Set audio volume level");
                    println!("  ifreeyuh wifi-toggle                 Toggle Wi-Fi on/off");
                    println!("  ifreeyuh bt-toggle                   Toggle Bluetooth on/off");
                    println!("  ifreeyuh brightness-set <1-100>      Set screen brightness percentage");
                    println!("  ifreeyuh brightness-up / down        Increase/decrease brightness (+-5%)");
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
