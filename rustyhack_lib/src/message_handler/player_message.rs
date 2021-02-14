use crate::background_map::AllMaps;
use crate::ecs::components::{EntityName, Position, Velocity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerMessage {
    CreatePlayer(CreatePlayerMessage),
    UpdateVelocity(VelocityMessage),
    GetAllMaps,
    Heartbeat,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerReply {
    PlayerCreated,
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
    pub client_addr: String,
    pub player_name: String,
    pub velocity: Velocity,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityUpdates {
    pub updates: HashMap<EntityName, Position>,
}
