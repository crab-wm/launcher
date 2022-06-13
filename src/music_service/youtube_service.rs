extern crate hyper;
extern crate hyper_rustls;

use std::fs;

use async_trait::async_trait;
use futures::future::join_all;
use google_youtube3::{Error, YouTube};
use google_youtube3::api::PlaylistListResponse;
use google_youtube3::oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use hyper::{Body, Client, Response};
use hyper_rustls::HttpsConnectorBuilder;

use crate::{ConfigMusicService, DATA_MUSIC_YOUTUBE_CACHE_FILE, display_err, ERROR_AUTH, MusicData};
use crate::music_object::MusicObject;
use crate::music_service::MusicServiceExt;

pub struct YoutubeService {
    hub: YouTube,
    should_force_fetch: bool,
}

impl YoutubeService {
    pub async fn new(should_force_fetch: bool) -> Self {
        Self {
            hub: Self::get_hub(false).await,
            should_force_fetch,
        }
    }

    async fn get_hub(remove_cache: bool) -> YouTube {
        let data_dir = dirs::data_local_dir().unwrap();
        let data_dir = data_dir.to_str().unwrap();

        let cache_path = format!("{}{}", data_dir, DATA_MUSIC_YOUTUBE_CACHE_FILE);

        if remove_cache {
            let _ = fs::remove_file(&cache_path);
        }

        let client_id = dotenv!("YOUTUBE_CLIENT_ID");
        let client_secret = dotenv!("YOUTUBE_CLIENT_SECRET");
        let auth_uri = dotenv!("YOUTUBE_AUTH_URI");
        let token_uri = dotenv!("YOUTUBE_TOKEN_URI");

        let secret = ApplicationSecret {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            auth_uri: auth_uri.into(),
            token_uri: token_uri.into(),
            redirect_uris: vec![],
            ..Default::default()
        };

        let auth = InstalledFlowAuthenticator::builder(
            secret,
            InstalledFlowReturnMethod::HTTPRedirect,
        ).persist_tokens_to_disk(cache_path).build().await;

        if auth.is_err() {
            display_err(ERROR_AUTH);
        }

        YouTube::new(
            Client::builder().build(
                HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build()
            ),
            auth.unwrap(),
        )
    }

    async fn get_playlists(&self) -> Result<(Response<Body>, PlaylistListResponse), Error> {
        self
            .hub
            .playlists()
            .list(&vec!["id".into(), "snippet".into()])
            .mine(true)
            .doit()
            .await
    }
}

#[async_trait(?Send)]
impl MusicServiceExt for YoutubeService {
    async fn get_all_playlists(&mut self) -> Vec<MusicObject> {
        let mut playlists = self.get_playlists().await;

        if playlists.is_err() {
            if !self.should_force_fetch {
                return vec![];
            }

            self.hub = Self::get_hub(true).await;
            playlists = self.get_playlists().await;
        }

        let (_, playlists) = playlists.unwrap();
        if playlists.items.is_none() {
            {
                dbg!("CCC");
                return vec![];
            }
        }
        let playlists = playlists.items.unwrap();

        let playlists = playlists.iter().map(|playlist| async {
            let playlist_items = self
                .hub
                .playlist_items()
                .list(&vec!["snippet".into()])
                .playlist_id(playlist.id.as_ref().unwrap().as_str())
                .doit()
                .await;

            if playlist_items.is_err() { return MusicObject::new(); }
            let (_, playlist_items) = playlist_items.unwrap();
            if playlist_items.items.is_none() { { return MusicObject::new(); } }

            let first_item = playlist_items.items.unwrap().first().map(|first_item|
                first_item.snippet.as_ref().unwrap().resource_id.as_ref().unwrap().video_id.as_ref().unwrap().clone()
            );

            let music_object = MusicObject::new();

            music_object.imp().data.replace(MusicData {
                id: playlist.id.as_ref().unwrap().clone(),
                title: playlist.snippet.as_ref().unwrap().title.as_ref().unwrap().clone(),
                first_id: first_item.clone(),
                service: ConfigMusicService::Youtube,
            });

            music_object
        });

        join_all(playlists).await
    }
}
