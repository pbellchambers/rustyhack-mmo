use crate::consts::{
    DEFAULT_MAP, DEFAULT_MONSTER_COLOUR, DEFAULT_MONSTER_ICON, DEFAULT_MONSTER_POSITION_X,
    DEFAULT_MONSTER_POSITION_Y, DEFAULT_MONSTER_TYPE,
};
use crate::ecs::components::{DisplayDetails, Inventory, MonsterDetails, Position, Stats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type AllMonsterDefinitions = HashMap<String, Monster>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Monster {
    pub monster_details: MonsterDetails,
    pub display_details: DisplayDetails,
    pub position: Position,
    pub stats: Stats,
    pub inventory: Inventory,
}

impl Default for Monster {
    fn default() -> Self {
        Monster {
            monster_details: MonsterDetails {
                id: Uuid::new_v4(),
                monster_type: DEFAULT_MONSTER_TYPE.to_string(),
                spawn_position: Position {
                    update_available: false,
                    pos_x: DEFAULT_MONSTER_POSITION_X,
                    pos_y: DEFAULT_MONSTER_POSITION_Y,
                    current_map: DEFAULT_MAP.to_string(),
                    velocity_x: 0,
                    velocity_y: 0,
                },
                is_active: false,
                current_target: None,
            },
            display_details: DisplayDetails {
                icon: DEFAULT_MONSTER_ICON,
                colour: DEFAULT_MONSTER_COLOUR,
                ..Default::default()
            },
            position: Position {
                update_available: false,
                pos_x: DEFAULT_MONSTER_POSITION_X,
                pos_y: DEFAULT_MONSTER_POSITION_Y,
                current_map: DEFAULT_MAP.to_string(),
                velocity_x: 0,
                velocity_y: 0,
            },
            stats: Stats {
                update_available: false,
                current_hp: 1.0,
                max_hp: 1.0,
                str: 1.0,
                dex: 1.0,
                con: 1.0,
                level: 1,
                exp: 0,
                exp_next: 1000,
            },
            inventory: Inventory::default(),
        }
    }
}
