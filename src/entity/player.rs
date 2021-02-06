use crate::entity::{Collidable, Location, Velocity};
use console_engine::Color;

#[derive(Clone, Copy)]
pub struct Player {
    pub location: Location,
    pub character_icon: char,
    pub collidable: Collidable,
    pub colour: Color,
    pub velocity: Velocity,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            location: Location { x, y },
            character_icon: '@',
            collidable: Collidable::True,
            colour: Color::Magenta,
            velocity: Velocity::Stationary,
        }
    }
}
