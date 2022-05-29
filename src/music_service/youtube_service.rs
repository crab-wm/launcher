use async_trait::async_trait;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use serde::Deserialize;

use crate::config::ConfigMusicService;
use crate::consts::*;
use crate::music_object::{MusicData, MusicObject};
use crate::music_service::MusicServiceExt;

#[derive(Debug, Deserialize)]
struct YoutubeApiPlaylistsListResponseItemSnippet {
    title: String,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiPlaylistsListResponseItem {
    id: String,
    snippet: YoutubeApiPlaylistsListResponseItemSnippet,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiPlaylistsListResponse {
    items: Vec<YoutubeApiPlaylistsListResponseItem>,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiPlaylistItemsListResponseItemSnippetResourceId {
    #[serde(rename(deserialize = "videoId"))]
    video_id: String,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiPlaylistItemsListResponseItemSnippet {
    #[serde(rename(deserialize = "resourceId"))]
    resource_id: YoutubeApiPlaylistItemsListResponseItemSnippetResourceId,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiPlaylistItemsListResponseItem {
    snippet: YoutubeApiPlaylistItemsListResponseItemSnippet,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiPlaylistItemsListResponse {
    items: Vec<YoutubeApiPlaylistItemsListResponseItem>,
}

pub struct YoutubeService {
    account_id: String,
    api_key: String,
}

impl YoutubeService {
    pub fn new(account_id: String, api_key: String) -> Self {
        Self {
            account_id,
            api_key,
        }
    }
}

#[async_trait(?Send)]
impl MusicServiceExt for YoutubeService {
    async fn get_all_playlists(&self) -> Vec<MusicObject> {
        let body: YoutubeApiPlaylistsListResponse = reqwest::get(format!(
            "{}{}&key={}",
            API_YOUTUBE_GET_PLAYLISTS_URL, self.account_id, self.api_key
        ))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        let mut playlists: Vec<MusicObject> = vec![];

        for item in body.items.iter() {
            let body: YoutubeApiPlaylistItemsListResponse = reqwest::get(format!(
                "{}{}&key={}",
                API_YOUTUBE_GET_PLAYLIST_ITEMS_URL,
                item.id.clone(),
                self.api_key
            ))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            let music_object = MusicObject::new();

            let first_item = body
                .items
                .get(0)
                .map(|item| item.snippet.resource_id.video_id.clone());

            music_object.imp().data.replace(MusicData {
                id: item.id.clone(),
                title: item.snippet.title.clone(),
                first_id: first_item.clone(),
                service: ConfigMusicService::Youtube,
            });

            playlists.push(music_object);
        }

        playlists
    }
}
