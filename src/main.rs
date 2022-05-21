mod consts;
mod crab_row;
mod window;
mod config;

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gio, CssProvider, StyleContext};

use consts::*;
use window::Window;
use crate::config::Config;

fn main() {
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
