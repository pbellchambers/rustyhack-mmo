use crate::consts::{BASE_COMBAT_ACCURACY, BASE_WEAPON_DAMAGE};
use rand::prelude::*;
use rustyhack_lib::ecs::components::Stats;
use std::collections::HashMap;
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

pub(crate) type CombatParties = HashMap<Defender, Attacker>;
pub(crate) type Attacker = Uuid;
pub(crate) type Defender = Uuid;
pub(crate) type CombatAttackerStats = HashMap<Uuid, Stats>;

pub(crate) fn resolve_combat(attacker_stats: &Stats, defender_stats: &Stats) -> i32 {
    info!("Resolving combat...");
    if check_attack_success(attacker_stats.dex, defender_stats.dex) {
        info!("Attack hit...");
        //todo weapon damage currently static constant, make actual equipment with range
        let actual_damage_received = calculate_actual_damage_received(
            calculate_damage_dealt(BASE_WEAPON_DAMAGE, attacker_stats.str),
            defender_stats.armour,
        );
        info!("Damage taken: {}", actual_damage_received);
        actual_damage_received
    } else {
        info!("Attack missed...");
        0
    }
}

fn calculate_damage_dealt(attacker_weapon_damage: i32, attacker_str: i32) -> i32 {
    //todo make str modify weapon damage range with range of increase
    attacker_weapon_damage * ((attacker_str / 100) + 1)
}

fn calculate_actual_damage_received(damage_dealt: i32, defender_armour: i32) -> i32 {
    damage_dealt * (1 - (defender_armour / 100))
}

fn check_attack_success(attacker_dex: i32, defender_dex: i32) -> bool {
    let mut rng = thread_rng();
    let combat_accuracy = BASE_COMBAT_ACCURACY
        + ((100 - BASE_COMBAT_ACCURACY) * (attacker_dex / 100))
        - ((100 - BASE_COMBAT_ACCURACY) * (defender_dex / 100));
    combat_accuracy >= rng.gen_range(0..=100)
}
