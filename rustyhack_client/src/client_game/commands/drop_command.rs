use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::{PlayerRequest, PositionMessage};

pub(crate) fn send_drop_item_request(
    sender: &Sender<Packet>,
    player: &Player,
    server_addr: &str,
    item_index: u8,
) {
    let packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::DropItem((
            item_index,
            PositionMessage {
                player_name: player.player_details.player_name.clone(),
                position: player.position.clone(),
            },
        )))
        .unwrap(),
        Some(13),
    );
    rustyhack_lib::message_handler::send_packet(packet, sender);
    info!(
        "Sent drop item request packet to server for item {}.",
        item_index
    );
}
