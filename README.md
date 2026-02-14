# wtui

A [WireGuard](https://www.wireguard.com/) client interface.

## Features

- List Interfaces/Connections
- See connection status (connected/disconnected, bytes received/sent)
- Actions to connect & disconnect
- List most of configuration (address, MTU, endpoint, allowed IPs, DNS)
- Copy public key

## TODO

- [ ] Available action help popup

# Usage

To use, simply run `wtui`.  
You'll very likely need root permission to (1) read WireGuard configuration files and (2) alter
network interfaces and routes.

See [Installation](#Installation) for simplified root usage.

## Keymap

| Key         | Action          |
| ----------- | --------------- |
| `j`, `Down` | Down            |
| `k`, `Up`   | Up              |
| `c`         | Connect         |
| `d`         | Disconnect      |
| `D`         | Disconnect all  |
| `y`         | Yank public key |
| `?`\*       | Help            |

\*: Soon, see TODO section

# Installation

1. Download from the [Releases](https://github.com/Valentin271/wtui/releases) page or build from source
2. Make binary executable `chmod +x wtui`
3. Move binary to PATH, e.g. `sudo mv wtui /usr/local/bin`
4. (optional) If `wtui` command is not available, add it to your `PATH`: `echo 'export PATH="$PATH:/usr/local/bin"' >> ~/.bashrc`

At this point, you can already use `wtui`, but you may want to enable root usage without password.

Use `visudo` to create a sudoers file:
`sudo visudo /etc/sudoers.d/wireguard`

Add the following lines, and replace `<user>` by your username.
```
<user> ALL= (root) NOPASSWD: /usr/bin/wg-quick
<user> ALL= (root) NOPASSWD: /usr/local/bin/wtui
```

## Keybind

After this setup, I recommend binding `wtui` to a shortcut.

For example, using i3wm & alacritty terminal.

```
bindsym $mod+grave exec alacritty -e sudo wtui
```

# Limitations/Caveats

- Works only with single peer configurations
- Unit tests require `wg` to be installed
