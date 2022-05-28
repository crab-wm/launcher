mod config;
mod consts;
mod crab_row;
mod crab_tabs;
mod music_object;
mod utils;
mod window;
mod daemon;
mod music_service;
mod temp_data;

use serde_json::json;
use std::cell::RefCell;
use std::fs;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gio, CssProvider, StyleContext};
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::rc::Rc;
use std::sync::Mutex;
use gtk::glib::{clone, MainContext, MainLoop, PRIORITY_DEFAULT, Receiver};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use once_cell::sync::Lazy;
use sysinfo::{System, SystemExt};

use crate::config::Config;
use crate::utils::{display_err, get_temp_music_file_path};
use crate::daemon::{CrabDaemonClient, CrabDaemonMethod, CrabDaemonServer};
use consts::*;
use window::Window;
use crate::music_object::MusicData;
use crate::music_service::MusicServiceExt;
use crate::music_service::youtube_service::YoutubeService;
use crate::temp_data::TempData;

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::new()));

pub static TEMP_DATA: Lazy<Mutex<TempData>> = Lazy::new(|| Mutex::new(TempData::new()));

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let arg = args.nth(1);

    if let Some(arg) = arg {
        match arg.as_str() {
            "--generate-config" => generate_config(),
            "--fetch" => fetch_playlists().await,
            "--show" => emit_show_window(),
            "--run" => run_standalone(),
            "--daemon" => run_daemon().await,
            param => display_err(format!("Uknown parameter: {}", param).as_str()),
        }

        exit(0);
    }

    run_daemon().await;
}

fn load_css() {
    let provider = CssProvider::new();

    CONFIG.lock().unwrap().apply(&provider);

    StyleContext::add_provider_for_display(
        &Display::default().expect(ERROR_DISPLAY),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application, show_window: bool, rx: Option<Rc<RefCell<Option<Receiver<bool>>>>>) {
    let window = Window::new(app, rx.is_some());

    if rx.is_some() {
        println!("Daemon started! Run with `--show` to show a window!");
    }

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
    let config_dir = dirs::config_dir().unwrap();
    let config_dir = config_dir.as_os_str().to_str().unwrap();

    fs::create_dir_all(format!(
        "{}{}",
        config_dir,
        CONFIG_USER_DIR
    )).unwrap();

    fs::create_dir_all(format!(
        "{}{}",
        config_dir,
        CONFIG_DEFAULT_DIR
    )).unwrap();

    let mut file = File::create(format!(
        "{}{}",
        config_dir,
        CONFIG_DEFAULT_PATH
    ))
        .unwrap();

    file.write_all(CONFIG_DEFAULT_STRING.as_bytes()).unwrap();

    println!("{}", CONFIG_GENERATED);
    exit(0);
}

async fn run_daemon() {
    println!("Starting daemon...");

    fetch_playlists().await;

    let s = System::new_all();

    #[cfg(not(debug_assertions))]
    let max_processes = 1;

    #[cfg(debug_assertions)]
    let max_processes = 2;

    let is_running = s.processes_by_exact_name(APP_TITLE).count() > max_processes;

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

async fn fetch_playlists() {
    let config = CONFIG.lock().unwrap();

    if config.music.is_none() {
        display_err(ERROR_MUSIC_CONFIG);
    }

    let data_dir = dirs::data_local_dir().unwrap();
    let data_dir = data_dir.to_str().unwrap();

    fs::create_dir_all(format!(
        "{}{}",
        data_dir,
        DATA_DIR
    )).unwrap();


    let youtube_service = YoutubeService::new(config.music.as_ref().unwrap().account_id.clone(), config.music.as_ref().unwrap().api_key.clone());
    let playlists = youtube_service.get_all_playlists().await;
    let playlists = json!(playlists.iter().map(|music_object| music_object.imp().data.take()).collect::<Vec<MusicData>>());

    serde_json::to_writer(
        &File::create(format!("{}{}", data_dir, get_temp_music_file_path(config.music.as_ref()).unwrap())).unwrap(),
        &playlists
    ).unwrap();
}