use std::{fs};
use std::error::Error;
use gtk::CssProvider;
use crate::consts::*;
use crate::utils::*;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct ConfigColors {
    bg: String,
    secondary_bg: String,
    text: String,
    secondary_text: String,
    accent: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    colors: ConfigColors,
    opacity: Option<f32>,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir().unwrap();
        let config_dir = config_dir.as_os_str().to_str().unwrap();

        let user_config_path = format!("{}{}", config_dir, CONFIG_USER_PATH);
        let default_config_path = format!("{}{}", config_dir, CONFIG_DEFAULT_PATH);

        let mut using_default_config = false;

        let mut config_file = fs::File::open(&user_config_path);

        if let Err(_) = config_file {
            config_file = fs::File::open(&default_config_path);

            if let Err(_) = config_file {
                display_err(ERROR_MISSING_CONFIG);
            }

            using_default_config = true;
        }

        let config = serde_yaml::from_reader::<_, Config>(config_file.unwrap());

        if let Err(_) = &config {
            if !using_default_config {
                let config_file = fs::File::open(&default_config_path);

                if let Err(_) = config_file {
                    display_err(ERROR_MISSING_CONFIG);
                }

                let config = serde_yaml::from_reader::<_, Config>(config_file.unwrap());

                if let Err(_) = config {
                    display_err(ERROR_BAD_CONFIG);
                }
            }

            display_err(ERROR_BAD_CONFIG);
        }

        config.unwrap()
    }

    pub fn apply(&self, provider: &CssProvider) {
        let mut opacity = self.opacity.unwrap_or(1.);

        opacity = if opacity < 0. || opacity > 1. { 1. } else { opacity };

        let style = format!("
            @define-color bg-color alpha({}, {});
            @define-color bg-secondary-color alpha({}, {});
            @define-color text-secondary-color {};
            @define-color text-color {};
            @define-color accent-color {};
        ", self.colors.bg, opacity, self.colors.secondary_bg, opacity, self.colors.secondary_text, self.colors.text, self.colors.accent);

        provider.load_from_data(&*[style.as_bytes(), include_bytes!("resources/style.css")].concat());
    }
}