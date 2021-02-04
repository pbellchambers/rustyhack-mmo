use crate::entity::door::Door;
use crate::entity::player::Player;
use crate::entity::wall::Wall;

pub(crate) mod door;
pub(crate) mod player;
pub(crate) mod wall;

#[derive(Clone, Copy)]
pub enum Entity {
    Player(Player),
    Wall(Wall),
    Door(Door),
    EmptySpace,
    Boundary,
    NewLine,
    EndOfFile,
}

#[derive(Clone, Copy)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub enum Collidable {
    True,
    False,
}

#[derive(Clone, Copy)]
pub enum OpenState {
    Open,
    Closed,
}
