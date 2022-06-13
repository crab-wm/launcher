use std::default::Default;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future::join_all;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use rspotify::{AuthCodePkceSpotify, ClientResult, Config, Credentials, OAuth, scopes};
use rspotify::clients::mutex::Mutex;
use rspotify::model::{Page, SimplifiedPlaylist};
use rspotify::prelude::{BaseClient, OAuthClient};

use crate::{ConfigMusicService, DATA_MUSIC_SPOTIFY_CACHE_FILE, MusicData, MusicServiceExt};
use crate::music_object::MusicObject;

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}

pub struct SpotifyService {
    auth: AuthCodePkceSpotify,
    should_force_fetch: bool
}

impl SpotifyService {
    pub fn new(should_force_fetch: bool) -> Self {
        Self {
            auth: Self::get_auth(true),
            should_force_fetch,
        }
    }

    pub fn get_auth(token_cached: bool) -> AuthCodePkceSpotify {
        let client_id = dotenv!("SPOTIFY_CLIENT_ID");
        let client_secret = dotenv!("SPOTIFY_CLIENT_SECRET");
        let redirect_uri = dotenv!("SPOTIFY_REDIRECT_URI");

        let credentials = Credentials::new(client_id, client_secret);

        let oauth = OAuth {
            scopes: scopes!("playlist-read-private", "playlist-read-collaborative"),
            redirect_uri: redirect_uri.to_string(),
            ..Default::default()
        };

        let data_dir = dirs::data_local_dir().unwrap();
        let data_dir = data_dir.to_str().unwrap();

        let config = Config {
            token_cached,
            token_refreshing: true,
            cache_path: format!("{}{}", data_dir, DATA_MUSIC_SPOTIFY_CACHE_FILE).parse().unwrap(),
            ..Default::default()
        };

        AuthCodePkceSpotify::with_config(credentials, oauth, config)
    }

    pub async fn regenerate_auth(&mut self) -> Result<(), ()> {
        let mut auth = SpotifyService::get_auth(false);

        let url = auth.get_authorize_url(None).unwrap();
        let token_request = auth.prompt_for_token(url.as_str()).await;

        if token_request.is_err() {
            return Err(());
        }

        auth.config.token_cached = true;
        auth.write_token_cache().await.unwrap();

        self.auth = auth;

        Ok(())
    }

    pub async fn get_playlists(&self) -> ClientResult<Page<SimplifiedPlaylist>> {
        self
            .auth
            .current_user_playlists_manual(Some(50), None)
            .await
    }
}

#[async_trait(? Send)]
impl MusicServiceExt for SpotifyService {
    async fn get_all_playlists(&mut self) -> Vec<MusicObject> {
        let url = self.auth.get_authorize_url(None).unwrap();
        let mut token_cache = self.auth.read_token_cache(false).await;
        let token_cache_error = token_cache.is_err() || token_cache.as_ref().unwrap_or(&None).is_none();

        if token_cache_error && !self.should_force_fetch { return vec![]; }

        if token_cache_error && self.should_force_fetch {
            let mut regen_request = self.regenerate_auth().await;

            while regen_request.is_err() {
                regen_request = self.regenerate_auth().await;
            };

            token_cache = self.auth.read_token_cache(false).await;
        }

        self.auth.token = Arc::new(Mutex::new(token_cache.unwrap_or(None)));

        let mut playlists = self.get_playlists().await;

        if playlists.is_err() {
            if !self.should_force_fetch { return vec![]; }

            let mut token_request = self.auth.prompt_for_token(url.as_str()).await;

            while token_request.is_err() {
                token_request = self.auth.prompt_for_token(url.as_str()).await;
            }

            playlists = self.get_playlists().await;

            if playlists.is_err() {
                let mut regen_request = self.regenerate_auth().await;

                while regen_request.is_err() {
                    regen_request = self.regenerate_auth().await;
                };

                playlists = self.get_playlists().await;
            }
        }

        if playlists.is_err() { return vec![]; }

        let playlists = playlists.unwrap();

        let playlists = playlists
            .items
            .iter()
            .map(|playlist| async {
                let items = self
                    .auth
                    .playlist_items_manual(&playlist.id, None, None, Some(1), None)
                    .await;

                if items.is_err() { return MusicObject::new(); }

                let items = items.unwrap();

                let first_id = items
                    .items
                    .first()
                    .map(|first_item|
                        first_item
                            .track
                            .as_ref()
                            .map(move |first_track| first_track.id())
                    );

                let first_id = first_id.flatten().flatten();

                let music_object = MusicObject::new();

                music_object.imp().data.replace(MusicData {
                    id: playlist.id.to_string(),
                    title: playlist.name.clone(),
                    first_id: first_id.map(|first_id| first_id.id().to_string()),
                    service: ConfigMusicService::Spotify,
                });

                music_object
            });

        join_all(playlists).await
    }
}