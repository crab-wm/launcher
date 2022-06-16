#[macro_use]
extern crate dotenv_codegen;

use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::rc::Rc;
use std::sync::Mutex;

use gtk::{CssProvider, gio, StyleContext};
use gtk::Application;
use gtk::gdk::Display;
use gtk::glib;
use gtk::glib::{clone, MainContext, MainLoop, PRIORITY_DEFAULT, Receiver};
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use once_cell::sync::Lazy;
use serde_json::json;
use sysinfo::{System, SystemExt};

use consts::*;
use window::Window;

use crate::config::{Config, ConfigMusicService};
use crate::daemon::{CrabDaemonClient, CrabDaemonMethod, CrabDaemonServer};
use crate::history::History;
use crate::music_object::MusicData;
use crate::music_service::MusicServiceExt;
use crate::music_service::spotify_service::SpotifyService;
use crate::music_service::youtube_service::YoutubeService;
use crate::temp_data::TempData;
use crate::utils::{display_err, get_temp_music_file_path};

mod config;
mod consts;
mod crab_row;
mod crab_tabs;
mod daemon;
mod history;
mod music_object;
mod music_service;
mod temp_data;
mod utils;
mod window;
mod song_info;

pub static HISTORY: Lazy<Mutex<History>> = Lazy::new(|| Mutex::new(History::new()));

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::new()));

pub static TEMP_DATA: Lazy<Mutex<TempData>> = Lazy::new(|| Mutex::new(TempData::new()));

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let arg = args.nth(1);

    if let Some(arg) = arg {
        match arg.as_str() {
            "--generate-config" => generate_config(),
            "--refresh-config" => emit_daemon_method(CrabDaemonMethod::RefreshConfig),
            "--fetch" => fetch_playlists(true).await,
            "--show" => emit_daemon_method(CrabDaemonMethod::ShowWindow),
            "--run" => run_standalone(),
            "--daemon" => run_daemon().await,
            "--help" => show_help(),
            param => {
                show_help();
                println!();
                display_err(format!("Uknown parameter: {}", param).as_str());
            }
        }

        exit(0);
    }

    show_help();
}

fn load_css() {
    let provider = CssProvider::new();

    let config = CONFIG.lock().unwrap();

    provider.load_from_data(
        &*[
            config.get_styles().as_bytes(),
            include_bytes!("resources/style.css"),
        ]
            .concat(),
    );

    StyleContext::add_provider_for_display(
        &Display::default().expect(ERROR_DISPLAY),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

async fn build_ui(
    app: &Application,
    show_window: bool,
    rx: Option<Rc<RefCell<Option<Receiver<CrabDaemonMethod>>>>>,
) {
    let window = Window::new(app, rx.is_some());

    if rx.is_some() {
        println!("Daemon started! Run with `--show` to show a window!");
    }

    if let Some(rx) = rx {
        if let Some(rx) = rx.take() {
            rx.attach(
                None,
                clone!(@strong window => move |event| {
                    let main_context = MainContext::default();

                    main_context.spawn_local(clone!(@strong window, @strong event => async move {
                       match event {
                           CrabDaemonMethod::ShowWindow => {
                               window.present();
                               window.clean_up();
                           }
                           CrabDaemonMethod::RefreshConfig => {
                               let mut config = CONFIG.lock().unwrap();
                               config.refresh();
                               drop(config);

                               load_css();

                               let mut temp_data = TEMP_DATA.lock().unwrap();
                               temp_data.refresh();
                               drop(temp_data);

                               fetch_playlists(false).await;
                           }
                       }
                   }));

                   Continue(true)
                }),
            );
        }
    }

    if show_window {
        window.present();
    }
}

fn emit_daemon_method(method: CrabDaemonMethod) {
    let crab_daemon = CrabDaemonClient::new();
    crab_daemon.run_method(method);
}

fn generate_config() {
    let config_dir = dirs::config_dir().unwrap();
    let config_dir = config_dir.as_os_str().to_str().unwrap();

    fs::create_dir_all(format!("{}{}", config_dir, CONFIG_USER_DIR)).unwrap();

    fs::create_dir_all(format!("{}{}", config_dir, CONFIG_DEFAULT_DIR)).unwrap();

    let mut file = File::create(format!("{}{}", config_dir, CONFIG_DEFAULT_PATH)).unwrap();

    file.write_all(CONFIG_DEFAULT_STRING.as_bytes()).unwrap();

    println!("{}", CONFIG_GENERATED);
    exit(0);
}

async fn run_daemon() {
    println!("Starting daemon...");

    fetch_playlists(false).await;

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
    let (tx, rx) = MainContext::channel::<CrabDaemonMethod>(PRIORITY_DEFAULT);
    let rx = Rc::new(RefCell::new(Some(rx)));

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| {
        let main_context = MainContext::default();

        main_context.spawn_local(clone!(@weak app, @weak rx => async move {
            build_ui(&app, false, Some(rx.clone())).await;
        }));
    });

    let owner_id = crab_daemon.start(tx);

    app.run_with_args::<&str>(&[]);

    MainLoop::new(None, false).run();
    gio::bus_unown_name(owner_id);
}

fn run_standalone() {
    gio::resources_register_include!("crab-launcher.gresource").expect(ERROR_RESOURCES);

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| {
        let main_context = MainContext::default();

        main_context.spawn_local(clone!(@weak app => async move {
            build_ui(&app, true, None).await;
        }));
    });

    app.run_with_args::<&str>(&[]);
}

