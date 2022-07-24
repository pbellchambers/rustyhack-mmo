use crate::ecs::components::{DisplayDetails, Position};
use crate::ecs::inventory::{Armour, Trinket, Weapon};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Armour(Armour),
    Gold(u32),
    Trinket(Trinket),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemEntity {
    pub display_details: DisplayDetails,
    pub position: Position,
    pub item: Item,
}
