use console_engine::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenState {
    pub open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CollisionState {
    pub collidable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Character {
    pub icon: char,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityColour {
    pub colour: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsPlayer {
    pub is_player: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VisibleState {
    pub visible: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct EntityName {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientAddress {
    pub address: String,
}
