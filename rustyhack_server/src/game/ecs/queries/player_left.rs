use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::consts::{DEFAULT_ITEM_COLOUR, DEFAULT_PLAYER_ICON};
use rustyhack_lib::ecs::components::{DisplayDetails, PlayerDetails, Position};
use rustyhack_lib::network::packets::ServerMessage;
use std::process;
use uuid::Uuid;

pub(crate) fn set_player_logged_out(
    world: &mut World,
    address: &str,
    originating_player_name: &str,
) -> (Uuid, String) {
    let mut logged_out_id = Uuid::new_v4();
    let mut logged_out_map = "".to_string();
    let mut query = <(&mut PlayerDetails, &mut DisplayDetails, &Position)>::query();
    for (player_details, display_details, position) in query.iter_mut(world) {
        if player_details.client_addr == address
            && player_details.player_name == originating_player_name
        {
            logged_out_id = player_details.id;
            logged_out_map = position.current_map.clone();
            display_details.visible = false;
            display_details.collidable = false;
            player_details.currently_online = false;
            player_details.client_addr = "".to_string();

            info!(
                "Player {} at {} logged out successfully.",
                &player_details.player_name, &address
            );
            break;
        }
    }
    (logged_out_id, logged_out_map)
}

pub(crate) fn set_player_disconnected(world: &mut World, address: &str) -> (Uuid, String) {
    let mut logged_out_id = Uuid::new_v4();
    let mut logged_out_map = "".to_string();
    let mut query = <(&mut PlayerDetails, &mut DisplayDetails, &Position)>::query();
    for (player_details, display_details, position) in query.iter_mut(world) {
        if player_details.client_addr == address {
            logged_out_id = player_details.id;
            logged_out_map = position.current_map.clone();
            display_details.visible = false;
            display_details.collidable = false;
            player_details.currently_online = false;
            player_details.client_addr = "".to_string();

            info!(
                "Player {} at {} now marked as disconnected.",
                &player_details.player_name, &address
            );
            break;
        }
    }
    (logged_out_id, logged_out_map)
}

pub(crate) fn broadcast_player_logged_out(
    world: &mut World,
    sender: &Sender<Packet>,
    logged_out_player_id: Uuid,
    logged_out_map: &str,
) {
    //broadcast update to other players
    let mut query = <(&PlayerDetails, &Position)>::query();
    for (player_details, player_position) in query.iter(world) {
        if player_details.currently_online && player_position.current_map == logged_out_map {
            info!(
                "Sending logged out player: {} update to: {}",
                &logged_out_player_id, &player_details.client_addr
            );

            let response = serialize(&ServerMessage::UpdateOtherEntities((
                logged_out_player_id,
                (
                    0,
                    0,
                    "LoggedOut".to_string(),
                    DEFAULT_PLAYER_ICON,
                    DEFAULT_ITEM_COLOUR,
                    logged_out_player_id.to_string(),
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
            });

            rustyhack_lib::network::send_packet(
                Packet::reliable_ordered(
                    player_details.client_addr.parse().unwrap(),
                    response,
                    Some(26),
                ),
                sender,
            );
        }
    }
}
