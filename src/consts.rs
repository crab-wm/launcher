#[cfg(debug_assertions)]
pub const APP_ID: &str = "wm.crab.launcher-debug";

#[cfg(not(debug_assertions))]
pub const APP_ID: &str = "wm.crab.launcher";

pub const APP_TITLE: &str = "crab-launcher";

pub const DATA_DIR: &str = "/crab/launcher";
pub const DATA_HISTORY_TEMP_FILE: &str = "/crab/launcher/history";
pub const DATA_MUSIC_YOUTUBE_TEMP_FILE: &str = "/crab/launcher/youtube.temp";
pub const DATA_MUSIC_YOUTUBE_CACHE_FILE: &str = "/crab/launcher/.youtube_token_cache.json";
pub const DATA_MUSIC_SPOTIFY_TEMP_FILE: &str = "/crab/launcher/spotify.temp";
pub const DATA_MUSIC_SPOTIFY_CACHE_FILE: &str = "/crab/launcher/.spotify_token_cache.json";

#[cfg(not(debug_assertions))]
pub const CONFIG_USER_PATH: &str = "/crab/launcher.yaml";
#[cfg(not(debug_assertions))]
pub const CONFIG_DEFAULT_PATH: &str = "/crab/launcher_def.yaml";

#[cfg(debug_assertions)]
pub const CONFIG_USER_PATH: &str = "/crab/launcher_debug.yaml";
#[cfg(debug_assertions)]
pub const CONFIG_DEFAULT_PATH: &str = "/crab/launcher_def_debug.yaml";

pub const CONFIG_USER_DIR: &str = "/crab";
pub const CONFIG_DEFAULT_DIR: &str = "/crab";
pub const CONFIG_GENERATED: &str = "Config generated successfully! You can find it in `~/.config/crab/launcher_def.yaml`. Copy the file to `~/.config/crab/launcher.yaml` and edit it as you want!";
pub const CONFIG_DEFAULT_STRING: &str = "\
# Sets colors for the launcher accordingly to their names
colors:
  bg: \"#1E2128\"
  secondary_bg: \"#32363D\"
  text: \"#FFFFFF\"
  secondary_text: \"#989A9E\"
  accent: \"#62AEEF\"

# Sets opacity for the entire app
opacity: 1.0

# Optional settings for the `music` section of the launcher
music:
  # Currently available options: `youtube`, `spotify`
  service: \"youtube\"
";

pub const ERROR_DAEMON: &str =
    "You cannot run more than one daemon instance. Close all running services and try again.";
pub const ERROR_RESOURCES: &str = "Failed to register resources.";
pub const ERROR_DISPLAY: &str = "Could not connect to a display.";
pub const ERROR_ITEMS: &str = "Could not get current items.";
pub const ERROR_AUTH: &str = "Could not authenticate.";
pub const ERROR_MISSING_CONFIG: &str = "Could not find configuration file for the application. Make sure you have a `~/.config/crab/launcher_def.yaml` file. If you don't have it, run `crab-launcher --generate-config`";
pub const ERROR_BAD_CONFIG: &str = "Could not read configuration file. Make sure you have a `~/.config/crab/launcher_def.yaml` file and it's properly formatted.";

pub const PLAYLISTS_FETCHED: &str = "Fetched your playlists!";

pub const KEY_UP_ARROW: u32 = 111;
pub const KEY_DOWN_ARROW: u32 = 116;
pub const KEY_ESC: u32 = 9;
pub const KEY_LEFT_SHIFT: u32 = 50;
pub const KEY_RIGHT_SHIFT: u32 = 62;
pub const KEY_ENTER: u32 = 36;
pub const KEY_TAB: u32 = 23;

pub const MAX_CHARS_IN_ROW: usize = 32;

pub const PLACEHOLDER_PROGRAMS: &str = "Search for an app...";
pub const PLACEHOLDER_MUSIC: &str = "Search for a playlist...";

pub const MUSIC_YOUTUBE_URL: &str =
    "https://www.youtube.com/watch?v={TRACK_ID}&list={LIST_ID}";
pub const MUSIC_SPOFITY_URL: &str =
    "spotify:track:{TRACK_ID}?context={LIST_ID}";

#[cfg(debug_assertions)]
pub const DBUS_SESSION_NAME: &str = "wm.crab.GDBus.LauncherServerDebug";
#[cfg(debug_assertions)]
pub const DBUS_OBJECT_PATH: &str = "/wm/crab/GDBus/LauncherObjectDebug";
#[cfg(debug_assertions)]
pub const DBUS_INTERFACE_NAME: &str = "wm.crab.GDBus.LauncherInterfaceDebug";

#[cfg(not(debug_assertions))]
pub const DBUS_SESSION_NAME: &str = "wm.crab.GDBus.LauncherServer";
#[cfg(not(debug_assertions))]
pub const DBUS_OBJECT_PATH: &str = "/wm/crab/GDBus/LauncherObject";
#[cfg(not(debug_assertions))]
pub const DBUS_INTERFACE_NAME: &str = "wm.crab.GDBus.LauncherInterface";
