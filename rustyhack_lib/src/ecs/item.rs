use crate::ecs::inventory::{Armour, Trinket, Weapon};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Armour(Armour),
    Gold(u32),
    Trinket(Trinket),
}

#[must_use]
pub fn get_item_name(item: &Item) -> String {
    match item {
        Item::Weapon(weapon) => weapon.name.clone(),
        Item::Armour(armour) => armour.name.clone(),
        Item::Gold(amount) => amount.to_string() + " Gold",
        Item::Trinket(trinket) => trinket.name.clone(),
    }
}
