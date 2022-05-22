pub const APP_ID: &str = "wm.crab.launcher";
pub const APP_TITLE: &str = "crab-launcher";

pub const CONFIG_USER_PATH: &str = "/crab/launcher.yaml";
pub const CONFIG_DEFAULT_PATH: &str = "/crab/launcher_def.yaml";

pub const ERROR_RESOURCES: &str = "Failed to register resources.";
pub const ERROR_DISPLAY: &str = "Could not connect to a display.";
pub const ERROR_ITEMS: &str = "Could not get current items.";
pub const ERROR_MISSING_CONFIG: &str = "Could not find configuration file for the application. Make sure you have a `~/.config/crab/launcher_def.yaml` file.";
pub const ERROR_BAD_CONFIG: &str = "Could not read configuration file. Make sure you have a `~/.config/crab/launcher_def.yaml` file and it's properly formatted.";

pub const KEY_UP_ARROW: u32 = 111;
pub const KEY_DOWN_ARROW: u32 = 116;
pub const KEY_ESC: u32 = 9;
pub const KEY_LEFT_SHIFT: u32 = 50;
pub const KEY_RIGHT_SHIFT: u32 = 62;
pub const KEY_ENTER: u32 = 36;
