use crate::music_object::MusicObject;
use async_trait::async_trait;

pub mod youtube_service;

#[async_trait(?Send)]
pub trait MusicServiceExt {
    async fn get_all_playlists(&self) -> Vec<MusicObject>;
}
