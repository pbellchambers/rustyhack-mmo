use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::{PlayerRequest, PositionMessage};

pub(crate) fn send_pickup_request(sender: &Sender<Packet>, player: &Player, server_addr: &str) {
    let packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::PickupItem(PositionMessage {
            player_name: player.player_details.player_name.clone(),
            position: player.position.clone(),
        }))
        .unwrap(),
        Some(12),
    );
    rustyhack_lib::message_handler::send_packet(packet, sender);
    info!("Sent pickup request packet to server.");
}
