pub mod door;
pub mod wall;
use crate::background_map::tiles::door::Door;
use crate::background_map::tiles::wall::Wall;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Collidable {
    True,
    False,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OpenState {
    Open,
    Closed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TilePosition {
    pub x: isize,
    pub y: isize,
}

#[derive(Display, Clone, Copy, Debug, Serialize, Deserialize)]
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
    pub fn character(&self) -> char {
        match self {
            Tile::Wall(wall) => wall.character_icon,
            Tile::Door(door) => door.character_icon,
            Tile::UpLadder => '^',
            Tile::DownLadder => 'v',
            Tile::EmptySpace => ' ',
            Tile::Boundary => '#',
            Tile::NewLine => ' ',
            Tile::CarriageReturn => ' ',
            Tile::EndOfFile => ' ',
        }
    }
}
