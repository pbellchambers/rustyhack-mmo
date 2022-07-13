use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::background_map::{AllMaps, AllMapsChunk};
use crate::ecs::components::{DisplayDetails, Position, Velocity};
use crate::ecs::player::Player;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerMessage {
    PlayerJoin(CreatePlayerMessage),
    UpdateVelocity(VelocityMessage),
    GetAllMaps,
    GetChunkedAllMaps,
    Timeout(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerReply {
    PlayerJoined(Player),
    PlayerAlreadyOnline,
    AllMaps(AllMaps),
    AllMapsChunk(AllMapsChunk),
    AllMapsChunksComplete,
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
    pub position_updates: HashMap<String, Position>,
    pub display_details: HashMap<String, DisplayDetails>,
}
