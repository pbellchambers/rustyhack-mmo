# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### 🚀 Features and Improvements
- .

### 🐛 Bug Fixes
- .

### 🧰 Maintenance
- .

## [v0.3.2]
### 🚀 Features and Improvements
- player can now change maps when standing on map exit locations by pressing 'm'

### 🐛 Bug Fixes
- graphical glitch showing caret after window resize resolved by dependency update

### 🧰 Maintenance
- dependencies updated

## [v0.3.1]
### 🚀 Features and Improvements
- server now backs up the world every 60 seconds
- server will try to load from backup on start, else create a new world
- player can now choose which item to drop
- player can now choose which stats to increase on level up
- added messages for picking up and dropping items
- monster collision checking with other monsters now exists
- improved monster movement, wandering, and return to spawn behaviour
- monsters now change target if attacked
- entities can now be attacked by multiple entities in the same tick
- system messages now have colours
- health regen now only applies when out of combat

### 🐛 Bug Fixes
- don't send drop item request to server if player inventory empty
- don't send pickup item request to server if nothing to pickup
- prevent entities from occupying the same tile by accident
- stop monsters from following disconnected players
- fix players not disappearing when disconnecting
- combat can no longer occur after player has moved away from target
- fix potential server crash when calculating loop sleep duration
- client map download now via tcp to avoid issues with large data over unreliable connections on udp

### 🧰 Maintenance
- updated dependencies
- code tidying and more sensible module names
- server tick ecs systems streamlined for efficiency
- collision detection more efficient
- ecs systems and various iterators now run multithreaded where possible
- converted all nested vecs to ndimensional arrays for better performance

## [v0.3.0]
The combat, stats, levelling, and inventory update!
### 🚀 Features and Improvements
- PvE combat with monsters
- PvP combat with players
- player stats now actually used:
  - str increases damage
  - dex increases accuracy
  - con increases health and regen rate
- player sidebar now displays and updates values in realtime
- death and respawning for players and monsters
- exp gain, levelling up, and stat gains
- inventory system
  - picking up items
  - dropping items
- default weapons and armour (used in combat calculations)
- system messages and combat notifications displayed in bottom of client window
- look command changed to L key
- client window can now be dynamically resized
- a slightly larger default map with more variety

### 🐛 Bug Fixes
- gracefully logout without causing server errors (only if you use ctrl-q)
- look command now displays monster name correctly
- look command now shows underneath items in all situations
- no longer crashes when player enters top row of map

### 🧰 Maintenance
- clippy::cargo and clippy::pedantic warnings resolved
- network code efficiency improved (no more MaxPacketSize issues)
- CHANGELOG.md added
- dependencies updated
- link time optimisations enabled for release build
- a lot of code refactoring and tidying

## [v0.2.2] - 2022-07-13
### 🚀 Features and Improvements
- enable large map data

### 🧰 Maintenance
- updated dependencies
- refactoring and tidying of messaging code

## [v0.2.1] - 2021-11-21
### 🧰 Maintenance
- updated dependencies
- updated rust edition to 2021

## [v0.2.0] - 2021-02-27
The monsters update!
### 🚀 Features and Improvements
- initial implementation of monsters
- ability to load monsters and spawns from .json assets
- player and monster stats
- stat window and map location display
- implemented look command

### 🐛 Bug Fixes
- fixed bug with logged out player being collidable

### 🧰 Maintenance
- updated dependencies
- updated logo

## [v0.1.1] - 2021-02-17
### 🚀 Features and Improvements
- support multiple players
- connectivity improvements
- added --debug logging switch
- implement player rejoin & check if player exists
- bind to 0.0.0.0 by default for simplicity
- implemented collisions between player entities
- added DisplayDetails to entity updates

### 🧰 Maintenance
- better exception handling
- updated some dependencies
- refactored method locations into appropriate modules
- limited pub methods to pub(crate)

## [v0.1.0] - 2021-02-12
### 🚀 Features and Improvements
- implemented client/server

## [v0.1.0-alpha.3] - 2021-02-07
### 🚀 Features and Improvements
- use release-drafter.yml

## [v0.1.0-alpha.2] - 2021-02-05
### 🚀 Features and Improvements
- ability to load a map from a file

## [v0.1.0-alpha.1] - 2021-02-03
### 🚀 Features and Improvements
- first version of working code

[Unreleased]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.3.2...HEAD
[v0.3.2]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.2.2...v0.3.0
[v0.2.2]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.1.0-alpha.3...v0.1.0
[v0.1.0-alpha.3]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.1.0-alpha.2...v0.1.0-alpha.3
[v0.1.0-alpha.2]: https://github.com/pbellchambers/rustyhack-mmo/compare/v0.1.0-alpha.1...v0.1.0-alpha.2
[v0.1.0-alpha.1]: https://github.com/pbellchambers/rustyhack-mmo/releases/tag/v0.1.0-alpha.1