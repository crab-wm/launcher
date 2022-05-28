#[macro_use]
extern crate dotenv_codegen;

mod config;
mod consts;
mod crab_row;
mod crab_tabs;
mod music_object;
mod utils;
mod window;
mod daemon;

use std::cell::RefCell;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gio, CssProvider, StyleContext};
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::rc::Rc;
use gtk::glib::{clone, MainContext, MainLoop, PRIORITY_DEFAULT, Receiver};
use sysinfo::{System, SystemExt};

use crate::config::Config;
use crate::utils::{display_err};
use crate::daemon::{CrabDaemonClient, CrabDaemonMethod, CrabDaemonServer};
use consts::*;
use window::Window;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let arg = args.nth(1);

    if let Some(arg) = arg {
        match arg.as_str() {
            "--generate-config" => generate_config(),
            "--show" => emit_show_window(),
            "--run" => run_standalone(),
            "--daemon" => run_daemon(),
            param => display_err(format!("Uknown parameter: {}", param).as_str()),
        }

        exit(0);
    }

    run_daemon();
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

fn build_ui(app: &Application, show_window: bool, rx: Option<Rc<RefCell<Option<Receiver<bool>>>>>) {
    let window = Window::new(app, rx.is_some());

    if let Some(rx) = rx {
        if let Some(rx) = rx.take() {
            rx.attach(None, clone!(@strong window => move |_| {
                window.present();
                window.clean_up();

                Continue(true)
            }));
        }
    }

    if show_window {
        window.present();
    }
}

fn emit_show_window() {
    let crab_daemon = CrabDaemonClient::new();
    crab_daemon.run_method(CrabDaemonMethod::ShowWindow);
}

fn generate_config() {
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

fn run_daemon() {
    let s = System::new_all();
    let is_running = s.processes_by_exact_name(APP_TITLE).count() > 1;

    if is_running {
        display_err(ERROR_DAEMON);
    }

    gio::resources_register_include!("crab-launcher.gresource").expect(ERROR_RESOURCES);

    let crab_daemon = CrabDaemonServer::new();
    let (tx, rx) = MainContext::channel::<bool>(PRIORITY_DEFAULT);
    let rx = Rc::new(RefCell::new(Some(rx)));

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| build_ui(app, false, Some(rx.clone())));

    let owner_id = crab_daemon.start(tx);

    app.run_with_args::<&str>(&[]);

    MainLoop::new(None, false).run();
    gio::bus_unown_name(owner_id);
}

fn run_standalone() {
    gio::resources_register_include!("crab-launcher.gresource").expect(ERROR_RESOURCES);

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(|app| build_ui(app, true, None));

    app.run_with_args::<&str>(&[]);
}