async fn fetch_playlists(should_force_fetch_playlists: bool) {
    let data_dir = dirs::data_local_dir().unwrap();
    let data_dir = data_dir.to_str().unwrap();

    let service = {
        let config = CONFIG.lock().unwrap();

        if config.music.is_none() {
            return;
        }

        fs::create_dir_all(format!("{}{}", data_dir, DATA_DIR)).unwrap();

        let service = config.music.as_ref().unwrap().service.clone();

        // Do not fetch playlists if service does not support ignoring auth
        #[allow(clippy::single_match)]
        match (&service, should_force_fetch_playlists) {
            (ConfigMusicService::Youtube, false) => return,
            _ => {}
        }

        service
    };

    let mut service: Box<dyn MusicServiceExt> = match service {
        ConfigMusicService::Youtube => Box::new(YoutubeService::new().await),
        ConfigMusicService::Spotify => Box::new(SpotifyService::new(should_force_fetch_playlists)),
    };

    let playlists = service.get_all_playlists().await;
    let playlists = json!(playlists
        .iter()
        .map(|music_object| music_object.imp().data.take())
        .collect::<Vec<MusicData>>());

    let config = CONFIG.lock().unwrap();

    serde_json::to_writer(
        &File::create(format!(
            "{}{}",
            data_dir,
            get_temp_music_file_path(config.music.as_ref()).unwrap()
        ))
            .unwrap(),
        &playlists,
    )
        .unwrap();

    if should_force_fetch_playlists {
        println!("{}", PLAYLISTS_FETCHED);
    }
}

fn show_help() {
    let usage_commands_list = vec![
        ("--generate-config    ", "Generates configuration file and saves it in the default app directory. After finishing its work, it outputs the file location."),
        ("--refresh-config     ", "Reloads the configuration file into the app. Changes all the configured things while keeping daemon service running."),
        ("--fetch              ", "Generates temporary file containing all user's playlists for the selected service in config file. Make sure you fill in all the fields in config's music section."),
        ("--show               ", "Shows the launcher window. Will work only if daemon service is running in the background."),
        ("--run                ", "Runs the standalone version of the launcher. Startup time will be longer and playlists won't be fetched automatically (if config set up). To fetch them, use --fetch option before."),
        ("--daemon             ", "Runs the daemon service. App launched in background automatically fetches playlists (if config set up). You have to fetch playlists manually for the first time (`--fetch`), though. To show the window, use `--show` option."),
        ("--help               ", "Shows help."),
    ];

    println!("USAGE: crab-launcher <OPTION>");
    println!();
    println!("Available commands:");

    for (option, description) in usage_commands_list {
        println!("{} {}", option, description);
    }
}
