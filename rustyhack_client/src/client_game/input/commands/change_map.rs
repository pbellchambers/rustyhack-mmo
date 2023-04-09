use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::{PlayerRequest, PositionMessage};

pub(crate) fn send_change_map_request(sender: &Sender<Packet>, player: &Player, server_addr: &str) {
    let packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::ChangeMap(PositionMessage {
            player_name: player.player_details.player_name.clone(),
            position: player.position.clone(),
        }))
        .unwrap(),
        Some(15),
    );
    rustyhack_lib::network::send_packet(packet, sender);
    info!("Sent change map request packet to server.");
}
