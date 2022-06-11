use std::borrow::Borrow;
use std::cell::RefCell;
use std::process::exit;

use gtk::{CustomFilter, FilterListModel, gio, SingleSelection};
use gtk::gio::AppInfo;
use gtk::glib::{clone, MainContext};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::{DATA_MUSIC_SPOTIFY_TEMP_FILE, DATA_MUSIC_YOUTUBE_TEMP_FILE, MusicData, PLACEHOLDER_MUSIC, PLACEHOLDER_PROGRAMS, TEMP_DATA, Window};
use crate::config::{ConfigMusic, ConfigMusicService};
use crate::crab_tabs::imp::CrabTab;
use crate::music_object::MusicObject;

pub fn open_app(app_info: &AppInfo, window: &Window) {
    let context = gtk::Window::new().display().app_launch_context();

    window.hide();

    let commandline = app_info.commandline().unwrap();
    let main_context = MainContext::default();

    main_context.spawn_local(clone!(@strong commandline, @strong window, @strong app_info, @strong context => async move {
        if async_process::Command::new(commandline.as_os_str()).output().await.is_err() {
            if let Err(err) = app_info.launch(&[], Some(&context)) {
                gtk::MessageDialog::builder()
                    .text(&format!("Failed to start {}!", app_info.name()))
                    .secondary_text(&err.to_string())
                    .message_type(gtk::MessageType::Error)
                    .modal(true)
                    .transient_for(&window)
                    .build()
                    .show();
            }
        }

        window.hide_or_close();
    }));
}

pub fn display_err(message: &str) {
    println!("Error! {}", message);

    exit(1);
}

pub fn get_programs_model(window: &Window) -> (CustomFilter, SingleSelection) {
    window
        .imp()
        .entry
        .set_placeholder_text(Some(PLACEHOLDER_PROGRAMS));

    let model = gio::ListStore::new(AppInfo::static_type());
    AppInfo::all().iter().for_each(|app_info| {
        model.append(app_info);
    });

    window.imp().current_items.replace(Some(model));

    let filter = CustomFilter::new(clone!(@strong window => move |obj| {
        let crab_entry = obj.downcast_ref::<gio::AppInfo>().unwrap();
        let search = window.imp().entry.buffer().text();

        if !search.is_empty() {
            crab_entry
                .name()
                .to_lowercase()
                .contains(&search.as_str().to_lowercase()) || if crab_entry.description().is_some() {
                crab_entry.description().unwrap().to_lowercase().contains(&search.as_str().to_lowercase())
            } else {
                false
            }
        } else {
            true
        }
    }));

    let filter_model = FilterListModel::new(Some(&window.current_items()), Some(&filter));

    let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
        let app_info1 = obj1.downcast_ref::<AppInfo>().unwrap();
        let app_info2 = obj2.downcast_ref::<AppInfo>().unwrap();

        app_info1
            .name()
            .to_lowercase()
            .cmp(&app_info2.name().to_lowercase())
            .into()
    });

    let sorted_model = gtk::SortListModel::new(Some(&filter_model), Some(&sorter));
    let selection_model = SingleSelection::new(Some(&sorted_model));

    (filter, selection_model)
}

pub fn get_music_model(window: &Window) -> (CustomFilter, SingleSelection) {
    window
        .imp()
        .entry
        .set_placeholder_text(Some(PLACEHOLDER_MUSIC));

    let model = gio::ListStore::new(MusicObject::static_type());

    let temp_data = TEMP_DATA.lock().unwrap();

    temp_data
        .playlists
        .iter()
        .map(|music_data| {
            let music_object = MusicObject::new();
            music_object.imp().data.replace(music_data.clone());
            music_object
        })
        .for_each(|music_object| {
            model.append(&music_object);
        });

    window.imp().current_items.replace(Some(model));

    let filter = CustomFilter::new(clone!(@strong window => move |obj| {
        let music_object = obj.downcast_ref::<MusicObject>().unwrap();
        let music_object_data: &RefCell<MusicData> = music_object.imp().data.borrow();
        let search = window.imp().entry.buffer().text();

        if !search.is_empty() {
            music_object_data
                .borrow()
                .title
                .to_lowercase()
                .contains(&search.as_str().to_lowercase())
        } else {
            true
        }
    }));

    let filter_model = FilterListModel::new(Some(&window.current_items()), Some(&filter));

    let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
        let music_object1: &RefCell<MusicData> = obj1
            .downcast_ref::<MusicObject>()
            .unwrap()
            .imp()
            .data
            .borrow();
        let music_object2: &RefCell<MusicData> = obj2
            .downcast_ref::<MusicObject>()
            .unwrap()
            .imp()
            .data
            .borrow();

        music_object1
            .borrow()
            .title
            .to_lowercase()
            .cmp(&music_object2.borrow().title.to_lowercase())
            .into()
    });

    let sorted_model = gtk::SortListModel::new(Some(&filter_model), Some(&sorter));
    let selection_model = SingleSelection::new(Some(&sorted_model));

    (filter, selection_model)
}

pub fn setup_programs_model(window: &Window) -> (CustomFilter, SingleSelection) {
    get_programs_model(window)
}

pub fn setup_list_model(window: &Window, tab: &CrabTab) -> (CustomFilter, SingleSelection) {
    match tab {
        CrabTab::Programs => get_programs_model(window),
        CrabTab::Music => get_music_model(window),
    }
}

pub fn get_temp_music_file_path(config: Option<&ConfigMusic>) -> Option<String> {
    config?;

    match config.as_ref().unwrap().service {
        ConfigMusicService::Youtube => Some(DATA_MUSIC_YOUTUBE_TEMP_FILE.to_string()),
        ConfigMusicService::Spotify => Some(DATA_MUSIC_SPOTIFY_TEMP_FILE.to_string()),
    }
}
