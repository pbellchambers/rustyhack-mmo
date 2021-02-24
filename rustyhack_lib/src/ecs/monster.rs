use crate::consts::{
    DEFAULT_MAP, DEFAULT_MONSTER_COLOUR, DEFAULT_MONSTER_ICON, DEFAULT_MONSTER_POSITION_X,
    DEFAULT_MONSTER_POSITION_Y, DEFAULT_MONSTER_TYPE,
};
use crate::ecs::components::{DisplayDetails, MonsterDetails, Position, Velocity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type AllMonsterDefinitions = HashMap<String, Monster>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Monster {
    pub monster_details: MonsterDetails,
    pub display_details: DisplayDetails,
    pub position: Position,
    pub velocity: Velocity,
}

impl Default for Monster {
    fn default() -> Self {
        Monster {
            monster_details: MonsterDetails {
                id: Uuid::new_v4(),
                monster_type: DEFAULT_MONSTER_TYPE.to_string(),
                spawn_position: Position {
                    x: DEFAULT_MONSTER_POSITION_X,
                    y: DEFAULT_MONSTER_POSITION_Y,
                    map: DEFAULT_MAP.to_string(),
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
                x: DEFAULT_MONSTER_POSITION_X,
                y: DEFAULT_MONSTER_POSITION_Y,
                map: DEFAULT_MAP.to_string(),
            },
            velocity: Velocity { x: 0, y: 0 },
        }
    }
}
