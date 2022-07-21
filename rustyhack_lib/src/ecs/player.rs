use crate::consts::{DEFAULT_MAP, DEFAULT_PLAYER_POSITION_X, DEFAULT_PLAYER_POSITION_Y};
use crate::ecs::components::{DisplayDetails, Inventory, PlayerDetails, Position, Stats};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub player_details: PlayerDetails,
    pub display_details: DisplayDetails,
    pub position: Position,
    pub stats: Stats,
    pub inventory: Inventory,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            player_details: PlayerDetails {
                id: Uuid::new_v4(),
                player_name: "".to_string(),
                client_addr: "".to_string(),
                currently_online: false,
                gold: 0,
            },
            display_details: DisplayDetails::default(),
            position: Position {
                update_available: false,
                pos_x: DEFAULT_PLAYER_POSITION_X,
                pos_y: DEFAULT_PLAYER_POSITION_Y,
                current_map: DEFAULT_MAP.to_string(),
                velocity_x: 0,
                velocity_y: 0,
            },
            stats: Stats {
                update_available: false,
                current_hp: 50.0,
                max_hp: 50.0,
                str: 10.0,
                dex: 10.0,
                con: 10.0,
                armour: 5.0,
                level: 1,
                exp: 0,
                exp_next: 1000,
            },
            inventory: Inventory::default(),
        }
    }
}
