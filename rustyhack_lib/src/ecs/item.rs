use crate::ecs::inventory::{Armour, Trinket, Weapon};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Armour(Armour),
    Gold(u32),
    Trinket(Trinket),
}
