use bincode::serialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::{
    DisplayDetails, Inventory, MonsterDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::{
    EntityUpdates, PlayerRequest, PositionMessage, ServerMessage,
};
use std::collections::HashMap;
use std::process;
use uuid::Uuid;

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
                PlayerRequest::PlayerLogout(client_details) => {
                    info!(
                        "Player logout notification received for {} from: {}",
                        &client_details.player_name, &client_details.client_addr
                    );
                    set_player_logged_out(
                        world,
                        &client_details.client_addr,
                        &client_details.player_name,
                    );
                }
                PlayerRequest::Timeout(address) => {
                    set_player_disconnected(world, &address);
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

fn set_player_logged_out(world: &mut World, address: &str, originating_player_name: &str) {
    let mut query = <(&mut PlayerDetails, &mut DisplayDetails)>::query();
    for (player_details, display_details) in query.iter_mut(world) {
        if player_details.client_addr == address
            && player_details.player_name == originating_player_name
        {
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
}

fn set_player_disconnected(world: &mut World, address: &str) {
    let mut query = <(&mut PlayerDetails, &mut DisplayDetails)>::query();
    for (player_details, display_details) in query.iter_mut(world) {
        if player_details.client_addr == address {
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
                Packet::reliable_ordered(client_addr.parse().unwrap(), response, Some(11)),
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
    info!("New player \"{}\" created: {:?}", name, &player_entity);
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

pub(crate) fn send_player_position_updates(world: &mut World, sender: &Sender<Packet>) {
    let mut query = <(&PlayerDetails, &mut Position)>::query();
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

pub(crate) fn send_player_stats_updates(world: &mut World, sender: &Sender<Packet>) {
    let mut query = <(&PlayerDetails, &mut Stats)>::query();
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

pub(crate) fn send_player_inventory_updates(world: &mut World, sender: &Sender<Packet>) {
    let mut query = <(&PlayerDetails, &mut Inventory)>::query();
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

pub(crate) fn send_other_entities_updates(world: &World, sender: &Sender<Packet>) {
    let mut position_updates: HashMap<String, Position> = HashMap::new();
    let mut display_details: HashMap<String, DisplayDetails> = HashMap::new();
    let mut monster_type_map: HashMap<String, String> = HashMap::new();
    let mut query = <(&PlayerDetails, &Position, &DisplayDetails)>::query();
    debug!("Getting all players positions");
    for (player_details, position, display) in query.iter(world) {
        if player_details.currently_online {
            position_updates.insert(player_details.player_name.clone(), position.clone());
            display_details.insert(player_details.player_name.clone(), *display);
        }
    }

    let mut query = <(&MonsterDetails, &Position, &DisplayDetails)>::query();
    debug!("Getting all monster positions");
    for (monster_details, position, display) in query.iter(world) {
        position_updates.insert(monster_details.id.to_string(), position.clone());
        display_details.insert(monster_details.id.to_string(), *display);
        monster_type_map.insert(
            monster_details.id.to_string(),
            monster_details.monster_type.to_string(),
        );
    }

    let mut query = <&PlayerDetails>::query();
    debug!("Sending entity updates to all players.");
    for player_details in query.iter(world) {
        if player_details.currently_online {
            debug!("Sending entity updates to: {}", &player_details.client_addr);
            let response = serialize(&ServerMessage::UpdateOtherEntities(EntityUpdates {
                position_updates: position_updates.clone(),
                display_details: display_details.clone(),
                monster_type_map: monster_type_map.clone(),
            }))
            .unwrap_or_else(|err| {
                error!(
                    "Failed to serialize entity updates: {:?}, error: {}",
                    &position_updates, err
                );
                process::exit(1);
            });
            rustyhack_lib::message_handler::send_packet(
                Packet::unreliable_sequenced(
                    player_details.client_addr.parse().unwrap(),
                    response,
                    Some(22),
                ),
                sender,
            );
        }
    }
    debug!("Finished sending entity updates to all players.");
}

pub(crate) fn send_message_to_player(
    player_name: &String,
    client_addr: &String,
    currently_online: bool,
    message: &str,
    sender: &Sender<Packet>,
) {
    if currently_online && !client_addr.eq("") {
        debug!(
            "Sending system message to player {} at: {}",
            &player_name, &client_addr
        );
        let response = serialize(&ServerMessage::SystemMessage(message.to_string()))
            .unwrap_or_else(|err| {
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
