pub mod door;
pub mod wall;

use crate::background_map::tiles::door::Door;
use crate::background_map::tiles::wall::Wall;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Collidable {
    True,
    False,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum OpenState {
    Open,
    Closed,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct TilePosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Display, Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Wall(Wall),
    Door(Door),
    UpLadder,
    DownLadder,
    EmptySpace,
    Boundary,
    NewLine,
    CarriageReturn,
    EndOfFile,
}

impl Tile {
    #[must_use]
    pub fn character(&self) -> char {
        match self {
            Tile::Wall(wall) => wall.character_icon,
            Tile::Door(door) => door.character_icon,
            Tile::UpLadder => '<',
            Tile::DownLadder => '>',
            Tile::Boundary => '#',
            Tile::EmptySpace | Tile::NewLine | Tile::CarriageReturn | Tile::EndOfFile => ' ',
        }
    }
}
