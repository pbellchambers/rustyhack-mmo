use rand::Rng;
use rustyhack_lib::ecs::components::{Inventory, Stats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Range;
use uuid::Uuid;

/*
Initial thoughts on combat stats...

Base stats:
HP - Flat figure
Weapon damage - Flat figure
Weapon speed - Flat figure
Base Accuracy% - Flat figure for all, 75%
Strength - Increases weapon damage by %
Dexterity - Increases accuracy / dodge / weapon speed / entity speed
Constitution - Increases HP, resistance to status effects
Armour - Reduces incoming damage by %

Calculated stats:
Damage dealt = Weapon damage * ((Str / 100) + 1)
Attack speed = Weapon speed * ((Dex / 100) + 1)
Actual Damage received = Damage dealt * (1 - (Armour / 100))
Accuracy% = Base accuracy + ((100 - base accuracy) * (Attacker's Dex / 100)) - ((100 - base accuracy) * (Defender's Dex / 100))

*/

pub(super) type CombatParties = HashMap<Attacker, Defender>;
pub(super) type CombatAttackerStats = HashMap<Uuid, (Stats, Inventory)>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub(crate) struct Attacker {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) client_addr: String,
    pub(crate) currently_online: bool,
    pub(crate) is_player: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub(crate) struct Defender {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) client_addr: String,
    pub(crate) currently_online: bool,
    pub(crate) is_player: bool,
}

impl Default for Defender {
    fn default() -> Self {
        Defender {
            id: Uuid::new_v4(),
            name: String::new(),
            client_addr: String::new(),
            currently_online: false,
            is_player: false,
        }
    }
}

pub(super) fn resolve_combat(
    attacker_stats: &Stats,
    attacker_inventory: &Inventory,
    defender_stats: &Stats,
    defender_inventory: &Inventory,
) -> f32 {
    debug!("Resolving combat...");
    if check_attack_success(
        attacker_stats.dex,
        attacker_inventory.equipped.weapon.accuracy,
        defender_stats.dex,
    ) {
        debug!("Attack hit...");
        let actual_damage_received = calculate_actual_damage_received(
            calculate_damage_dealt(
                &attacker_inventory.equipped.weapon.damage_range,
                attacker_stats.str,
            ),
            defender_inventory
                .equipped
                .armour
                .damage_reduction_percentage,
        );
        debug!("Damage taken: {}", actual_damage_received);
        actual_damage_received
    } else {
        debug!("Attack missed...");
        0.0
    }
}

fn calculate_damage_dealt(attacker_weapon_damage_range: &Range<f32>, attacker_str: f32) -> f32 {
    let mut rng = rand::rng();
    let attacker_weapon_damage = rng.random_range(attacker_weapon_damage_range.clone());
    debug!(
        "Weapon damage before strength modifier: {}",
        attacker_weapon_damage
    );
    attacker_weapon_damage * ((attacker_str / 100.0) + 1.0)
}

fn calculate_actual_damage_received(damage_dealt: f32, defender_armour: f32) -> f32 {
    damage_dealt * (1.0 - (defender_armour / 100.0))
}

fn check_attack_success(
    attacker_dex: f32,
    attacker_weapon_accuracy: f32,
    defender_dex: f32,
) -> bool {
    let mut rng = rand::rng();
    let combat_accuracy = attacker_weapon_accuracy
        + ((100.0 - attacker_weapon_accuracy) * (attacker_dex / 100.0))
        - ((100.0 - attacker_weapon_accuracy) * (defender_dex / 100.0));
    combat_accuracy >= rng.random_range(0.0..=100.0)
}
