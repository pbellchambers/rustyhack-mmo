use crate::background_map::tiles::{Collidable, TilePosition};

#[derive(Clone, Copy)]
pub struct Wall {
    pub tile_position: TilePosition,
    pub character_icon: char,
    pub collidable: Collidable,
}

impl Wall {
    pub fn new(x: i32, y: i32, character_icon: char) -> Wall {
        Wall {
            tile_position: TilePosition { x, y },
            character_icon,
            collidable: Collidable::True,
        }
    }
}
