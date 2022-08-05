use crate::game::map::state::EntityPositionMap;
use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::consts::DEAD_MAP;
use rustyhack_lib::ecs::components::{DisplayDetails, Inventory, PlayerDetails, Position, Stats};
use rustyhack_lib::network::packets::ServerMessage;
use std::process;
use uuid::Uuid;

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
            rustyhack_lib::network::send_packet(
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
            rustyhack_lib::network::send_packet(
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
            rustyhack_lib::network::send_packet(
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

#[system]
pub(crate) fn broadcast_entity_updates(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &Position)>,
    #[resource] sender: &Sender<Packet>,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    for (player_details, player_position) in query.iter_mut(world) {
        if player_details.currently_online {
            for (entity_id, (entity_position, entity_display_details, entity_name_or_type)) in
                entity_position_map.clone()
            {
                if entity_position.current_map == player_position.current_map {
                    debug!("Sending entity update to: {}", &player_details.client_addr);
                    let response = serialize_entity_broadcast_packet(
                        entity_id,
                        entity_position,
                        entity_display_details,
                        entity_name_or_type,
                        player_details,
                        player_position,
                    );

                    rustyhack_lib::network::send_packet(
                        Packet::unreliable_sequenced(
                            player_details.client_addr.parse().unwrap(),
                            response,
                            Some(22),
                        ),
                        sender,
                    );
                } else if entity_position.current_map
                    == (player_position.current_map.clone() + DEAD_MAP)
                {
                    debug!(
                        "Sending dead entity update: {}",
                        &player_details.client_addr
                    );
                    let response = serialize_entity_broadcast_packet(
                        entity_id,
                        entity_position,
                        entity_display_details,
                        entity_name_or_type,
                        player_details,
                        player_position,
                    );

                    rustyhack_lib::network::send_packet(
                        Packet::reliable_ordered(
                            player_details.client_addr.parse().unwrap(),
                            response,
                            Some(25),
                        ),
                        sender,
                    );
                }
            }
        }
    }
    entity_position_map.clear();
    debug!("Finished broadcasting entity updates.");
}

fn serialize_entity_broadcast_packet(
    entity_id: Uuid,
    entity_position: Position,
    entity_display_details: DisplayDetails,
    entity_name_or_type: String,
    player_details: &PlayerDetails,
    player_position: &Position,
) -> Vec<u8> {
    serialize(&ServerMessage::UpdateOtherEntities((
        entity_id,
        (
            entity_position.pos_x,
            entity_position.pos_y,
            entity_position.current_map,
            entity_display_details.icon,
            entity_display_details.colour,
            entity_name_or_type,
        ),
    )))
    .unwrap_or_else(|err| {
        error!(
            "Failed to serialize entity position broadcast to: {}, {}, @ map: {} error: {}",
            &player_details.player_name,
            &player_details.client_addr,
            &player_position.current_map,
            err
        );
        process::exit(1);
    })
}
