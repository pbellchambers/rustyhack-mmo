use bincode::serialize;
use chrono::{DateTime, Local};
use console_engine::{ConsoleEngine, KeyCode};
use crossbeam_channel::{Receiver, Sender};
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::{
    EntityUpdates, PlayerRequest, PositionMessage, ServerMessage,
};

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

pub(crate) fn check_for_received_server_messages(
    channel_receiver: &Receiver<ServerMessage>,
    player: &mut Player,
    mut entity_updates: EntityUpdates,
    status_messages: &mut Vec<String>,
) -> EntityUpdates {
    debug!("Checking for received messages from server.");
    while !channel_receiver.is_empty() {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                ServerMessage::UpdatePosition(new_position) => {
                    debug!("Player position update received: {:?}", &new_position);
                    player.position = new_position;
                }
                ServerMessage::UpdateStats(new_stats) => {
                    debug!("Player stats update received: {:?}", &new_stats);
                    player.stats = new_stats;
                }
                ServerMessage::UpdateInventory(new_inventory) => {
                    debug!("Player stats update received: {:?}", &new_inventory);
                    player.inventory = new_inventory.clone();
                }
                ServerMessage::SystemMessage(message) => {
                    debug!("System message received: {}", &message);
                    let date_time: DateTime<Local> = Local::now();
                    let time = date_time.format("[%H:%M:%S] ").to_string();
                    status_messages.push(time + &message);
                }
                ServerMessage::UpdateOtherEntities(new_updates) => {
                    debug!("Entity updates received: {:?}", &new_updates);
                    entity_updates = new_updates;
                }
                _ => {
                    warn!(
                        "Unexpected message on channel from message handler: {:?}",
                        received_message
                    );
                }
            }
        }
    }
    entity_updates
}
