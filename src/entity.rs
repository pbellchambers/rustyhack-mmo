use crate::entity::player::Player;

pub(crate) mod player;

pub enum Entity {
    Player(Player),
}

pub struct Location {
    pub x: i32,
    pub y: i32,
}
