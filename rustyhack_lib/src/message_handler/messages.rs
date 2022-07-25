use crossterm::style::Color;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::background_map::{AllMaps, AllMapsChunk};
use crate::ecs::components::{Inventory, Position, Stats};
use crate::ecs::player::Player;

pub type EntityPositionBroadcast = HashMap<Uuid, (u32, u32, String, char, Color, String)>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerRequest {
    PlayerJoin(ClientDetails),
    PlayerLogout(ClientDetails),
    UpdateVelocity(PositionMessage),
    PickupItem(PositionMessage),
    DropItem(PositionMessage),
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
    UpdateStats(Stats),
    UpdateInventory(Inventory),
    UpdateOtherEntities((Uuid, (u32, u32, String, char, Color, String))),
    SystemMessage(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientDetails {
    pub client_addr: String,
    pub player_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionMessage {
    pub player_name: String,
    pub position: Position,
}
