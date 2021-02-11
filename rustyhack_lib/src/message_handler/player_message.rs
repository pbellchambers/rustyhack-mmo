use crate::ecs::components::{Velocity, Position};
use serde::{Deserialize, Serialize};
use crate::background_map::AllMaps;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerMessage {
    CreatePlayer(CreatePlayerMessage),
    UpdateVelocity(VelocityMessage),
    GetAllMaps,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerReply {
    PlayerCreated,
    AllMaps(AllMaps),
    UpdatePosition(Position),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePlayerMessage {
    pub client_addr: String,
    pub player_name: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VelocityMessage {
    pub client_addr: String,
    pub player_name: String,
    pub velocity: Velocity
}