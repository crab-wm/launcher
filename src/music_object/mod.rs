use crate::crab_row::imp::CrabRowExt;
use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};
use crate::config::ConfigMusicService;
use crate::consts::*;

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

    pub fn get_url(&self) -> Option<String> {
        let music_data: &RefCell<MusicData> = self.imp().data.borrow();

        music_data.borrow().first_id.as_ref()?;

        match music_data.borrow().service {
            ConfigMusicService::Youtube => Some(
                MUSIC_YOUTUBE_URL
                    .replace("{VIDEO_ID}", &music_data.borrow().first_id.clone().unwrap())
                    .replace("{LIST_ID}", &music_data.borrow().id.clone())
            )
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
