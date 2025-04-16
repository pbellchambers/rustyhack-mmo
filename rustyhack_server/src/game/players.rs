use bincode::{config, encode_to_vec};
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::ecs::components::Position;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::ServerMessage;
use std::collections::HashMap;
use std::process;
use uuid::Uuid;

pub(super) type PlayersPositions = HashMap<Uuid, Position>;

pub(super) fn send_player_joined_response(player: &Player, sender: &Sender<Packet>) {
    let response = encode_to_vec(
        ServerMessage::PlayerJoined(player.clone()),
        config::standard(),
    )
    .unwrap_or_else(|err| {
        error!("Failed to encode player created response, error: {err}");
        process::exit(1);
    });
    rustyhack_lib::network::send_packet(
        Packet::reliable_ordered(
            player.player_details.client_addr.parse().unwrap(),
            response,
            Some(11),
        ),
        sender,
    );
}
