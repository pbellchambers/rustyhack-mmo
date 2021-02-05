use crate::entity::door::Door;
use crate::entity::wall::Wall;
use crate::entity::{Entity, OpenState};

pub fn map_character_to_entity(x: i32, y: i32, character: char) -> Entity {
    match character {
        '\n' => Entity::NewLine,
        '\r' => Entity::CarriageReturn,
        '%' => Entity::EndOfFile,
        '#' => Entity::Boundary,
        ' ' => Entity::EmptySpace,
        '|' => Entity::Wall(Wall::new(x, y, character)),
        '-' => Entity::Wall(Wall::new(x, y, character)),
        'x' => Entity::Door(Door::new(x, y, OpenState::Closed)),
        '+' => Entity::Door(Door::new(x, y, OpenState::Open)),
        ',' => Entity::Wall(Wall::new(x, y, character)),
        '*' => Entity::Wall(Wall::new(x, y, character)),
        _ => Entity::EmptySpace,
    }
}
