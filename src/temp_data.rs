use std::fs;

use crate::{CONFIG, MusicData};
use crate::utils::get_temp_music_file_path;

pub struct TempData {
    pub playlists: Vec<MusicData>,
}

impl Default for TempData {
    fn default() -> Self {
        Self::new()
    }
}

impl TempData {
    pub fn new() -> Self {
        Self {
            playlists: Self::get_playlists()
        }
    }

    fn get_playlists() -> Vec<MusicData> {
        let config = CONFIG.lock().unwrap();

        let data_dir = dirs::data_dir().unwrap();
        let data_dir = data_dir.as_os_str().to_str().unwrap();

        let temp_data_file_path = get_temp_music_file_path(config.music.as_ref());

        if temp_data_file_path.is_none() {
            return vec![];
        }

        let temp_data_file = format!("{}{}", data_dir, temp_data_file_path.unwrap());
        let data_file = fs::File::open(&temp_data_file);

        if data_file.is_err() {
            return vec![];
        }

        serde_json::from_reader::<_, Vec<MusicData>>(data_file.unwrap()).unwrap_or(vec![])
    }

    pub fn refresh(&mut self) {
        let playlists = Self::get_playlists();

        self.playlists = playlists;
    }
}
