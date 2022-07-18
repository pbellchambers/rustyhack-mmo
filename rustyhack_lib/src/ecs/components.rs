use crate::consts::{DEFAULT_PLAYER_COLOUR, DEFAULT_PLAYER_ICON};
use crate::ecs::monster::Monster;
use crate::ecs::player::Player;
use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    Monster(Monster),
    Player(Player),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub pos_x: u32,
    pub pos_y: u32,
    pub current_map: String,
    pub velocity_x: i32,
    pub velocity_y: i32,
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
    pub id: Uuid,
    pub player_name: String,
    pub client_addr: String,
    pub currently_online: bool,
    pub level: u32,
    pub exp: u32,
    pub gold: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterDetails {
    pub id: Uuid,
    pub monster_type: String,
    pub spawn_position: Position,
    pub is_active: bool,
    pub current_target: Option<String>,
    pub exp: u32,
    pub gold: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub update_available: bool,
    pub current_hp: f32,
    pub max_hp: f32,
    pub str: f32,
    pub dex: f32,
    pub con: f32,
    pub armour: f32,
}
