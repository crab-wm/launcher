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

    let config = Config::new().colors;

    let style = format!("
        @define-color bg-color {};
        @define-color bg-secondary-color {};
        @define-color text-secondary-color {};
        @define-color text-color {};
        @define-color accent-color {};
    ", config.bg, config.secondary_bg, config.secondary_text, config.text, config.accent);

    provider.load_from_data(&*[style.as_bytes(), include_bytes!("resources/style.css")].concat());

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
