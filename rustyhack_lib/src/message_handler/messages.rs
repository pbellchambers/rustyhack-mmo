use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::background_map::{AllMaps, AllMapsChunk};
use crate::ecs::components::{DisplayDetails, Position, Velocity};
use crate::ecs::player::Player;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerRequest {
    PlayerJoin(ClientDetails),
    PlayerLogout(ClientDetails),
    UpdateVelocity(VelocityMessage),
    GetChunkedAllMaps,
    Timeout(String),
    Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerJoined(Player),
    PlayerAlreadyOnline,
    AllMaps(AllMaps),
    AllMapsChunk(AllMapsChunk),
    AllMapsChunksComplete,
    UpdatePosition(Position),
    UpdateOtherEntities(EntityUpdates),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientDetails {
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
    pub monster_type_map: HashMap<String, String>,
}
