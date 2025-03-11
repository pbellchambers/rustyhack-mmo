use bincode::{Decode, Encode};
use crossterm::style::Color;
use std::collections::HashMap;
use uuid::Uuid;

use crate::background_map::{AllMaps, AllMapsChunk};
use crate::ecs::components::{Inventory, Position, Stats};
use crate::ecs::player::Player;

pub type EntityPositionBroadcast = HashMap<Uuid, (u32, u32, String, char, Color, String)>;

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum PlayerRequest {
    PlayerJoin(ClientDetails),
    PlayerLogout(ClientDetails),
    UpdateVelocity(PositionMessage),
    PickupItem(PositionMessage),
    ChangeMap(PositionMessage),
    DropItem((u16, PositionMessage)),
    StatUp((String, String)),
    GetAllMaps,
    Timeout(String),
    Undefined,
}

#[derive(Debug, Encode, Decode)]
pub enum ServerMessage {
    PlayerJoined(Player),
    PlayerAlreadyOnline,
    AllMaps(AllMaps),
    AllMapsChunk(AllMapsChunk),
    AllMapsChunksComplete,
    UpdatePosition(Position),
    UpdateStats(Stats),
    UpdateInventory(Inventory),
    UpdateOtherEntities(#[bincode(with_serde)] (Uuid, (u32, u32, String, char, Color, String))),
    SystemMessage(SystemMessage),
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct ClientDetails {
    pub client_addr: String,
    pub player_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct PositionMessage {
    pub player_name: String,
    pub position: Position,
}

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub struct SystemMessage {
    pub message: String,
    #[bincode(with_serde)]
    pub colour: Option<Color>,
}
