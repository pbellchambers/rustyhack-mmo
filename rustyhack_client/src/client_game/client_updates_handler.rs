use crate::client_consts::DEFAULT_FG_COLOUR;
use chrono::{DateTime, Local};
use crossbeam_channel::Receiver;
use crossterm::style::Color;
use rustyhack_lib::consts::DEAD_MAP;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::{EntityPositionBroadcast, ServerMessage};

pub(crate) fn handle_received_server_messages(
    channel_receiver: &Receiver<ServerMessage>,
    player: &mut Player,
    entity_position_broadcast: &mut EntityPositionBroadcast,
    status_messages: &mut Vec<(String, Color)>,
) {
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
                    debug!("System message received: {:?}", &message);
                    let date_time: DateTime<Local> = Local::now();
                    let time = date_time.format("[%H:%M:%S] ").to_string();
                    status_messages.push((
                        (time + &message.message),
                        message.colour.unwrap_or(DEFAULT_FG_COLOUR),
                    ));
                }
                ServerMessage::UpdateOtherEntities(new_update) => {
                    debug!("Entity position broadcast received: {:?}", &new_update);
                    entity_position_broadcast.insert(new_update.0, new_update.1);
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
}

pub(crate) fn cleanup_dead_entities(
    player: &Player,
    entity_position_map: &mut EntityPositionBroadcast,
) {
    for (
        uuid,
        (
            _position_x,
            _position_y,
            entity_current_map,
            _entity_icon,
            _entity_icon_colour,
            _entity_name,
        ),
    ) in entity_position_map.clone()
    {
        if (player.position.current_map.clone() + DEAD_MAP) == entity_current_map {
            entity_position_map.remove(&uuid);
            debug!("Removed dead entity from entity map.");
        }
    }
}
