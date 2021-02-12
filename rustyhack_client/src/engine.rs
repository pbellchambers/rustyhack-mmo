use crate::consts::{TARGET_FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use crate::player::Player;
use crate::viewport::Viewport;
use bincode::{deserialize, serialize};
use console_engine::{Color, ConsoleEngine, KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{Character, EntityColour, EntityName, Position, Velocity};
use rustyhack_lib::message_handler::player_message::{
    CreatePlayerMessage, PlayerMessage, PlayerReply, VelocityMessage,
};
use std::collections::HashMap;

pub fn run(
    sender: &Sender<Packet>,
    receiver: &Receiver<SocketEvent>,
    server_addr: &str,
    client_addr: &str,
    player_name: &str,
) {
    let mut player = create_new_player(&sender, &receiver, player_name, &server_addr, &client_addr);
    info!("player_name is: {}", player.entity_name.name);
    let all_maps = download_all_maps_data(&sender, &receiver, &server_addr);
    info!("All maps is: {:?}", all_maps);

    let viewport = Viewport::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, TARGET_FPS);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);

    loop {
        console.wait_frame();
        console.clear_screen();

        player = send_and_request_location(
            &sender,
            &receiver,
            &console,
            player,
            &server_addr,
            &client_addr,
        );
        viewport.draw_viewport_contents(
            &mut console,
            &player,
            all_maps.get(&player.position.map).unwrap(),
        );

        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            break;
        }
    }
}

fn create_new_player(
    sender: &Sender<Packet>,
    receiver: &Receiver<SocketEvent>,
    player_name: &str,
    server_addr: &str,
    client_addr: &str,
) -> Player {
    let create_player_request_packet = Packet::reliable_unordered(
        server_addr.parse().unwrap(),
        serialize(&PlayerMessage::CreatePlayer(CreatePlayerMessage {
            client_addr: client_addr.to_string(),
            player_name: player_name.to_string(),
        }))
        .unwrap(),
    );
    sender
        .send(create_player_request_packet)
        .expect("This should work.");
    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let msg_deserialised = deserialize::<PlayerReply>(msg).unwrap();
                    if let PlayerReply::PlayerCreated = msg_deserialised {
                        info!("NewPlayer reply from {:?}", packet.addr())
                    }
                    break;
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Client connected from: {}", connect_event)
                }
                SocketEvent::Timeout(address) => {
                    info!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }
    new_player(player_name.to_string())
}

fn new_player(name: String) -> Player {
    Player {
        entity_name: EntityName { name },
        position: Position {
            x: 5,
            y: 5,
            map: DEFAULT_MAP.to_string(),
        },
        character: Character { icon: '@' },
        entity_colour: EntityColour {
            colour: Color::Magenta,
        },
    }
}

fn download_all_maps_data(
    sender: &Sender<Packet>,
    receiver: &Receiver<SocketEvent>,
    server_addr: &str,
) -> AllMaps {
    let get_all_maps_request_packet = Packet::reliable_ordered(
        server_addr.parse().unwrap(),
        serialize(&PlayerMessage::GetAllMaps).unwrap(),
        Some(1),
    );
    sender
        .send(get_all_maps_request_packet)
        .expect("This should work.");
    let mut all_maps = HashMap::new();
    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let msg_deserialised = deserialize::<PlayerReply>(msg)
                        .unwrap_or_else(|err| panic!("An error deserialising: {}", err));
                    if let PlayerReply::AllMaps(maps) = msg_deserialised {
                        all_maps = maps
                    }
                    // let address = packet.addr();
                    // info!("AllMaps reply from {:?}", address);
                    break;
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Client connected from: {}", connect_event)
                }
                SocketEvent::Timeout(address) => {
                    info!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }
    all_maps
}

fn send_and_request_location(
    sender: &Sender<Packet>,
    receiver: &Receiver<SocketEvent>,
    console: &ConsoleEngine,
    mut player: Player,
    server_addr: &str,
    client_addr: &str,
) -> Player {
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
        let packet = Packet::unreliable_sequenced(
            server_addr.parse().unwrap(),
            serialize(&PlayerMessage::UpdateVelocity(VelocityMessage {
                client_addr: client_addr.to_string(),
                player_name: player.entity_name.name.clone(),
                velocity,
            }))
            .unwrap(),
            Some(10),
        );

        sender.send(packet).expect("This should work.");

        loop {
            if let Ok(event) = receiver.recv() {
                match event {
                    SocketEvent::Packet(packet) => {
                        let msg = packet.payload();
                        let msg_deserialised = deserialize::<PlayerReply>(msg).unwrap();
                        if let PlayerReply::UpdatePosition(position) = msg_deserialised {
                            info!("Position update received from server: {:?}", &position);
                            player.position = position;
                        }
                        break;
                    }
                    SocketEvent::Connect(connect_event) => {
                        info!("Client connected from: {}", connect_event)
                    }
                    SocketEvent::Timeout(address) => {
                        info!("Client timed out: {}", address);
                    }
                    _ => {}
                }
            }
        }
    }
    player
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}