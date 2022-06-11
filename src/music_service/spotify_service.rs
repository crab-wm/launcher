//COMMAND TO RUN: spotify --uri="spotify:track:<TRACK>?context=spotify:playlist:<PLAYLIST>"

use std::default::Default;

use async_trait::async_trait;
use futures::future::join_all;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use rspotify::{AuthCodePkceSpotify, Config, Credentials, OAuth, scopes};
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
}

impl SpotifyService {
    pub fn new() -> Self {
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
            token_cached: true,
            token_refreshing: true,
            cache_path: format!("{}{}", data_dir, DATA_MUSIC_SPOTIFY_CACHE_FILE).parse().unwrap(),
            ..Default::default()
        };

        Self {
            auth: AuthCodePkceSpotify::with_config(credentials, oauth, config)
        }
    }
}

#[async_trait(? Send)]
impl MusicServiceExt for SpotifyService {
    async fn get_all_playlists(&mut self) -> Vec<MusicObject> {
        let url = self.auth.get_authorize_url(None).unwrap();
        self.auth.prompt_for_token(url.as_str()).await.unwrap();

        let playlists = self
            .auth
            .current_user_playlists_manual(Some(50), None)
            .await
            .unwrap();

        let playlists = playlists
            .items
            .iter()
            .map(|playlist| async {
                let items = self
                    .auth
                    .playlist_items_manual(&playlist.id, None, None, Some(1), None)
                    .await
                    .unwrap();

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

        let playlists = join_all(playlists).await;

        println!("{:#?}", playlists);

        playlists
    }
}