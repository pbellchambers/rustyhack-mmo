use bincode::serialize;
use console_engine::ConsoleEngine;
use crossbeam_channel::Sender;
use crossterm::event::KeyCode;
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::{PlayerRequest, PositionMessage};

pub(crate) fn send_player_updates(
    sender: &Sender<Packet>,
    console: &ConsoleEngine,
    player: &mut Player,
    server_addr: &str,
) {
    if console.is_key_held(KeyCode::Up) {
        player.position.velocity_y = -1;
    } else if console.is_key_held(KeyCode::Down) {
        player.position.velocity_y = 1;
    } else if console.is_key_held(KeyCode::Left) {
        player.position.velocity_x = -1;
    } else if console.is_key_held(KeyCode::Right) {
        player.position.velocity_x = 1;
    }

    if player.position.velocity_y != 0 || player.position.velocity_x != 0 {
        debug!("Movement detected, sending velocity packet to server.");
        send_velocity_packet(sender, server_addr, player);
    }
    player.position.velocity_x = 0;
    player.position.velocity_y = 0;
}

fn send_velocity_packet(sender: &Sender<Packet>, server_addr: &str, player: &Player) {
    let packet = Packet::unreliable_sequenced(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::UpdateVelocity(PositionMessage {
            player_name: player.player_details.player_name.clone(),
            position: player.position.clone(),
        }))
        .unwrap(),
        Some(10),
    );
    rustyhack_lib::message_handler::send_packet(packet, sender);
    debug!("Sent velocity packet to server.");
}
