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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayDetails {
    pub icon: char,
    pub colour: Color,
    pub visible: bool,
    pub collidable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerDetails {
    pub player_name: String,
    pub client_addr: String,
    pub currently_online: bool,
}
