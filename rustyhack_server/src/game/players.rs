use rustyhack_lib::ecs::components::Position;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) type PlayersPositions = HashMap<Uuid, Position>;
