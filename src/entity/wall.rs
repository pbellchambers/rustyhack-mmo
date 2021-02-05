use crate::entity::{Collidable, Location};

#[derive(Clone, Copy)]
pub struct Wall {
    pub location: Location,
    pub character_icon: char,
    pub collidable: Collidable,
}

impl Wall {
    pub fn new(x: i32, y: i32, character_icon: char) -> Wall {
        Wall {
            location: Location { x, y },
            character_icon,
            collidable: Collidable::True,
        }
    }
}
