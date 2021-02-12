# rustyhack-mmo
Mostly a sandbox for learning rust. Currently produces a client & server console program that allows a player to move around a map with arrow keys.

[![Build status](https://img.shields.io/github/workflow/status/pbellchambers/rustybox/CI/main)](https://github.com/pbellchambers/rustybox/actions)
[![Downloads](https://img.shields.io/github/downloads/pbellchambers/rustybox/total)](https://github.com/pbellchambers/rustybox/releases)
[![License](https://img.shields.io/github/license/pbellchambers/rustyhack-mmo)](https://github.com/pbellchambers/rustybox/blob/main/LICENSE)


## Usage
1. Download the relevant OS version from [Releases](https://github.com/pbellchambers/rustybox/releases)
2. Unzip
3. Run `rustyhack_server` from the command line
4. Run `rustyhack_client` from the command line
5. Connect client to server

Use arrow keys to move around, ctrl-q to quit.

## Components
- **rustyhack_client** - contains all the client code
- **rustyhack_server** - contains all the server code
- **rustyhack_lib** - contains modules that are shared between both client and server
