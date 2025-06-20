# hypr-session
Simple script to launch apps from config file

## Usage
In your config directory create hypr/hypr-session.json file. 
Example of hypr-session.json:
```
[
  {
    "workspace": null,
    "name": "Spotify",
    "command": "spotify-launcher"
  },
  {
    "workspace": null,
    "name": "Telegram",
    "command": "telegram-desktop"
  },
  {
    "workspace": 2,
    "name": "Firefox",
    "command": "firefox"
  }
]
```
If you leave workspace as null, then it will be launched at workspace 1.
