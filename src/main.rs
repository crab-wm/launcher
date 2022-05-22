mod config;
mod consts;
mod crab_row;
mod crab_tabs;
mod utils;
mod window;

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gio, CssProvider, StyleContext};
use std::fs::File;
use std::io::Write;
use std::process::exit;

use crate::config::Config;
use crate::utils::display_err;
use consts::*;
use window::Window;

fn main() {
    let mut args = std::env::args();
    let arg = args.nth(1);

    if let Some(arg) = arg {
        match arg.as_str() {
            "--generate-config" => {
                let mut file = File::create(format!(
                    "{}{}",
                    dirs::config_dir().unwrap().as_os_str().to_str().unwrap(),
                    CONFIG_DEFAULT_PATH
                ))
                .unwrap();

                file.write_all(CONFIG_DEFAULT_STRING.as_bytes()).unwrap();

                println!("{}", CONFIG_GENERATED);
                exit(0);
            }
            a => display_err(format!("Uknown parameter: {}", a).as_str()),
        }
    }

    gio::resources_register_include!("crab-launcher.gresource").expect(ERROR_RESOURCES);

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run();
}

fn load_css() {
    let provider = CssProvider::new();

    Config::new().apply(&provider);

    StyleContext::add_provider_for_display(
        &Display::default().expect(ERROR_DISPLAY),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
