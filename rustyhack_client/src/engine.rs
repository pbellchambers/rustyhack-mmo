use crate::consts::{CLIENT_ADDR, SERVER_ADDR, TARGET_FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
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

pub fn run(sender: &Sender<Packet>, receiver: &Receiver<SocketEvent>) {
    let mut player = create_new_player(&sender, &receiver, "client_player");
    info!("player_name is: {}", player.entity_name.name);
    let all_maps = download_all_maps_data(&sender, &receiver);
    info!("All maps is: {:?}", all_maps);

    let viewport = Viewport::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, TARGET_FPS);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);

    loop {
        console.wait_frame();
        console.clear_screen();

        player = send_and_request_location(&sender, &receiver, &console, player);
        viewport.draw_viewport_contents(&mut console, &player, all_maps.get(DEFAULT_MAP).unwrap());

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
) -> Player {
    let create_player_request_packet = Packet::reliable_unordered(
        SERVER_ADDR.parse().unwrap(),
        serialize(&PlayerMessage::CreatePlayer(CreatePlayerMessage {
            client_addr: CLIENT_ADDR.to_string(),
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
                    match msg_deserialised {
                        PlayerReply::PlayerCreated => {
                            info!("NewPlayer reply from {:?}", packet.addr())
                        }
                        _ => {}
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

fn download_all_maps_data(sender: &Sender<Packet>, receiver: &Receiver<SocketEvent>) -> AllMaps {
    let get_all_maps_request_packet = Packet::reliable_ordered(
        SERVER_ADDR.parse().unwrap(),
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
                    let msg_deserialised =
                        deserialize::<PlayerReply>(msg).expect(&String::from_utf8_lossy(msg));
                    match msg_deserialised {
                        PlayerReply::AllMaps(maps) => all_maps = maps,
                        _ => {}
                    }
                    let address = packet.addr();
                    info!("AllMaps reply from {:?}", address);
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
            SERVER_ADDR.parse().unwrap(),
            serialize(&PlayerMessage::UpdateVelocity(VelocityMessage {
                client_addr: CLIENT_ADDR.to_string(),
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
                        match msg_deserialised {
                            PlayerReply::UpdatePosition(position) => {
                                info!("Position update received from server: {:?}", &position);
                                player.position = position;
                            }
                            _ => {}
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
