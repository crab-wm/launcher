use std::fs;
use gtk::CssProvider;
use crate::consts::*;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ConfigColors {
    pub bg: String,
    pub secondary_bg: String,
    pub text: String,
    pub secondary_text: String,
    pub accent: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub colors: ConfigColors,
}

impl Config {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir = home_dir.as_os_str().to_str().unwrap();

        let user_config_path = format!("{}{}", home_dir, CONFIG_USER_PATH);
        let default_config_path = format!("{}{}", home_dir, CONFIG_DEFAULT_PATH);

        let mut using_default_config = false;

        let mut config_file = fs::File::open(&user_config_path);

        if let Err(_) = config_file {
            config_file = fs::File::open(&default_config_path);
            using_default_config = true;
        }

        let config_file = config_file.expect(ERROR_MISSING_CONFIG);

        let serde_parser = serde_yaml::from_reader::<_, Config>(config_file);

        if let Err(_) = &serde_parser {
            if !using_default_config {
                return serde_yaml::from_reader::<_, Config>(fs::File::open(&default_config_path).expect(ERROR_MISSING_CONFIG)).expect(ERROR_BAD_CONFIG);
            }

            panic!("{}", ERROR_BAD_CONFIG);
        }

        serde_parser.expect(ERROR_BAD_CONFIG)
    }

    pub fn apply(&self, provider: &CssProvider) {
        let style = format!("
            @define-color bg-color {};
            @define-color bg-secondary-color {};
            @define-color text-secondary-color {};
            @define-color text-color {};
            @define-color accent-color {};
        ", self.colors.bg, self.colors.secondary_bg, self.colors.secondary_text, self.colors.text, self.colors.accent);

        provider.load_from_data(&*[style.as_bytes(), include_bytes!("resources/style.css")].concat());
    }
}