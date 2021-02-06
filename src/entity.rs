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
    CarriageReturn,
    EndOfFile,
}

impl Entity {
    pub fn character(&self) -> char {
        match self {
            Entity::Player(player) => player.character_icon,
            Entity::Wall(wall) => wall.character_icon,
            Entity::Door(door) => door.character_icon,
            Entity::EmptySpace => ' ',
            Entity::Boundary => '#',
            Entity::NewLine => ' ',
            Entity::CarriageReturn => ' ',
            Entity::EndOfFile => ' ',
        }
    }
}

#[derive(Clone, Copy)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Collidable {
    True,
    False,
}

#[derive(Clone, Copy)]
pub enum OpenState {
    Open,
    Closed,
}

#[derive(Clone, Copy)]
pub enum Velocity {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Stationary,
}
