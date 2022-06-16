use std::fs;

use serde::{Deserialize, Serialize};

use crate::consts::*;
use crate::utils::*;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum ConfigMusicService {
    #[serde(rename = "youtube")]
    Youtube,
    #[serde(rename = "spotify")]
    Spotify,
}

impl Default for ConfigMusicService {
    fn default() -> Self {
        Self::Youtube
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ConfigMusic {
    pub service: ConfigMusicService,
}

#[derive(Deserialize, Debug, Default)]
pub struct ConfigColors {
    pub bg: String,
    pub secondary_bg: String,
    pub text: String,
    pub secondary_text: String,
    pub accent: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub colors: ConfigColors,
    pub opacity: Option<f32>,
    pub music: Option<ConfigMusic>,
}

impl Config {
    pub fn new() -> Self {
        Self::get_config()
    }

    pub fn refresh(&mut self) {
        let config = Self::get_config();

        self.colors = config.colors;
        self.music = config.music;
        self.opacity = config.opacity;
    }

    pub fn get_styles(&self) -> String {
        let mut opacity = self.opacity.unwrap_or(1.);

        opacity = if !(0...=1.).contains(&opacity) {
            1.
        } else {
            opacity
        };

        let style = format!(
            "
            @define-color bg-color alpha({}, {});
            @define-color bg-secondary-color alpha({}, {});
            @define-color text-secondary-color {};
            @define-color text-color {};
            @define-color accent-color {};
        ",
            self.colors.bg,
            opacity,
            self.colors.secondary_bg,
            opacity,
            self.colors.secondary_text,
            self.colors.text,
            self.colors.accent
        );

        style
    }

    fn get_config() -> Self {
        let config_dir = dirs::config_dir().unwrap();
        let config_dir = config_dir.as_os_str().to_str().unwrap();

        let user_config_path = format!("{}{}", config_dir, CONFIG_USER_PATH);
        let default_config_path = format!("{}{}", config_dir, CONFIG_DEFAULT_PATH);

        let mut using_default_config = false;

        let mut config_file = fs::File::open(&user_config_path);

        if config_file.is_err() {
            config_file = fs::File::open(&default_config_path);

            if config_file.is_err() {
                display_err(ERROR_MISSING_CONFIG);
            }

            using_default_config = true;
        }

        let config = serde_yaml::from_reader::<_, Config>(config_file.unwrap());

        if config.is_err() {
            if !using_default_config {
                let config_file = fs::File::open(&default_config_path);

                if config_file.is_err() {
                    display_err(ERROR_MISSING_CONFIG);
                }

                let config = serde_yaml::from_reader::<_, Config>(config_file.unwrap());

                if config.is_err() {
                    display_err(ERROR_BAD_CONFIG);
                }
            }

            display_err(ERROR_BAD_CONFIG);
        }

        config.unwrap()
    }
}
