use crate::entity::Location;

pub struct Player {
    pub location: Location,
    pub character_icon: char,
}

impl Player {
    pub fn new() -> Player {
        Player {
            location: Location { x: 5, y: 5 },
            character_icon: '@',
        }
    }
}
