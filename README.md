# ![Logo](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/crab_launcher_big.svg)

## Crab Launcher
Crab Launcher is a simple and modern Crab app launcher whitten in Rust programming language.

## How to use it?
Download binary from [here](https://github.com/crab-wm/launcher/releases) and bind it in your favourite window manager!

After downloading the binary generate default config file and copy it to `~/.config/crab/launcher.yaml`.

To generate the config file, run `crab-launcher --generate-config`.

After generating config file you have two options:
- run standalone version: `crab-launcher --run`
- run daemon and show window if you want to: `crab-launcher --daemon`, `crab-launcher --show`

You made it!

## Screenshots
Here are some screenshots of this app.

|                                                    Default config                                                    |                                                  Light mode config                                                   |                                                     Color config                                                     |
|:--------------------------------------------------------------------------------------------------------------------:|:--------------------------------------------------------------------------------------------------------------------:|:--------------------------------------------------------------------------------------------------------------------:|
| ![Screenshot 1](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/screenshots/screenshot_3.png) | ![Screenshot 2](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/screenshots/screenshot_2.png) | ![Screenshot 3](https://raw.githubusercontent.com/crab-wm/launcher/main/.github/assets/screenshots/screenshot_1.png) |

## Troubleshooting
### My launcher will not launch! It shows me an error.
- ```Could not find configuration file for the application. Make sure you have a `~/.config/crab/launcher_def.yaml` file.```
Copy and paste code from below into `~/.config/crab/launcher_def.yaml` and `~/.config/crab/launcher.yaml` file.
- ```Could not read configuration file. Make sure you have a `~/.config/crab/launcher_def.yaml` file and it's properly formatted.```
Check syntax of your configuration file.

## Default config
```yaml
colors:
  bg: "#1E2128"
  secondary_bg: "#32363D"
  text: "#FFFFFF"
  secondary_text: "#989A9E"
  accent: "#62AEEF"

opacity: 1.0

music:
  account_id: ""
  service: "youtube"
  api_key: ""
```