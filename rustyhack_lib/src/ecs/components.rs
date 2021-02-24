use crate::consts::{DEFAULT_PLAYER_COLOUR, DEFAULT_PLAYER_ICON};
use console_engine::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl Default for DisplayDetails {
    fn default() -> Self {
        DisplayDetails {
            icon: DEFAULT_PLAYER_ICON,
            colour: DEFAULT_PLAYER_COLOUR,
            visible: true,
            collidable: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerDetails {
    pub player_name: String,
    pub client_addr: String,
    pub currently_online: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterDetails {
    pub id: Uuid,
    pub monster_type: String,
    pub spawn_position: Position,
    pub is_active: bool,
    pub current_target: Option<String>,
}
