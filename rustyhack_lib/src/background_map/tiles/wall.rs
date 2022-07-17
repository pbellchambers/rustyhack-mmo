use crate::background_map::tiles::{Collidable, TilePosition};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Wall {
    pub tile_position: TilePosition,
    pub character_icon: char,
    pub collidable: Collidable,
}

impl Wall {
    pub fn new(x: isize, y: isize, character_icon: char) -> Wall {
        Wall {
            tile_position: TilePosition { x, y },
            character_icon,
            collidable: Collidable::True,
        }
    }
}
