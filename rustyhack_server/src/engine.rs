use crate::background_map;
use crate::message_handler;
use bincode::serialize;
use console_engine::Color;
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Socket};
use legion::*;
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components;
use rustyhack_lib::ecs::components::*;
use rustyhack_lib::message_handler::player_message::{EntityUpdates, PlayerMessage, PlayerReply};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::{process, thread};

pub fn run(server_addr: &str) {
    info!("Attempting to bind socket to: {}", &server_addr);
    let mut socket = Socket::bind(&server_addr).unwrap_or_else(|err| {
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

    let mut player_velocity_updates: HashMap<EntityName, Velocity> = HashMap::new();

    let mut time = Instant::now();
    loop {
        player_velocity_updates =
            process_player_messages(&mut world, &channel_receiver, player_velocity_updates);

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
        if time.elapsed() > Duration::from_millis(50) {
            send_other_entities_updates(&mut world, &local_sender);
            time = Instant::now();
        }

        //todo: tune this, else it eats up cpu
        //need to add a counter that checks how long a loop took to run,
        // and sleep for the remaining "tick time"
        thread::sleep(Duration::from_millis(10));
    }
}

fn process_player_messages(
    world: &mut World,
    channel_receiver: &Receiver<PlayerMessage>,
    mut player_velocity_updates: HashMap<EntityName, Velocity>,
) -> HashMap<EntityName, Velocity> {
    debug!("Checking for player messages.");
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
                    create_player(world, message.player_name, message.client_addr);
                }
                PlayerMessage::UpdateVelocity(message) => {
                    debug!(
                        "Velocity update received for {} from: {}",
                        &message.player_name, &message.client_addr
                    );
                    player_velocity_updates.insert(
                        EntityName {
                            name: message.player_name,
                        },
                        message.velocity,
                    );
                    debug!("Processed velocity update: {:?}", &player_velocity_updates);
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

pub fn create_player(world: &mut World, name: String, client_addr: String) -> Entity {
    let player = world.push((
        EntityName { name: name.clone() },
        ClientAddress {
            address: client_addr,
        },
        IsPlayer { is_player: true },
        Position {
            x: 5,
            y: 5,
            map: DEFAULT_MAP.to_string(),
        },
        components::Velocity { x: 0, y: 0 },
        CollisionState { collidable: true },
        Character { icon: '@' },
        EntityColour {
            colour: Color::Magenta,
        },
        VisibleState { visible: true },
    ));
    info!("New player \"{}\" created: {:?}", name, &player);
    player
}

#[system(par_for_each)]
fn update_player_input(
    world_entity_name: &EntityName,
    velocity: &mut Velocity,
    #[resource] player_updates: &HashMap<EntityName, Velocity>,
) {
    debug!("Adding player velocity updates to world.");
    for (update_entity_name, update_velocity) in player_updates {
        if update_entity_name == world_entity_name {
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
    mut player_velocity_updates: HashMap<EntityName, Velocity>,
) -> HashMap<EntityName, Velocity> {
    let mut query = <(&EntityName, &mut Position, &ClientAddress)>::query();
    for (player_name, position, client_address) in query.iter_mut(world) {
        if player_velocity_updates.contains_key(player_name) {
            debug!("Sending player velocity update for: {}", &player_name.name);
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
                    client_address.address.parse().unwrap(),
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
    let mut updates: HashMap<EntityName, Position> = HashMap::new();
    let mut query = <(&EntityName, &mut Position)>::query();
    debug!("Getting all entities positions");
    for (entity_name, position) in query.iter_mut(world) {
        updates.insert(entity_name.clone(), position.clone());
    }

    let mut query2 = <&ClientAddress>::query();
    debug!("Sending entity updates to all players.");
    for client_address in query2.iter_mut(world) {
        debug!("Sending entity updates to: {}", &client_address.address);
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
                client_address.address.parse().unwrap(),
                response,
                Some(21),
            ),
            &sender,
        );
    }
    debug!("Finished sending entity updates to all players.");
}
