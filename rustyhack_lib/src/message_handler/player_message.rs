use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::background_map::AllMaps;
use crate::ecs::components::{Position, Velocity};
use crate::ecs::player::Player;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerMessage {
    PlayerJoin(CreatePlayerMessage),
    UpdateVelocity(VelocityMessage),
    GetAllMaps,
    Timeout(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerReply {
    PlayerJoined(Player),
    PlayerAlreadyOnline,
    AllMaps(AllMaps),
    UpdatePosition(Position),
    UpdateOtherEntities(EntityUpdates),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePlayerMessage {
    pub client_addr: String,
    pub player_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VelocityMessage {
    pub player_name: String,
    pub velocity: Velocity,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityUpdates {
    pub updates: HashMap<String, Position>,
}
