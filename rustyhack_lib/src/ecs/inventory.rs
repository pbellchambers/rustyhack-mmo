use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode, Default)]
pub struct Equipment {
    pub weapon: Weapon,
    pub armour: Armour,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Weapon {
    pub name: String,
    pub damage_range: Range<f32>,
    pub accuracy: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            name: "Wooden Sword".to_string(),
            damage_range: 5.0..10.0,
            accuracy: 75.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Armour {
    pub name: String,
    pub damage_reduction_percentage: f32,
}

impl Default for Armour {
    fn default() -> Self {
        Armour {
            name: "Cloth Shirt".to_string(),
            damage_reduction_percentage: 5.0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Trinket {
    pub name: String,
}
