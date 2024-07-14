# wtui

A [WireGuard](https://www.wireguard.com/) client interface.

## Features

- List Interfaces/Connections
- See connection status (connected/disconnected, bytes received/sent)
- Actions to connect & disconnect
- List most of configuration (address, MTU, endpoint, allowed IPs, DNS)

## TODO

- [ ] Copy actions (copy pubkey ...)
- [ ] Available action help popup

# Usage

To use, simply run `wtui`.  
You'll very likely need root permission to (1) read WireGuard configuration files and (2) alter
network interfaces and routes.

## Keymap

| Key         | Action     |
| ----------- | ---------- |
| `j`, `Down` | Down       |
| `k`, `Up`   | Up         |
| `c`         | Connect    |
| `d`         | Disconnect |
| `y`\*       | Yank menu  |
| `?`\*       | Help       |

\*: Soon, see TODO section

# Limitations/Caveats

- Works only with single peer configurations
- Unit tests require `wg` to be installed
