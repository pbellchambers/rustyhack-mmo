use std::{env, process, thread};
use std::fs::File;

use laminar::{Packet, Socket, SocketEvent};
use simplelog::*;

use bincode::serialize;
use crate::background_map::initialise_all_maps;

mod consts;
mod ecs;
mod engine;
mod viewport;
mod background_map;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    initialise_log();
    run_server();
    engine::run(41, 15, 15);
    info!("Program terminated.");
}

fn initialise_log() {
    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push("rustyhack_server.log");
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

fn run_server() {
    const SERVER_ADDR: &str = "127.0.0.1:50201";

    let mut socket = Socket::bind(SERVER_ADDR).unwrap();
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let msg = String::from_utf8_lossy(msg);
                    let address = packet.addr();

                    info!("Received {:?} from {:?}", msg, address);
                    if msg.starts_with("CreatePlayer:") {
                        let response = serialize(&String::from("NewPlayer: someplayer")).unwrap();
                        debug!("Will try to send NewPlayer: {:?}", response);
                        sender
                            .send(Packet::reliable_unordered(
                                packet.addr(),
                                serialize(&"NewPlayer: someplayer").unwrap(),
                            ))
                            .expect("This should send");
                    } else if msg == "GetAllMaps" {
                        let all_maps = initialise_all_maps();
                        let response = serialize(&all_maps).unwrap();
                        debug!("Will try to send AllMaps: {:?}", response);
                        sender
                            .send(Packet::reliable_ordered(
                                packet.addr(),
                                serialize(&all_maps).unwrap(),
                                Some(2),
                            ))
                            .expect("This should send");
                        break;
                    } else {
                        warn!("Unknown message from {:?}: {:?}", address, msg);
                    }
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
