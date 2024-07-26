# Viperbar

A blazingly (cause its the rust meme) utility bar for Wayland (and maybe not Wayland?) compositors.

## Docs:

This docs page will move one day to its own wiki thing but now it can't do much.

## Config

We support creating a custom config, for now the config only accepts 1 key.

Must create a config in `~/.config/viperbar/viperbar.json`.

Example config:

```json
{
  "quickLaunch": [
    {
      "icon": " ",
      "tooltip": "Discord",
      "exec": "discord"
    },
    {
      "icon": "",
      "tooltip": "Terminal",
      "exec": "kitty"
    },
    {
      "icon": "",
      "tooltip": "Spotify",
      "exec": "spotify"
    },
    {
      "icon": "󰈹",
      "tooltip": "Firefox",
      "exec": "firefox"
    },
    {
      "icon": "󰨞",
      "tooltip": "VSCode",
      "exec": "code"
    },
    {
      "icon": "",
      "tooltip": "Steam",
      "exec": "steam"
    }
  ]
}
```

## Quick launch

Launches an app from Viperbar, can be customized through the config.
