use std::fs;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::MusicData;
use crate::music_object::MusicObject;
use crate::utils::get_temp_music_file_path;

pub struct TempData {
    pub playlists: Vec<MusicObject>
}

impl TempData {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir().unwrap();
        let data_dir = data_dir.as_os_str().to_str().unwrap();

        let temp_data_file_path = get_temp_music_file_path();

        if temp_data_file_path.is_none() {
            return Self {
                playlists: vec![]
            };
        }

        let temp_data_file = format!("{}{}", data_dir, temp_data_file_path.unwrap());
        let data_file = fs::File::open(&temp_data_file);

        if data_file.is_err() {
            return Self {
                playlists: vec![]
            };
        }

        let playlists = serde_json::from_reader::<_, Vec<MusicData>>(data_file.unwrap());

        if playlists.is_err() {
            return Self {
                playlists: vec![]
            };
        }

        let playlists = playlists.unwrap().iter().map(|playlist| {
            let music_object = MusicObject::new();

            music_object.imp().data.replace(playlist.clone());

            music_object
        }).collect();

        Self {
            playlists
        }
    }
}