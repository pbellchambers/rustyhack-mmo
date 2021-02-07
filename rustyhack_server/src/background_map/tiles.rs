pub(crate) mod door;
pub(crate) mod wall;

use crate::background_map::tiles::door::Door;
use crate::background_map::tiles::wall::Wall;

#[derive(Clone, Copy, PartialEq)]
pub enum Collidable {
    True,
    False,
}

#[derive(Clone, Copy)]
pub enum OpenState {
    Open,
    Closed,
}

#[derive(Clone, Copy)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub enum Tile {
    Wall(Wall),
    Door(Door),
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
            Tile::EmptySpace => ' ',
            Tile::Boundary => '#',
            Tile::NewLine => ' ',
            Tile::CarriageReturn => ' ',
            Tile::EndOfFile => ' ',
        }
    }
}
