use crate::background_map::tiles::{Collidable, TilePosition};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Wall {
    pub tile_position: TilePosition,
    pub character_icon: char,
    pub collidable: Collidable,
}

impl Wall {
    #[must_use]
    pub fn new(x: u32, y: u32, character_icon: char) -> Wall {
        Wall {
            tile_position: TilePosition { x, y },
            character_icon,
            collidable: Collidable::True,
        }
    }
}
