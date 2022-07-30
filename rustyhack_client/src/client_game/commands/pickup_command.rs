use crate::client_game::commands::look_command::return_visible_entity_at;
use bincode::serialize;
use chrono::{DateTime, Local};
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::{
    EntityPositionBroadcast, PlayerRequest, PositionMessage,
};

pub(crate) fn send_pickup_request(
    entity_position_map: &EntityPositionBroadcast,
    system_messages: &mut Vec<String>,
    sender: &Sender<Packet>,
    player: &Player,
    server_addr: &str,
) {
    let mut entity_underneath = "Nothing".to_string();
    entity_underneath = return_visible_entity_at(
        entity_underneath,
        entity_position_map,
        player,
        player.position.pos_x,
        player.position.pos_y,
    );

    if entity_underneath == *"Nothing" {
        let date_time: DateTime<Local> = Local::now();
        let time = date_time.format("[%H:%M:%S] ").to_string();
        info!("No item to pickup.");
        system_messages.push(time + "No item to pickup.");
    } else {
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
}
