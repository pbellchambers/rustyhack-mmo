use crate::game::systems;
use bincode::serialize;
use crossbeam_channel::{Receiver, Sender};
use crossterm::style::Color;
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::consts::{DEFAULT_ITEM_COLOUR, DEFAULT_PLAYER_ICON};
use rustyhack_lib::ecs::components::{DisplayDetails, Inventory, PlayerDetails, Position, Stats};
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::{
    PlayerRequest, PositionMessage, ServerMessage, SystemMessage,
};
use std::process;
use uuid::Uuid;

//todo resolve this clippy warning
#[allow(clippy::too_many_lines)]
pub(crate) fn process_player_messages(
    world: &mut World,
    channel_receiver: &Receiver<PlayerRequest>,
    sender: &Sender<Packet>,
) -> bool {
    let mut has_player_updates = false;
    while !channel_receiver.is_empty() {
        debug!("Player messages are present.");
        let received = channel_receiver.try_recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerRequest::PlayerJoin(client_details) => {
                    info!(
                        "Player joined request received for {} from: {}",
                        &client_details.player_name, &client_details.client_addr
                    );
                    join_player(
                        world,
                        &client_details.player_name,
                        client_details.client_addr,
                        sender,
                    );
                }
                PlayerRequest::UpdateVelocity(position_message) => {
                    debug!(
                        "Velocity update received for {}",
                        &position_message.player_name
                    );
                    set_player_velocity(world, &position_message);
                    debug!("Processed velocity update.");
                }
                PlayerRequest::PickupItem(position_message) => {
                    debug!(
                        "Pickup item request received from {} at ({},{}) on {} map.",
                        &position_message.player_name,
                        &position_message.position.pos_x,
                        &position_message.position.pos_y,
                        &position_message.position.current_map,
                    );
                    systems::player_systems::pickup_item(world, &position_message, sender);
                    debug!("Processed item pickup request.");
                }
                PlayerRequest::DropItem(drop_item_details) => {
                    debug!(
                        "Drop item request received from {} at ({},{}) on {} map.",
                        &drop_item_details.1.player_name,
                        &drop_item_details.1.position.pos_x,
                        &drop_item_details.1.position.pos_y,
                        &drop_item_details.1.position.current_map,
                    );
                    systems::player_systems::drop_item(
                        world,
                        drop_item_details.0,
                        &drop_item_details.1,
                        sender,
                    );
                    debug!("Processed item pickup request.");
                }
                PlayerRequest::StatUp(stat_up_details) => {
                    debug!(
                        "Stat up request received from {} for {}.",
                        &stat_up_details.1, &stat_up_details.0,
                    );
                    systems::player_systems::increase_stat(
                        world,
                        &stat_up_details.0,
                        &stat_up_details.1,
                        sender,
                    );
                    debug!("Processed stat up request.");
                }
                PlayerRequest::PlayerLogout(client_details) => {
                    info!(
                        "Player logout notification received for {} from: {}",
                        &client_details.player_name, &client_details.client_addr
                    );
                    let (logged_out_player_id, logged_out_map) = set_player_logged_out(
                        world,
                        &client_details.client_addr,
                        &client_details.player_name,
                    );
                    broadcast_player_logged_out(
                        world,
                        sender,
                        logged_out_player_id,
                        &logged_out_map,
                    );
                }
                PlayerRequest::Timeout(address) => {
                    let (logged_out_player_id, logged_out_map) =
                        set_player_disconnected(world, &address);
                    broadcast_player_logged_out(
                        world,
                        sender,
                        logged_out_player_id,
                        &logged_out_map,
                    );
                }
                _ => {
                    warn!("Didn't match any known message to process.");
                }
            }
            has_player_updates = true;
        } else {
            debug!("Player messages channel receiver is now empty.");
        }
    }
    has_player_updates
}

fn set_player_velocity(world: &mut World, position_message: &PositionMessage) {
    let mut query = <(&mut PlayerDetails, &mut Position)>::query();
    for (player_details, position) in query.iter_mut(world) {
        if player_details.player_name == position_message.player_name {
            position.velocity_x = position_message.position.velocity_x;
            position.velocity_y = position_message.position.velocity_y;
        }
    }
}

