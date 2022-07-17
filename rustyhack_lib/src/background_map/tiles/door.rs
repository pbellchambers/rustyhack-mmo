use crate::background_map::tiles::{Collidable, OpenState, TilePosition};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Door {
    pub tile_position: TilePosition,
    pub character_icon: char,
    pub collidable: Collidable,
    pub open_state: OpenState,
}

impl Door {
    pub fn new(x: isize, y: isize, open_state: OpenState) -> Door {
        Door {
            tile_position: TilePosition { x, y },
            character_icon: match open_state {
                OpenState::Open => '/',
                OpenState::Closed => '+',
            },
            collidable: match open_state {
                OpenState::Open => Collidable::False,
                OpenState::Closed => Collidable::True,
            },
            open_state,
        }
    }
}
