use crate::game::map::state::EntityPositionMap;
use bincode::{config, encode_to_vec};
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::{maybe_changed, system};
use rustyhack_lib::consts::DEAD_MAP;
use rustyhack_lib::ecs::components::{DisplayDetails, Inventory, PlayerDetails, Position, Stats};
use rustyhack_lib::network::packets::ServerMessage;
use std::process;
use uuid::Uuid;

#[system(par_for_each)]
#[filter(maybe_changed::<Position>())]
pub(super) fn send_player_position_updates(
    player_details: &PlayerDetails,
    position: &mut Position,
    #[resource] sender: &Sender<Packet>,
) {
    if position.update_available && player_details.currently_online {
        debug!(
            "Sending player position update for: {}",
            &player_details.player_name
        );
        let response = encode_to_vec(
            ServerMessage::UpdatePosition(position.clone()),
            config::standard(),
        )
        .unwrap_or_else(|err| {
            error!(
                "Failed to encode player position: {:?}, error: {}",
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

#[system(par_for_each)]
#[filter(maybe_changed::<Stats>())]
pub(super) fn send_player_stats_updates(
    player_details: &PlayerDetails,
    stats: &mut Stats,
    #[resource] sender: &Sender<Packet>,
) {
    if stats.update_available && player_details.currently_online {
        debug!(
            "Sending player stats update for: {}",
            &player_details.player_name
        );
        let response = encode_to_vec(ServerMessage::UpdateStats(*stats), config::standard())
            .unwrap_or_else(|err| {
                error!(
                    "Failed to encode player stats: {:?}, error: {}",
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

#[system(par_for_each)]
#[filter(maybe_changed::<Inventory>())]
pub(super) fn send_player_inventory_updates(
    player_details: &PlayerDetails,
    inventory: &mut Inventory,
    #[resource] sender: &Sender<Packet>,
) {
    if inventory.update_available && player_details.currently_online {
        debug!(
            "Sending player inventory update for: {}",
            &player_details.player_name
        );
        let response = encode_to_vec(
            ServerMessage::UpdateInventory(inventory.clone()),
            config::standard(),
        )
        .unwrap_or_else(|err| {
            error!(
                "Failed to encode player inventory: {:?}, error: {}",
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

#[system(par_for_each)]
pub(super) fn broadcast_entity_updates(
    player_details: &PlayerDetails,
    player_position: &Position,
    #[resource] sender: &Sender<Packet>,
    #[resource] entity_position_map: &EntityPositionMap,
) {
    if player_details.currently_online {
        for (entity_id, (entity_position, entity_display_details, entity_name_or_type)) in
            entity_position_map.clone()
        {
            //todo don't broadcast position updates to all players all the time, handle map changes better
            if entity_display_details.icon == '@'
                || entity_position.current_map == player_position.current_map
            {
                debug!("Sending entity update to: {}", &player_details.client_addr);
                let response = encode_entity_broadcast_packet(
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
                let response = encode_entity_broadcast_packet(
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

#[system]
pub(super) fn clear_entity_position_map(#[resource] entity_position_map: &mut EntityPositionMap) {
    debug!("Clearing entity position map.");
    entity_position_map.clear();
}

fn encode_entity_broadcast_packet(
    entity_id: Uuid,
    entity_position: Position,
    entity_display_details: DisplayDetails,
    entity_name_or_type: String,
    player_details: &PlayerDetails,
    player_position: &Position,
) -> Vec<u8> {
    encode_to_vec(
        ServerMessage::UpdateOtherEntities((
            entity_id,
            (
                entity_position.pos_x,
                entity_position.pos_y,
                entity_position.current_map,
                entity_display_details.icon,
                entity_display_details.colour,
                entity_name_or_type,
            ),
        )),
        config::standard(),
    )
    .unwrap_or_else(|err| {
        error!(
            "Failed to encode entity position broadcast to: {}, {}, @ map: {} error: {}",
            &player_details.player_name,
            &player_details.client_addr,
            &player_position.current_map,
            err
        );
        process::exit(1);
    })
}
