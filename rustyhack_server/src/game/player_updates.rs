use crate::networking::message_handler;
use bincode::serialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components;
use rustyhack_lib::ecs::components::{
    DisplayDetails, MonsterDetails, PlayerDetails, Position, Stats, Velocity,
};
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::player_message::{EntityUpdates, PlayerMessage, PlayerReply};
use std::collections::HashMap;
use std::process;

pub(crate) fn process_player_messages(
    world: &mut World,
    channel_receiver: &Receiver<PlayerMessage>,
    sender: &Sender<Packet>,
    mut player_velocity_updates: HashMap<String, Velocity>,
) -> HashMap<String, Velocity> {
    while !channel_receiver.is_empty() {
        debug!("Player messages are present.");
        let received = channel_receiver.try_recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerMessage::PlayerJoin(message) => {
                    info!(
                        "Player joined request received for {} from: {}",
                        &message.player_name, &message.client_addr
                    );
                    join_player(world, message.player_name, message.client_addr, &sender);
                }
                PlayerMessage::UpdateVelocity(message) => {
                    debug!("Velocity update received for {}", &message.player_name);
                    player_velocity_updates.insert(message.player_name, message.velocity);
                    debug!("Processed velocity update: {:?}", &player_velocity_updates);
                }
                PlayerMessage::Timeout(address) => {
                    set_player_disconnected(world, address);
                }
                _ => {
                    warn!("Didn't match any known message to process.")
                }
            }
        } else {
            debug!("Player messages channel receiver is now empty.")
        }
    }
    player_velocity_updates
}

fn set_player_disconnected(world: &mut World, address: String) {
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

fn join_player(world: &mut World, name: String, client_addr: String, sender: &Sender<Packet>) {
    let mut query = <(&mut PlayerDetails, &mut DisplayDetails, &Position, &Stats)>::query();
    let mut should_create_new_player = true;
    for (player_details, display_details, position, stats) in query.iter_mut(world) {
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
            };
            send_player_joined_response(player, sender);
            should_create_new_player = false;
            break;
        } else if player_details.player_name == name && player_details.currently_online {
            warn!("Player join request from {} for existing player that's currently online ({} at {}).", &client_addr, &name, &player_details.client_addr);
            let response = serialize(&PlayerReply::PlayerAlreadyOnline).unwrap_or_else(|err| {
                error!(
                    "Failed to serialise player already online response, error: {}",
                    err
                );
                process::exit(1);
            });
            message_handler::send_packet(
                Packet::reliable_ordered(client_addr.parse().unwrap(), response, Some(11)),
                &sender,
            );
            should_create_new_player = false;
            break;
        }
    }
    if should_create_new_player {
        create_player(world, name, client_addr, sender);
    }
}

fn create_player(world: &mut World, name: String, client_addr: String, sender: &Sender<Packet>) {
    let player = Player {
        player_details: PlayerDetails {
            player_name: name.clone(),
            client_addr,
            currently_online: true,
            level: 1,
            exp: 0,
            gold: 0,
        },
        ..Default::default()
    };

    let player_entity = world.push((
        player.player_details.clone(),
        player.display_details,
        player.position.clone(),
        player.stats,
        components::Velocity { x: 0, y: 0 },
    ));
    info!("New player \"{}\" created: {:?}", name, &player_entity);
    send_player_joined_response(player, sender);
}

fn send_player_joined_response(player: Player, sender: &Sender<Packet>) {
    let response = serialize(&PlayerReply::PlayerJoined(player.clone())).unwrap_or_else(|err| {
        error!(
            "Failed to serialise player created response, error: {}",
            err
        );
        process::exit(1);
    });
    message_handler::send_packet(
        Packet::reliable_ordered(
            player.player_details.client_addr.parse().unwrap(),
            response,
            Some(11),
        ),
        &sender,
    );
}

pub(crate) fn send_player_updates(
    world: &mut World,
    sender: &Sender<Packet>,
    mut player_velocity_updates: HashMap<String, Velocity>,
) -> HashMap<String, Velocity> {
    let mut query = <(&PlayerDetails, &mut Position)>::query();
    for (player_details, position) in query.iter_mut(world) {
        if player_velocity_updates.contains_key(&player_details.player_name)
            && player_details.currently_online
        {
            debug!(
                "Sending player velocity update for: {}",
                &player_details.player_name
            );
            let response = serialize(&PlayerReply::UpdatePosition(position.clone()))
                .unwrap_or_else(|err| {
                    error!(
                        "Failed to serialise player position: {:?}, error: {}",
                        &position, err
                    );
                    process::exit(1);
                });
            message_handler::send_packet(
                Packet::unreliable_sequenced(
                    player_details.client_addr.parse().unwrap(),
                    response,
                    Some(20),
                ),
                &sender,
            );
        }
    }
    player_velocity_updates.clear();
    debug!("Finished sending player velocity updates.");
    player_velocity_updates
}

pub(crate) fn send_other_entities_updates(world: &World, sender: &Sender<Packet>) {
    let mut position_updates: HashMap<String, Position> = HashMap::new();
    let mut display_details: HashMap<String, DisplayDetails> = HashMap::new();
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
    }

    let mut query = <&PlayerDetails>::query();
    debug!("Sending entity updates to all players.");
    for player_details in query.iter(world) {
        if player_details.currently_online {
            debug!("Sending entity updates to: {}", &player_details.client_addr);
            let response = serialize(&PlayerReply::UpdateOtherEntities(EntityUpdates {
                position_updates: position_updates.clone(),
                display_details: display_details.clone(),
            }))
            .unwrap_or_else(|err| {
                error!(
                    "Failed to serialise entity updates: {:?}, error: {}",
                    &position_updates, err
                );
                process::exit(1);
            });
            message_handler::send_packet(
                Packet::unreliable_sequenced(
                    player_details.client_addr.parse().unwrap(),
                    response,
                    Some(21),
                ),
                &sender,
            );
        }
    }
    debug!("Finished sending entity updates to all players.");
}
