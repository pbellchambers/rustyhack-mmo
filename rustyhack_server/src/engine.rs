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
use std::thread;
use std::time::Duration;

pub fn run(server_addr: &str) {
    let mut socket = Socket::bind(&server_addr).unwrap();
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let local_sender = sender.clone();
    thread::spawn(move || socket.start_polling());

    let mut world = World::default();
    let all_maps_resource = background_map::initialise_all_maps();
    let all_maps = background_map::initialise_all_maps();
    let (channel_sender, channel_receiver) = crossbeam_channel::unbounded();
    thread::spawn(move || message_handler::run(&sender, &receiver, &all_maps, channel_sender));

    let mut schedule = Schedule::builder()
        .add_system(update_player_input_system())
        .add_system(update_entities_position_system())
        .build();

    let mut resources = Resources::default();
    resources.insert(all_maps_resource);

    let mut player_velocity_updates: HashMap<EntityName, Velocity> = HashMap::new();
    let mut count = 0;
    loop {
        player_velocity_updates =
            process_player_messages(&mut world, &channel_receiver, player_velocity_updates);
        if !player_velocity_updates.is_empty() {
            info!("Player velocity updates available, proceeding with world update.");
            resources.insert(player_velocity_updates.to_owned());
            schedule.execute(&mut world, &mut resources);
            player_velocity_updates =
                send_player_updates(&mut world, &local_sender, player_velocity_updates);
        }

        //aim to send once per second tick
        //todo make this better than a simple count, use actual time elapsed
        if count > 100 {
            send_other_entities_updates(&mut world, &local_sender);
            count = 0;
        }

        //todo: tune this, else it eats up cpu
        //need to add a counter that checks how long a loop took to run,
        // and sleep for the remaining "tick time"
        thread::sleep(Duration::from_millis(10));
        count += 1;
    }
}

fn process_player_messages(
    world: &mut World,
    channel_receiver: &Receiver<PlayerMessage>,
    mut player_velocity_updates: HashMap<EntityName, Velocity>,
) -> HashMap<EntityName, Velocity> {
    while !channel_receiver.is_empty() {
        info!("Channel receiver has entries.");
        let received = channel_receiver.try_recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerMessage::CreatePlayer(message) => {
                    create_player(world, message.player_name, message.client_addr);
                }
                PlayerMessage::UpdateVelocity(message) => {
                    info!("Processing update velocity message");
                    player_velocity_updates.insert(
                        EntityName {
                            name: message.player_name,
                        },
                        message.velocity,
                    );
                    info!("Processed: {:?}", &player_velocity_updates);
                }
                _ => {
                    warn!("Didn't match any known message to process.")
                }
            }
        } else {
            info!("Channel receiver is now empty.")
        }
    }
    player_velocity_updates
}

pub fn create_player(world: &mut World, name: String, client_addr: String) -> Entity {
    let player = world.push((
        EntityName { name },
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
    info!("Created player: {:?}", player);
    player
}

#[system(par_for_each)]
fn update_player_input(
    world_entity_name: &EntityName,
    velocity: &mut Velocity,
    #[resource] player_updates: &HashMap<EntityName, Velocity>,
) {
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
    let current_map = all_maps.get(&position.map).unwrap();
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
            let response = serialize(&PlayerReply::UpdatePosition(position.clone())).unwrap();
            sender
                .send(Packet::unreliable_sequenced(
                    client_address.address.parse().unwrap(),
                    response,
                    Some(20),
                ))
                .expect("Player update didn't send.");
        }
    }
    player_velocity_updates.clear();
    player_velocity_updates
}

fn send_other_entities_updates(world: &mut World, sender: &Sender<Packet>) {
    let mut updates: HashMap<EntityName, Position> = HashMap::new();
    let mut query = <(&EntityName, &mut Position)>::query();
    for (entity_name, position) in query.iter_mut(world) {
        updates.insert(entity_name.clone(), position.clone());
    }

    let mut query2 = <&ClientAddress>::query();
    info!("Sending entity updates to all players.");
    for client_address in query2.iter_mut(world) {
        let response = serialize(&PlayerReply::UpdateOtherEntities(EntityUpdates {
            updates: updates.clone(),
        }))
        .unwrap();
        sender
            .send(Packet::unreliable_sequenced(
                client_address.address.parse().unwrap(),
                response,
                Some(21),
            ))
            .expect("Entities update didn't send.");
    }
}
