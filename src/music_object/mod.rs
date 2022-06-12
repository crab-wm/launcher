use std::borrow::Borrow;
use std::cell::RefCell;
use std::process::Command;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::ConfigMusicService;
use crate::consts::*;
use crate::crab_row::imp::CrabRowExt;

mod imp;

glib::wrapper! {
    pub struct MusicObject(ObjectSubclass<imp::MusicObject>);
}

impl Default for MusicObject {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicObject {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `MusicObject`.")
    }

    pub fn get_uri(&self) -> Option<String> {
        let music_data: &RefCell<MusicData> = self.imp().data.borrow();

        music_data.borrow().first_id.as_ref()?;

        match music_data.borrow().service {
            ConfigMusicService::Youtube => Some(
                MUSIC_YOUTUBE_URL
                    .replace("{VIDEO_ID}", &music_data.borrow().first_id.clone().unwrap())
                    .replace("{LIST_ID}", &music_data.borrow().id.clone()),
            ),
            ConfigMusicService::Spotify => Some(
                MUSIC_SPOFITY_URL
                    .replace("{TRACK_ID}", &music_data.borrow().first_id.clone().unwrap())
                    .replace("{LIST_ID}", &music_data.borrow().id.clone()),
            ),
        }
    }

    pub fn start_playing(&self) {
        let music_data: &RefCell<MusicData> = self.imp().data.borrow();
        let action_uri = self.get_uri().unwrap();

        match music_data.borrow().service {
            ConfigMusicService::Youtube => {
                Command::new("xdg-open").arg(action_uri).spawn().unwrap();
            },
            ConfigMusicService::Spotify => {
                Command::new("spotify").arg(format!("--uri={}", &action_uri)).spawn().unwrap();
            },
        }
    }
}

impl CrabRowExt for MusicObject {
    fn get_name(&self) -> String {
        let music_data: &RefCell<MusicData> = self.imp().data.borrow();

        music_data.borrow().title.clone()
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MusicData {
    pub id: String,
    pub title: String,
    pub first_id: Option<String>,
    pub service: ConfigMusicService,
}
