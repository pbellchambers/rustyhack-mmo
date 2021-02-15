use crate::message_handler;
use crate::message_handler::get_laminar_config;
use crate::{background_map, consts};
use bincode::serialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Socket};
use legion::*;
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components;
use rustyhack_lib::ecs::components::*;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::player_message::{EntityUpdates, PlayerMessage, PlayerReply};
use std::collections::HashMap;
use std::time::Instant;
use std::{process, thread};

pub fn run(server_addr: &str) {
    info!("Attempting to bind socket to: {}", &server_addr);
    let mut socket =
        Socket::bind_with_config(&server_addr, get_laminar_config()).unwrap_or_else(|err| {
            error!("Unable to bind socket to {}, error: {}", &server_addr, err);
            process::exit(1);
        });
    info!("Bound to socket successfully.");

    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let local_sender = sender.clone();
    thread::spawn(move || socket.start_polling());
    info!("Spawned socket polling thread.");

    let mut world = World::default();
    info!("Initialised ECS World");

    let all_maps_resource = background_map::initialise_all_maps();
    let all_maps = background_map::initialise_all_maps();

    let (channel_sender, channel_receiver) = crossbeam_channel::unbounded();
    info!("Created thread channel sender and receiver.");
    thread::spawn(move || message_handler::run(&sender, &receiver, &all_maps, channel_sender));

    let mut schedule = Schedule::builder()
        .add_system(update_player_input_system())
        .add_system(update_entities_position_system())
        .build();
    info!("Built system schedule.");

    let mut resources = Resources::default();
    resources.insert(all_maps_resource);
    info!("Finished loading all maps resource.");

    let mut player_velocity_updates: HashMap<String, Velocity> = HashMap::new();

    let mut entity_tick_time = Instant::now();
    let mut loop_tick_time = Instant::now();
    loop {
        player_velocity_updates = process_player_messages(
            &mut world,
            &channel_receiver,
            &local_sender,
            player_velocity_updates,
        );

        if !player_velocity_updates.is_empty() {
            debug!("Player velocity updates available, proceeding with world update.");
            resources.insert(player_velocity_updates.to_owned());
            debug!("Added player velocity updates to world resources.");

            debug!("Executing schedule...");
            schedule.execute(&mut world, &mut resources);
            debug!("Schedule executed successfully.");

            player_velocity_updates =
                send_player_updates(&mut world, &local_sender, player_velocity_updates);
        }

        //do every 50ms
        if entity_tick_time.elapsed() > consts::ENTITY_UPDATE_TICK {
            send_other_entities_updates(&mut world, &local_sender);
            entity_tick_time = Instant::now();
        }

        //todo: tune this, else it eats up cpu
        if loop_tick_time.elapsed() > consts::LOOP_TICK {
            warn!(
                "Loop took longer than specified tick time, expected: {:?}, actual: {:?}",
                consts::LOOP_TICK,
                loop_tick_time.elapsed()
            );
            loop_tick_time = Instant::now();
            continue;
        } else {
            let duration_to_sleep = consts::LOOP_TICK - loop_tick_time.elapsed();
            if duration_to_sleep.as_nanos() > 0 {
                thread::sleep(duration_to_sleep);
            }
            loop_tick_time = Instant::now();
        }
    }
}

fn process_player_messages(
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
                PlayerMessage::CreatePlayer(message) => {
                    info!(
                        "New player request received for {} from: {}",
                        &message.player_name, &message.client_addr
                    );
                    create_player(world, message.player_name, message.client_addr, &sender);
                }
                PlayerMessage::UpdateVelocity(message) => {
                    debug!(
                        "Velocity update received for {} from: {}",
                        &message.player_name, &message.client_addr
                    );
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
    let mut query = <&mut PlayerDetails>::query();
    for player_details in query.iter_mut(world) {
        if player_details.client_addr == address {
            player_details.currently_online = false;
            player_details.client_addr = "".to_string();
            info!(
                "Player {} at {} now marked as disconnected.",
                &player_details.player_name, &player_details.client_addr
            );
            break;
        }
    }
}

pub fn create_player(
    world: &mut World,
    name: String,
    client_addr: String,
    sender: &Sender<Packet>,
) {
    let player = Player {
        player_details: PlayerDetails {
            player_name: name.clone(),
            client_addr,
            currently_online: true,
        },
        ..Default::default()
    };

    let player_entity = world.push((
        player.player_details.clone(),
        player.display_details,
        player.position.clone(),
        components::Velocity { x: 0, y: 0 },
    ));
    info!("New player \"{}\" created: {:?}", name, &player_entity);

    let response = serialize(&PlayerReply::PlayerCreated(player.clone())).unwrap_or_else(|err| {
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

#[system(par_for_each)]
fn update_player_input(
    player_details: &PlayerDetails,
    velocity: &mut Velocity,
    #[resource] player_updates: &HashMap<String, Velocity>,
) {
    debug!("Adding player velocity updates to world.");
    for (update_entity_name, update_velocity) in player_updates {
        if update_entity_name == &player_details.player_name {
            velocity.x = update_velocity.x;
            velocity.y = update_velocity.y;
        }
    }
}

#[system(par_for_each)]
#[filter(maybe_changed::<Velocity>())]
fn update_entities_position(
    velocity: &mut Velocity,
    position: &mut Position,
    #[resource] all_maps: &AllMaps,
) {
    debug!("Updating world entities positions after velocity updates.");
    let current_map = all_maps.get(&position.map).unwrap_or_else(|| {
        error!(
            "Entity is located on a map that does not exist: {}",
            &position.map
        );
        warn!("Will return the default map, but this may cause problems.");
        all_maps.get(DEFAULT_MAP).unwrap()
    });
    if !entity_is_colliding(current_map.get_tile_at(
        (position.x + velocity.x) as usize,
        (position.y + velocity.y) as usize,
    )) {
        position.x += velocity.x;
        position.y += velocity.y;
    }
    velocity.x = 0;
    velocity.y = 0;
}

fn entity_is_colliding(tile: Tile) -> bool {
    match tile {
        Tile::Door(door) => door.collidable == Collidable::True,
        Tile::Wall(wall) => wall.collidable == Collidable::True,
        Tile::Boundary => true,
        _ => false,
    }
}

fn send_player_updates(
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

fn send_other_entities_updates(world: &mut World, sender: &Sender<Packet>) {
    let mut updates: HashMap<String, Position> = HashMap::new();
    let mut query = <(&PlayerDetails, &mut Position)>::query();
    debug!("Getting all players positions");
    for (player_details, position) in query.iter_mut(world) {
        if player_details.currently_online {
            updates.insert(player_details.player_name.clone(), position.clone());
        }
    }

    let mut query2 = <&PlayerDetails>::query();
    debug!("Sending entity updates to all players.");
    for player_details in query2.iter_mut(world) {
        if player_details.currently_online {
            debug!("Sending entity updates to: {}", &player_details.client_addr);
            let response = serialize(&PlayerReply::UpdateOtherEntities(EntityUpdates {
                updates: updates.clone(),
            }))
            .unwrap_or_else(|err| {
                error!(
                    "Failed to serialise entity updates: {:?}, error: {}",
                    &updates, err
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
