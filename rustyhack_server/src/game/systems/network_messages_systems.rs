use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::ecs::components::{Inventory, PlayerDetails, Position, Stats};
use rustyhack_lib::message_handler::messages::ServerMessage;
use std::process;

#[system]
pub(crate) fn send_player_position_updates(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &mut Position)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (player_details, position) in query.iter_mut(world) {
        if position.update_available && player_details.currently_online {
            debug!(
                "Sending player position update for: {}",
                &player_details.player_name
            );
            let response = serialize(&ServerMessage::UpdatePosition(position.clone()))
                .unwrap_or_else(|err| {
                    error!(
                        "Failed to serialize player position: {:?}, error: {}",
                        &position, err
                    );
                    process::exit(1);
                });
            rustyhack_lib::message_handler::send_packet(
                Packet::unreliable_sequenced(
                    player_details.client_addr.parse().unwrap(),
                    response,
                    Some(20),
                ),
                sender,
            );
            position.update_available = false;
        }
    }
    debug!("Finished sending player position updates.");
}

#[system]
pub(crate) fn send_player_stats_updates(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &mut Stats)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (player_details, stats) in query.iter_mut(world) {
        if stats.update_available && player_details.currently_online {
            debug!(
                "Sending player stats update for: {}",
                &player_details.player_name
            );
            let response = serialize(&ServerMessage::UpdateStats(*stats)).unwrap_or_else(|err| {
                error!(
                    "Failed to serialize player stats: {:?}, error: {}",
                    &stats, err
                );
                process::exit(1);
            });
            rustyhack_lib::message_handler::send_packet(
                Packet::unreliable_sequenced(
                    player_details.client_addr.parse().unwrap(),
                    response,
                    Some(21),
                ),
                sender,
            );
            stats.update_available = false;
        }
    }
    debug!("Finished sending player stats updates.");
}

#[system]
pub(crate) fn send_player_inventory_updates(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &mut Inventory)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (player_details, inventory) in query.iter_mut(world) {
        if inventory.update_available && player_details.currently_online {
            debug!(
                "Sending player inventory update for: {}",
                &player_details.player_name
            );
            let response = serialize(&ServerMessage::UpdateInventory(inventory.clone()))
                .unwrap_or_else(|err| {
                    error!(
                        "Failed to serialize player inventory: {:?}, error: {}",
                        &inventory, err
                    );
                    process::exit(1);
                });
            rustyhack_lib::message_handler::send_packet(
                Packet::unreliable_sequenced(
                    player_details.client_addr.parse().unwrap(),
                    response,
                    Some(24),
                ),
                sender,
            );
            inventory.update_available = false;
        }
    }
    debug!("Finished sending player stats updates.");
}
