use crate::game::combat::{Attacker, Defender};
use crate::network_messages::send_message_to_player;
use crossbeam_channel::Sender;
use crossterm::style::Color;
use laminar::Packet;

pub(crate) fn send_combat_updates_to_players(
    defender: &Defender,
    attacker: &Attacker,
    damage: f32,
    current_hp: f32,
    exp_gain: u32,
    gold_gain: u32,
    sender: &Sender<Packet>,
) {
    send_combat_messages_to_players(defender, attacker, damage, current_hp, sender);
    send_exp_messages_to_players(defender, attacker, current_hp, exp_gain, sender);
    send_gold_messages_to_players(defender, attacker, current_hp, gold_gain, sender);
}

pub(crate) fn send_gold_messages_to_players(
    defender: &Defender,
    attacker: &Attacker,
    current_hp: f32,
    gold_gain: u32,
    sender: &Sender<Packet>,
) {
    debug!(
        "Sending gold gain message to players: {:?}, {}",
        attacker, gold_gain
    );
    if current_hp <= 0.0 && gold_gain > 0 {
        if defender.is_player {
            send_message_to_player(
                &defender.name,
                &defender.client_addr,
                defender.currently_online,
                &("You lost ".to_string()
                    + &gold_gain.to_string()
                    + " gold to "
                    + &attacker.name
                    + "."),
                Some(Color::DarkYellow),
                sender,
            );
        }
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You gained ".to_string()
                + &gold_gain.to_string()
                + " gold from killing "
                + &defender.name
                + "!"),
            Some(Color::DarkYellow),
            sender,
        );
    }
}

pub(crate) fn send_exp_messages_to_players(
    defender: &Defender,
    attacker: &Attacker,
    current_hp: f32,
    exp_gain: u32,
    sender: &Sender<Packet>,
) {
    debug!(
        "Sending exp gain message to players: {:?}, {}",
        attacker, exp_gain
    );
    if current_hp <= 0.0 && exp_gain > 0 {
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You gained ".to_string()
                + &exp_gain.to_string()
                + " exp from killing "
                + &defender.name
                + "!"),
            Some(Color::DarkYellow),
            sender,
        );
    }
}

pub(crate) fn send_combat_messages_to_players(
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
            Some(Color::DarkRed),
            sender,
        );
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You hit ".to_string() + &defender.name + " for " + &damage.to_string() + " damage."),
            Some(Color::DarkGreen),
            sender,
        );
    } else {
        send_message_to_player(
            &defender.name,
            &defender.client_addr,
            defender.currently_online,
            &(attacker.name.to_string() + " attacks you, but missed."),
            Some(Color::Grey),
            sender,
        );
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You missed your attack against ".to_string() + &defender.name + "."),
            Some(Color::Grey),
            sender,
        );
    }
    if current_hp <= 0.0 {
        send_message_to_player(
            &defender.name,
            &defender.client_addr,
            defender.currently_online,
            &(attacker.name.to_string() + " killed you."),
            Some(Color::DarkRed),
            sender,
        );
        send_message_to_player(
            &attacker.name,
            &attacker.client_addr,
            attacker.currently_online,
            &("You killed ".to_string() + &defender.name + "."),
            Some(Color::DarkGreen),
            sender,
        );
    }
}
