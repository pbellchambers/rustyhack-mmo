use crate::background_map::tiles::{Collidable, OpenState, TilePosition};

#[derive(Clone, Copy)]
pub struct Door {
    pub tile_position: TilePosition,
    pub character_icon: char,
    pub collidable: Collidable,
    pub open_state: OpenState,
}

impl Door {
    pub fn new(x: i32, y: i32, open_state: OpenState) -> Door {
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
