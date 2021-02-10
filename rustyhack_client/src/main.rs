#[macro_use]
extern crate log;
extern crate simplelog;

use laminar::{Packet, Socket, SocketEvent};
use simplelog::*;
use std::{process, env, thread};
use std::fs::File;
use crossbeam_channel::{Receiver, Sender};
use bincode::deserialize;
use rustyhack_lib::background_map::BackgroundMap;
use std::collections::HashMap;

fn main() {
    const SERVER_ADDR: &str = "127.0.0.1:50201";
    const CLIENT_ADDR: &str = "127.0.0.1:50202";

    initialise_log();

    let mut socket = Socket::bind(CLIENT_ADDR).unwrap();
    let sender = socket.get_packet_sender();
    let receiver = socket.get_event_receiver();
    let _thread = thread::spawn(move || socket.start_polling());

    let player_id = create_new_player(SERVER_ADDR, &sender, &receiver);
    let all_maps = download_all_maps_data(SERVER_ADDR, &sender, &receiver);
    info!("player_id is: {}", player_id);
    info!("All maps is: {:?}", all_maps);
}

fn download_all_maps_data(server_address: &str, sender: &Sender<Packet>, receiver: &Receiver<SocketEvent>) -> HashMap<String, BackgroundMap> {
    let get_all_maps_request_packet = Packet::reliable_ordered(
        server_address.parse().unwrap(),
        String::from("GetAllMaps").into_bytes(),
        Some(1),
    );
    sender.send(get_all_maps_request_packet).expect("This should work.");
    let all_maps;
    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let msg_deserialised = deserialize::<HashMap<String, BackgroundMap>>(msg).unwrap();
                    let address = packet.addr();
                    info!("AllMaps reply from {:?}: {:?}", address, msg_deserialised);
                    all_maps = msg_deserialised;
                    info!("AllMaps set to {:?}", all_maps);
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

fn create_new_player(server_address: &str, sender: &Sender<Packet>, receiver: &Receiver<SocketEvent>) -> String {
    let create_player_request_packet = Packet::reliable_unordered(
        server_address.parse().unwrap(),
        String::from("CreatePlayer: test_player").into_bytes(),
    );
    sender.send(create_player_request_packet).expect("This should work.");
    let player_id;
    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let msg_deserialised = deserialize::<String>(msg).unwrap();
                    let address = packet.addr();
                    info!("NewPlayer reply from {:?}: {:?}", address, msg_deserialised);
                    player_id = msg_deserialised;
                    info!("NewPlayer set to {}", player_id);
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
    player_id
}

fn initialise_log() {
    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push("rustyhack_client.log");
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(file_location.as_path()).unwrap(),
        ),
    ])
        .unwrap();
}
