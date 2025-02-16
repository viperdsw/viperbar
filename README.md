[![Open Source](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://opensource.org/)

# Viperbar

A blazingly (cause its the rust meme) utility bar for Wayland (and maybe not Wayland?) compositors.

## Docs:

This docs page will move one day to its own wiki thing but now it can't do much.

## Config

We support creating a custom config.

Must create a config in `~/.config/viperbar/viperbar.json`. For best results, I recommend installing and using a nerdfont on your system to display icons. However I do have plans to try and get other icon types (svg's) working.

Each of these keys are the built in modules. Eventually we will allow positional placements.

Example config:

```json
{
  "weather": {
    "api_key": "<YOUR_OPENWEATHERMAP_API_KEY>",
    "country_code": "UK", // This is the ISO 3166-1 alpha-2 standard, 2 letter country codes
    "city": "London"
  },
  "clock": {
    "format": "%d/%m%Y %H/%M/%S"
  },
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
