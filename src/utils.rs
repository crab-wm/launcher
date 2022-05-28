use crate::config::ConfigMusicService;
use crate::crab_tabs::imp::CrabTab;
use crate::music_object::MusicObject;
use crate::{Config, Window, PLACEHOLDER_MUSIC, PLACEHOLDER_PROGRAMS};
use gtk::gio::{AppInfo};
use gtk::glib::{clone, MainContext};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, CustomFilter, FilterListModel, SingleSelection};
use std::process::exit;
use crate::music_service::*;
use crate::music_service::youtube_service::*;

pub fn open_app(app_info: &AppInfo, window: &Window) {
    let context = gtk::Window::new().display().app_launch_context();

    window.hide();

    let commandline = app_info.commandline().unwrap();
    let main_context = MainContext::default();

    main_context.spawn_local(clone!(@strong commandline, @strong window, @strong app_info, @strong context => async move {
        if let Err(_) = async_process::Command::new(commandline.as_os_str()).output().await {
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

pub async fn get_music_model(window: &Window) -> (CustomFilter, SingleSelection) {
    // TODO: Change `Config` to be singeleton
    let config = Config::new();
    let music_config = config.music.unwrap();

    window
        .imp()
        .entry
        .set_placeholder_text(Some(PLACEHOLDER_MUSIC));

    let model = gio::ListStore::new(MusicObject::static_type());
    get_all_user_playlists(music_config.account_id.as_str(), music_config.service)
        .await
        .iter()
        .for_each(|music_object| {
            model.append(music_object);
        });

    window.imp().current_items.replace(Some(model));

    let filter = CustomFilter::new(clone!(@strong window => move |obj| {
        let music_object = obj.downcast_ref::<MusicObject>().unwrap();
        let music_object_data = music_object.imp().data.borrow();
        let search = window.imp().entry.buffer().text();

        if !search.is_empty() {
            music_object_data
                .title
                .to_lowercase()
                .contains(&search.as_str().to_lowercase())
        } else {
            true
        }
    }));

    let filter_model = FilterListModel::new(Some(&window.current_items()), Some(&filter));

    let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
        let music_object1 = obj1.downcast_ref::<MusicObject>().unwrap();
        let music_object2 = obj2.downcast_ref::<MusicObject>().unwrap();

        music_object1
            .imp()
            .data
            .borrow()
            .title
            .to_lowercase()
            .cmp(&music_object2.imp().data.borrow().title.to_lowercase())
            .into()
    });

    let sorted_model = gtk::SortListModel::new(Some(&filter_model), Some(&sorter));
    let selection_model = SingleSelection::new(Some(&sorted_model));

    (filter, selection_model)
}

pub fn setup_programs_model(window: &Window) -> (CustomFilter, SingleSelection) {
    get_programs_model(window)
}

pub async fn setup_list_model_async(
    window: &Window,
    tab: &CrabTab,
) -> (CustomFilter, SingleSelection) {
    match tab {
        CrabTab::Programs => get_programs_model(window),
        CrabTab::Music => get_music_model(window).await,
    }
}

async fn get_all_user_playlists(user_id: &str, service: ConfigMusicService) -> Vec<MusicObject> {
    match service {
        ConfigMusicService::Youtube => {
            let youtube_service = YoutubeService::new(user_id.to_string());
            youtube_service.get_all_playlists().await
        },
    }
}