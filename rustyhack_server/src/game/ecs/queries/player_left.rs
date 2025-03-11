use bincode::{config, encode_to_vec};
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
    let mut logged_out_map = String::new();
    let mut query = <(&mut PlayerDetails, &mut DisplayDetails, &Position)>::query();
    for (player_details, display_details, position) in query.iter_mut(world) {
        if player_details.client_addr == address
            && player_details.player_name == originating_player_name
        {
            logged_out_id = player_details.id;
            logged_out_map.clone_from(&position.current_map);
            display_details.visible = false;
            display_details.collidable = false;
            player_details.currently_online = false;
            player_details.client_addr = String::new();

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
    let mut logged_out_map = String::new();
    let mut query = <(&mut PlayerDetails, &mut DisplayDetails, &Position)>::query();
    for (player_details, display_details, position) in query.iter_mut(world) {
        if player_details.client_addr == address {
            logged_out_id = player_details.id;
            logged_out_map.clone_from(&position.current_map);
            display_details.visible = false;
            display_details.collidable = false;
            player_details.currently_online = false;
            player_details.client_addr = String::new();

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
    logged_out_player_map: &str,
) {
    //broadcast update to other players
    let mut query = <(&PlayerDetails, &Position)>::query();
    query.par_for_each(world, |(player_details, player_position)| {
        if player_details.currently_online && player_position.current_map == logged_out_player_map {
            info!(
                "Sending logged out player: {} update to: {}",
                &logged_out_player_id, &player_details.client_addr
            );

            let response = encode_to_vec(
                ServerMessage::UpdateOtherEntities((
                    logged_out_player_id,
                    (
                        0,
                        0,
                        "LoggedOut".to_string(),
                        DEFAULT_PLAYER_ICON,
                        DEFAULT_ITEM_COLOUR,
                        logged_out_player_id.to_string(),
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
    });
}
