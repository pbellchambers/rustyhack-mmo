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
    pub pos_x: usize,
    pub pos_y: usize,
    pub current_map: String,
    pub velocity_x: isize,
    pub velocity_y: isize,
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
    pub level: usize,
    pub exp: usize,
    pub gold: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterDetails {
    pub id: Uuid,
    pub monster_type: String,
    pub spawn_position: Position,
    pub is_active: bool,
    pub current_target: Option<String>,
    pub exp: usize,
    pub gold: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub current_hp: isize,
    pub max_hp: usize,
    pub str: usize,
    pub dex: usize,
    pub con: usize,
    pub armour: usize,
}
