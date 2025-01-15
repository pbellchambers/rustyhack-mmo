![Rustyhack Logo](https://github.com/pbellchambers/rustyhack-mmo/raw/main/assets/logo/rustyhack-logo.png "Rustyhack Logo")

# Rustyhack MMO
A barebones cross between an ASCII "roguelike" and MMORPG / MUD written in Rust. Lacking a lot of basic features. It currently has a client & server console program that allows a player to be created, move around, fight other players/monsters, pick up and drop things, look at things, and level up.

[![Build status](https://img.shields.io/github/actions/workflow/status/pbellchambers/rustyhack-mmo/main.yml?branch=main)](https://github.com/pbellchambers/rustyhack-mmo/actions)
[![Downloads](https://img.shields.io/github/downloads/pbellchambers/rustyhack-mmo/total)](https://github.com/pbellchambers/rustyhack-mmo/releases)
[![License](https://img.shields.io/github/license/pbellchambers/rustyhack-mmo)](https://github.com/pbellchambers/rustyhack-mmo/blob/main/LICENSE)


## Usage
1. Download the relevant OS version from [Releases](https://github.com/pbellchambers/rustyhack-mmo/releases)
2. Unzip
3. Run `rustyhack_server` from the command line
4. Run `rustyhack_client` from the command line
5. Connect client to server *(note: if you're running both locally, just accept the default address/ports, and it will autoconfigure)*
6. By default, the server will back up to `rustyhack_server_world_backup.json` every 60 seconds, and will attempt to load from this on start (if it exists)

## Controls
- Movement: ← ↑ → ↓ Arrow keys
- Combat: Move into enemy
- Commands:
  - L - Look around you
  - P - Pick up item underneath you
  - D - Drop item
  - U - increase stat points after level up
  - M - change map when standing on map exit location
- Quit: Ctrl-q

## Components
- **rustyhack_client** - contains all the client code
- **rustyhack_server** - contains all the server code
- **rustyhack_lib** - contains modules that are shared between both client and server
- **assets** - assets required for the server to run, note: assets directory must be in the same location as `rustyhack_server`

## Assets
Currently, the following functionality is defined entirely by text or json files located in the `assets` directory:
- **maps** - *.map plain-text* - Map definitions. All maps should be enclosed by a boundary of # characters in any shape, and end with a % character on the last line. See existing examples.
- **map_exits** - *.json* - Map exit locations, and where they lead.
- **monsters** - *.json* - Types of monsters, their stats and inventory etc.
- **spawns** - *.json* - Spawn locations of monsters. There should be one spawn file per map.

## Building from source
1. Install latest version of [rust](https://www.rust-lang.org/) (most recently confirmed working `1.84.0`)
2. Download this repository
3. Run `cargo build` in the repository root directory
4. Copy `assets` directory into `target/debug` directory
5. Run `rustyhack_server` and `rustyhack_client` from `target/debug` directory
