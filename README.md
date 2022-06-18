# ![Logo](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/crab_launcher_big.svg)

## Crab Launcher
Crab Launcher is a simple and modern Crab app launcher written in Rust programming language.

## How to use it?
Download binary from [here](https://github.com/crab-wm/launcher/releases) and bind it in your favourite window manager!

After downloading the binary generate default config file and copy it to `~/.config/crab/launcher.yaml`.

To generate the config file, run `crab-launcher --generate-config`.

***NOTE: Make sure you have `crab-launcher` in your path.***

After generating config file you have two options:

- run standalone version: `crab-launcher --run`
- run daemon and show window if you want to: `crab-launcher --daemon`, `crab-launcher --show`

You made it!

### Daemon

If you want to run daemon, make sure you set it to autostart on your system. Then, to run launcher you will only need to
run `crab-launcher --show`.

## Music

Crab Launcher has the ability tu play playlists fetched from your YouTube or Spotify account. To enable this option,
choose your favourite music service and put it in `music.service` section inside of the config file.

To fetch playlists use `crab-launcher --fetch`. It's going to read config file and fetch playlists with given
informations and for the
given service. **You should run this at least once before running the launcher for the first time.**

### 1. Spotify

#### How to sign in?

After running the `crab-launcher --fetch` command, browser window will pop up and you will have to sign in to your
Spotify account. After signing in, you'll have to copy the url from your browser and paste it inside of the terminal
emulator.

#### How does it work?

Spotify will fetch your playlists every time you run the daemon service (unless the session finishes). If your playlists
are not up to date, it means you have to run `crab-launcher --fetch` command again.

### 2. YouTube

#### How to sign in?

After running the `crab-launcher --fetch` command, browser window will pop up and you will have to sign in to your
Youtube account. After signing in, you'll be prompted to close the window.

#### How does it work?

YouTube will fetch playlists only if you run the `crab-launcher --fetch` command. If your playlists are not up to date,
you have to run the command again.

## Screenshots

Here are some screenshots of this app.

|                                                    Default config                                                    |                                                  Light mode config                                                   |                                                     Color config                                                     |
|:--------------------------------------------------------------------------------------------------------------------:|:--------------------------------------------------------------------------------------------------------------------:|:--------------------------------------------------------------------------------------------------------------------:|
| ![Screenshot 1](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/screenshots/screenshot_3.png) | ![Screenshot 2](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/screenshots/screenshot_2.png) | ![Screenshot 3](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/screenshots/screenshot_1.png) |

## Troubleshooting

#### 1. My launcher will not launch! It shows me an error: "Could not find configuration file for the application. Make sure you have a `~/.config/crab/launcher_def.yaml` file."

- Copy and paste code from below into `~/.config/crab/launcher_def.yaml` and `~/.config/crab/launcher.yaml` file.

#### 2. My launcher will not launch! It shows me an error: "Could not read configuration file. Make sure you have a `~/.config/crab/launcher_def.yaml` file and it's properly formatted."

- Check syntax of your configuration file.

#### 3. My playlists are not the same as on my account.

- You have to update them manually (probably because user session has expired). To do this, run `crab-launcher --fetch`.

#### 4. My launcher does not read my config.

- You have to restart the daemon. To do this, run `killall crab-launcher && crab-launcher --daemon`.

## Default config

```yaml
# Sets colors for the launcher accordingly to their names
colors:
  bg: "#1E2128"
  secondary_bg: "#32363D"
  text: "#FFFFFF"
  secondary_text: "#989A9E"
  accent: "#62AEEF"

# Sets opacity for the entire app
opacity: 1.0

# Optional settings for the `music` section of the launcher
music:
  # Currently available options: `youtube`, `spotify`
  service: "youtube"
```

## Usage
This section contains all the available options for running `crab-launcher`.
- `--generate-config` - Generates configuration file and saves it in the default app directory. After finishing its
  work, it outputs the file location.
- `--refresh-config` - Reloads the configuration file into the app. Changes all the configured things while keeping
  daemon service running.
- `--fetch` - Generates temporary file containing all user's playlists for the selected service in config file. Make
  sure you fill in all the fields in config's music section.
- `--show` - Shows the launcher window. Will work only if daemon service is running in the background.
- `--run` - Runs the standalone version of the launcher. Startup time will be longer and playlists won't be fetched
  automatically (if config set up). To fetch them, use `--fetch` option before.
- `--daemon` - Runs the daemon service. App launched in background automatically fetches playlists (if config set up).
  You have to fetch playlists manually for the first time (`--fetch`), though. To show the window, use `--show` option.
- `--help` - Shows help.
