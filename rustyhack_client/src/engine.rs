use crate::consts::{GAME_TITLE, TARGET_FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use crate::message_handler;
use crate::viewport::Viewport;
use bincode::serialize;
use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::components::Velocity;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::player_message::{
    CreatePlayerMessage, EntityUpdates, PlayerMessage, PlayerReply, VelocityMessage,
};
use std::collections::HashMap;
use std::time::Duration;
use std::{process, thread};

pub fn run(
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    server_addr: &str,
    client_addr: &str,
    player_name: &str,
) {
    let (player_update_sender, player_update_receiver) = crossbeam_channel::unbounded();
    let (entity_update_sender, entity_update_receiver) = crossbeam_channel::unbounded();
    debug!("Spawned thread channels.");
    let local_sender = sender.clone();
    thread::spawn(move || {
        message_handler::run(sender, receiver, player_update_sender, entity_update_sender)
    });

    let all_maps = request_all_maps_data(&local_sender, &server_addr, &player_update_receiver);
    let mut player = send_new_player_request(
        &local_sender,
        player_name,
        &server_addr,
        &client_addr,
        &player_update_receiver,
    );
    info!("player_name is: {}", player.player_details.player_name);
    debug!("All maps is: {:?}", all_maps);

    let mut viewport = Viewport::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, TARGET_FPS);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);
    console.set_title(GAME_TITLE);
    info!("Initialised console engine.");

    let mut other_entities = EntityUpdates {
        updates: HashMap::new(),
    };

    loop {
        console.wait_frame();
        console.clear_screen();

        debug!("About to send player velocity update.");
        send_player_updates(&local_sender, &console, &player, &server_addr, &client_addr);

        debug!("About to wait for entity updates from server.");
        player = check_for_received_player_updates(&player_update_receiver, player);
        other_entities = check_for_received_entity_updates(&entity_update_receiver, other_entities);

        viewport.draw_viewport_contents(
            &mut console,
            &player,
            all_maps.get(&player.position.map).unwrap_or_else(|| {
                error!(
                    "There is no map for current player position: {}",
                    &player.position.map
                );
                process::exit(1);
            }),
            &other_entities,
        );

        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            break;
        }
    }
}

fn send_new_player_request(
    sender: &Sender<Packet>,
    player_name: &str,
    server_addr: &str,
    client_addr: &str,
    channel_receiver: &Receiver<PlayerReply>,
) -> Player {
    let create_player_request_packet = Packet::reliable_unordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerMessage::CreatePlayer(CreatePlayerMessage {
            client_addr: client_addr.to_string(),
            player_name: player_name.to_string(),
        }))
        .unwrap(),
    );
    message_handler::send_packet(create_player_request_packet, sender);
    info!("Sent new player request to server.");
    wait_for_new_player_response(channel_receiver)
}

fn request_all_maps_data(
    sender: &Sender<Packet>,
    server_addr: &str,
    channel_receiver: &Receiver<PlayerReply>,
) -> AllMaps {
    let get_all_maps_request_packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerMessage::GetAllMaps).expect("Error serialising GetAllMaps request."),
        Some(1),
    );
    message_handler::send_packet(get_all_maps_request_packet, sender);
    info!("Requested all maps data from server.");
    wait_for_all_maps_response(channel_receiver)
}

fn wait_for_all_maps_response(channel_receiver: &Receiver<PlayerReply>) -> AllMaps {
    let mut all_maps_downloaded = false;
    let mut all_maps = HashMap::new();
    loop {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerReply::AllMaps(message) => {
                    info!("All maps downloaded from server.");
                    all_maps_downloaded = true;
                    all_maps = message;
                }
                _ => {
                    info!(
                        "Ignoring other message types until maps downloaded. {:?}",
                        received_message
                    )
                }
            }
        }
        if all_maps_downloaded {
            info!("Got all maps data.");
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    all_maps
}

fn wait_for_new_player_response(channel_receiver: &Receiver<PlayerReply>) -> Player {
    let mut new_player_confirmed = false;
    let mut player = Player::default();
    loop {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerReply::PlayerCreated(message) => {
                    info!("New player creation confirmed.");
                    new_player_confirmed = true;
                    player = message;
                }
                _ => {
                    info!(
                        "Ignoring other message types until new player confirmed. {:?}",
                        received_message
                    )
                }
            }
        }
        if new_player_confirmed {
            info!("Got all data needed to begin game.");
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    player
}

fn send_player_updates(
    sender: &Sender<Packet>,
    console: &ConsoleEngine,
    player: &Player,
    server_addr: &str,
    client_addr: &str,
) {
    let mut velocity = Velocity { x: 0, y: 0 };
    if console.is_key_held(KeyCode::Up) {
        velocity.y = -1;
    } else if console.is_key_held(KeyCode::Down) {
        velocity.y = 1;
    } else if console.is_key_held(KeyCode::Left) {
        velocity.x = -1;
    } else if console.is_key_held(KeyCode::Right) {
        velocity.x = 1;
    }

    if velocity.y != 0 || velocity.x != 0 {
        debug!("Movement detected, sending velocity packet to server.");
        send_velocity_packet(sender, server_addr, client_addr, player, velocity);
    }
}

fn send_velocity_packet(
    sender: &Sender<Packet>,
    server_addr: &str,
    client_addr: &str,
    player: &Player,
    velocity: Velocity,
) {
    let packet = Packet::unreliable_sequenced(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerMessage::UpdateVelocity(VelocityMessage {
            client_addr: client_addr.to_string(),
            player_name: player.player_details.player_name.clone(),
            velocity,
        }))
        .unwrap(),
        Some(10),
    );
    message_handler::send_packet(packet, sender);
    debug!("Sent velocity packet to server.");
}

fn check_for_received_player_updates(
    channel_receiver: &Receiver<PlayerReply>,
    mut player: Player,
) -> Player {
    debug!("Checking for received player position from server.");
    while !channel_receiver.is_empty() {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerReply::UpdatePosition(new_position) => {
                    debug!("Player position update received: {:?}", &new_position);
                    player.position = new_position
                }
                _ => {
                    warn!(
                        "Unexpected message on channel from message handler: {:?}",
                        received_message
                    )
                }
            }
        }
    }
    player
}

fn check_for_received_entity_updates(
    channel_receiver: &Receiver<PlayerReply>,
    mut entity_updates: EntityUpdates,
) -> EntityUpdates {
    debug!("Checking for received entity updates from server.");
    while !channel_receiver.is_empty() {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerReply::UpdateOtherEntities(new_updates) => {
                    debug!("Entity updates received: {:?}", &new_updates);
                    entity_updates = new_updates;
                }
                _ => {
                    warn!(
                        "Unexpected message on channel from message handler: {:?}",
                        received_message
                    )
                }
            }
        }
    }
    entity_updates
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}
