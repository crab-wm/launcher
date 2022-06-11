use async_trait::async_trait;

use crate::music_object::MusicObject;

pub mod youtube_service;
pub mod spotify_service;

#[async_trait(? Send)]
pub trait MusicServiceExt {
    async fn get_all_playlists(&mut self) -> Vec<MusicObject>;
}
