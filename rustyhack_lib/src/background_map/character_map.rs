use crate::background_map::tiles::door::Door;
use crate::background_map::tiles::wall::Wall;
use crate::background_map::tiles::{OpenState, Tile};

pub fn map_character_to_tile(x: isize, y: isize, character: char) -> Tile {
    match character {
        '\n' => Tile::NewLine,
        '\r' => Tile::CarriageReturn,
        '%' => Tile::EndOfFile,
        '#' => Tile::Boundary,
        ' ' => Tile::EmptySpace,
        '^' => Tile::UpLadder,
        'v' => Tile::DownLadder,
        '|' => Tile::Wall(Wall::new(x, y, character)),
        '-' => Tile::Wall(Wall::new(x, y, character)),
        '+' => Tile::Door(Door::new(x, y, OpenState::Closed)),
        '/' => Tile::Door(Door::new(x, y, OpenState::Open)),
        ',' => Tile::Wall(Wall::new(x, y, character)),
        '*' => Tile::Wall(Wall::new(x, y, character)),
        _ => Tile::EmptySpace,
    }
}
