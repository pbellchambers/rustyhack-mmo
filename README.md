![Rustyhack Logo](https://github.com/pbellchambers/rustyhack-mmo/raw/main/assets/logo/rustyhack-logo.png "Rustyhack Logo")

# Rustyhack MMO
Partly a sandbox for me learning rust, partly an ASCII "roguelike" MMORPG. Lacking a lot of basic features. Currently has a client & server console program that allows a player to be created, move around and fight other players/monsters.

[![Build status](https://img.shields.io/github/workflow/status/pbellchambers/rustyhack-mmo/CI/main)](https://github.com/pbellchambers/rustyhack-mmo/actions)
[![Downloads](https://img.shields.io/github/downloads/pbellchambers/rustyhack-mmo/total)](https://github.com/pbellchambers/rustyhack-mmo/releases)
[![License](https://img.shields.io/github/license/pbellchambers/rustyhack-mmo)](https://github.com/pbellchambers/rustyhack-mmo/blob/main/LICENSE)


## Usage
1. Download the relevant OS version from [Releases](https://github.com/pbellchambers/rustyhack-mmo/releases)
2. Unzip
3. Run `rustyhack_server` from the command line
4. Run `rustyhack_client` from the command line
5. Connect client to server

Use arrow keys to move around, spacebar to look, ctrl-q to quit.

## Components
- **rustyhack_client** - contains all the client code
- **rustyhack_server** - contains all the server code
- **rustyhack_lib** - contains modules that are shared between both client and server
- **assets** - assets required for the server to run, note: assets directory must be in the same location as `rustyhack_server`
