use crate::consts::{BASE_COMBAT_ACCURACY, BASE_WEAPON_DAMAGE};
use crate::game::player_updates::send_message_to_player;
use crossbeam_channel::Sender;
use laminar::Packet;
use rand::prelude::*;
use rustyhack_lib::ecs::components::Stats;
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

pub(crate) type CombatParties = HashMap<Defender, Attacker>;
pub(crate) type CombatAttackerStats = HashMap<Uuid, Stats>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub(crate) struct Attacker {
    pub id: Uuid,
    pub name: String,
    pub client_addr: String,
    pub currently_online: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub(crate) struct Defender {
    pub id: Uuid,
    pub name: String,
    pub client_addr: String,
    pub currently_online: bool,
}

impl Default for Defender {
    fn default() -> Self {
        Defender {
            id: Uuid::new_v4(),
            name: "".to_string(),
            client_addr: "".to_string(),
            currently_online: false,
        }
    }
}

pub(crate) fn resolve_combat(attacker_stats: &Stats, defender_stats: &Stats) -> f32 {
    debug!("Resolving combat...");
    if check_attack_success(attacker_stats.dex, defender_stats.dex) {
        debug!("Attack hit...");
        let actual_damage_received = calculate_actual_damage_received(
            calculate_damage_dealt(BASE_WEAPON_DAMAGE, attacker_stats.str),
            defender_stats.armour,
        );
        debug!("Damage taken: {}", actual_damage_received);
        actual_damage_received
    } else {
        debug!("Attack missed...");
        0.0
    }
}

fn calculate_damage_dealt(attacker_weapon_damage_range: Range<f32>, attacker_str: f32) -> f32 {
    let mut rng = thread_rng();
    let attacker_weapon_damage = rng.gen_range(attacker_weapon_damage_range);
    debug!(
        "Weapon damage before strength modifier: {}",
        attacker_weapon_damage
    );
    attacker_weapon_damage * ((attacker_str / 100.0) + 1.0)
}

fn calculate_actual_damage_received(damage_dealt: f32, defender_armour: f32) -> f32 {
    damage_dealt * (1.0 - (defender_armour / 100.0))
}

fn check_attack_success(attacker_dex: f32, defender_dex: f32) -> bool {
    let mut rng = thread_rng();
    let combat_accuracy = BASE_COMBAT_ACCURACY
        + ((100.0 - BASE_COMBAT_ACCURACY) * (attacker_dex / 100.0))
        - ((100.0 - BASE_COMBAT_ACCURACY) * (defender_dex / 100.0));
    combat_accuracy >= rng.gen_range(0.0..=100.0)
}

pub(crate) fn send_combat_system_messages_to_players(
    defender: &Defender,
    attacker: &Attacker,
    damage: f32,
    current_hp: f32,
    sender: &Sender<Packet>,
) {
    debug!(
        "Sending combat message to players: {:?}, {:?}, {}",
        defender, attacker, damage
    );
    if damage > 0.0 {
        send_message_to_player(
            &defender.name,
            &defender.client_addr,
            defender.currently_online,
            &(attacker.name.to_string() + " hit you for " + &damage.to_string() + " damage."),
            sender,
        );
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You hit ".to_string() + &defender.name + " for " + &damage.to_string() + " damage."),
            sender,
        );
    } else {
        send_message_to_player(
            &defender.name,
            &defender.client_addr,
            defender.currently_online,
            &(attacker.name.to_string() + " missed their attack against you."),
            sender,
        );
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You missed your attack against ".to_string() + &defender.name + "."),
            sender,
        );
    }
    if current_hp <= 0.0 {
        send_message_to_player(
            &defender.name,
            &defender.client_addr,
            defender.currently_online,
            &(attacker.name.to_string() + " killed you."),
            sender,
        );
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You killed ".to_string() + &defender.name + "."),
            sender,
        );
    }
}
