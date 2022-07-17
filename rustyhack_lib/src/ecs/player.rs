use crate::consts::{DEFAULT_MAP, DEFAULT_PLAYER_POSITION_X, DEFAULT_PLAYER_POSITION_Y};
use crate::ecs::components::{DisplayDetails, PlayerDetails, Position, Stats};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub player_details: PlayerDetails,
    pub display_details: DisplayDetails,
    pub position: Position,
    pub stats: Stats,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            player_details: PlayerDetails {
                player_name: "".to_string(),
                client_addr: "".to_string(),
                currently_online: false,
                level: 1,
                exp: 0,
                gold: 0,
            },
            display_details: DisplayDetails::default(),
            position: Position {
                pos_x: DEFAULT_PLAYER_POSITION_X,
                pos_y: DEFAULT_PLAYER_POSITION_Y,
                current_map: DEFAULT_MAP.to_string(),
                velocity_x: 0,
                velocity_y: 0,
            },
            stats: Stats {
                current_hp: 50,
                max_hp: 50,
                str: 10,
                dex: 10,
                con: 10,
                armour: 5,
            },
        }
    }
}
