use crate::consts::{
    DEFAULT_MAP, DEFAULT_PLAYER_COLOUR, DEFAULT_PLAYER_ICON, DEFAULT_PLAYER_POSITION_X,
    DEFAULT_PLAYER_POSITION_Y,
};
use crate::ecs::components::{DisplayDetails, PlayerDetails, Position};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub player_details: PlayerDetails,
    pub display_details: DisplayDetails,
    pub position: Position,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            player_details: PlayerDetails {
                player_name: "".to_string(),
                client_addr: "".to_string(),
                currently_online: false,
            },
            display_details: DisplayDetails {
                icon: DEFAULT_PLAYER_ICON,
                colour: DEFAULT_PLAYER_COLOUR,
                visible: true,
                collidable: true,
            },
            position: Position {
                x: DEFAULT_PLAYER_POSITION_X,
                y: DEFAULT_PLAYER_POSITION_Y,
                map: DEFAULT_MAP.to_string(),
            },
        }
    }
}