fn set_player_logged_out(
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

fn set_player_disconnected(world: &mut World, address: &str) -> (Uuid, String) {
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

fn broadcast_player_logged_out(
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

            rustyhack_lib::message_handler::send_packet(
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

fn join_player(world: &mut World, name: &str, client_addr: String, sender: &Sender<Packet>) {
    let mut query = <(
        &mut PlayerDetails,
        &mut DisplayDetails,
        &Position,
        &Stats,
        &Inventory,
    )>::query();
    let mut should_create_new_player = true;
    for (player_details, display_details, position, stats, inventory) in query.iter_mut(world) {
        if player_details.player_name == name && !player_details.currently_online {
            player_details.currently_online = true;
            player_details.client_addr = client_addr.clone();
            display_details.collidable = true;
            display_details.visible = true;
            info!(
                "Existing player \"{}\" logged in from: {}",
                name, &client_addr
            );
            let player = Player {
                player_details: player_details.clone(),
                display_details: *display_details,
                position: position.clone(),
                stats: *stats,
                inventory: inventory.clone(),
            };
            send_player_joined_response(&player, sender);
            should_create_new_player = false;
            break;
        } else if player_details.player_name == name && player_details.currently_online {
            warn!("Player join request from {} for existing player that's currently online ({} at {}).", &client_addr, &name, &player_details.client_addr);
            let response = serialize(&ServerMessage::PlayerAlreadyOnline).unwrap_or_else(|err| {
                error!(
                    "Failed to serialize player already online response, error: {}",
                    err
                );
                process::exit(1);
            });
            rustyhack_lib::message_handler::send_packet(
                Packet::reliable_ordered(client_addr.parse().unwrap(), response, Some(14)),
                sender,
            );
            should_create_new_player = false;
            break;
        }
    }
    if should_create_new_player {
        create_player(world, name, client_addr, sender);
    }
}

fn create_player(world: &mut World, name: &str, client_addr: String, sender: &Sender<Packet>) {
    let player = Player {
        player_details: PlayerDetails {
            id: Uuid::new_v4(),
            player_name: name
                .parse()
                .expect("Something went wrong parsing player name."),
            client_addr,
            currently_online: true,
        },
        ..Default::default()
    };

    let player_entity = world.push((
        player.player_details.clone(),
        player.display_details,
        player.position.clone(),
        player.stats,
        player.inventory.clone(),
    ));
    info!("New player \"{}\" created: {:?}", name, player_entity);
    send_player_joined_response(&player, sender);
}

fn send_player_joined_response(player: &Player, sender: &Sender<Packet>) {
    let response = serialize(&ServerMessage::PlayerJoined(player.clone())).unwrap_or_else(|err| {
        error!(
            "Failed to serialize player created response, error: {}",
            err
        );
        process::exit(1);
    });
    rustyhack_lib::message_handler::send_packet(
        Packet::reliable_ordered(
            player.player_details.client_addr.parse().unwrap(),
            response,
            Some(11),
        ),
        sender,
    );
}

pub(crate) fn send_message_to_player(
    player_name: &String,
    client_addr: &String,
    currently_online: bool,
    message: &str,
    colour: Option<Color>,
    sender: &Sender<Packet>,
) {
    if currently_online && !client_addr.eq("") {
        debug!(
            "Sending system message to player {} at: {}",
            &player_name, &client_addr
        );
        let system_message = SystemMessage {
            message: message.to_string(),
            colour,
        };
        let response =
            serialize(&ServerMessage::SystemMessage(system_message)).unwrap_or_else(|err| {
                error!(
                    "Failed to serialize system message: {}, error: {}",
                    message, err
                );
                process::exit(1);
            });
        rustyhack_lib::message_handler::send_packet(
            Packet::reliable_ordered(client_addr.parse().unwrap(), response, Some(23)),
            sender,
        );
    }
}
