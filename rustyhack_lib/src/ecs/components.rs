use crate::consts::{
    DEAD_ICON, DEAD_MAP, DEFAULT_MAP, DEFAULT_PLAYER_COLOUR, DEFAULT_PLAYER_ICON,
    DEFAULT_PLAYER_POSITION_X, DEFAULT_PLAYER_POSITION_Y,
};
use crate::ecs::inventory::Equipment;
use crate::ecs::item::Item;
use crate::ecs::monster::Monster;
use crate::ecs::player::Player;
use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    Monster(Monster),
    Player(Player),
    Item(Item),
}

pub trait Dead: Sized {
    fn dead() -> Self;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub update_available: bool,
    pub pos_x: u32,
    pub pos_y: u32,
    pub current_map: String,
    pub velocity_x: i32,
    pub velocity_y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            update_available: false,
            pos_x: DEFAULT_PLAYER_POSITION_X,
            pos_y: DEFAULT_PLAYER_POSITION_Y,
            current_map: DEFAULT_MAP.to_string(),
            velocity_x: 0,
            velocity_y: 0,
        }
    }
}

impl Dead for Position {
    fn dead() -> Self {
        Position {
            update_available: false,
            pos_x: 0,
            pos_y: 0,
            current_map: DEAD_MAP.to_string(),
            velocity_x: 0,
            velocity_y: 0,
        }
    }
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

impl Dead for DisplayDetails {
    fn dead() -> Self {
        DisplayDetails {
            icon: DEAD_ICON,
            colour: DEFAULT_PLAYER_COLOUR,
            visible: false,
            collidable: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerDetails {
    pub id: Uuid,
    pub player_name: String,
    pub client_addr: String,
    pub currently_online: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterDetails {
    pub id: Uuid,
    pub monster_type: String,
    pub spawn_position: Position,
    pub current_target: Option<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemDetails {
    pub id: Uuid,
    pub has_been_picked_up: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub update_available: bool,
    pub current_hp: f32,
    pub max_hp: f32,
    pub str: f32,
    pub dex: f32,
    pub con: f32,
    pub level: u32,
    pub exp: u32,
    pub exp_next: u32,
    pub in_combat: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Inventory {
    pub update_available: bool,
    pub gold: u32,
    pub equipped: Equipment,
    pub carried: Vec<Item>,
}
