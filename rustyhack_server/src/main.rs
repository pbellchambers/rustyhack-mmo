mod background_map;
mod consts;
mod ecs;
mod engine;
mod viewport;

#[macro_use]
extern crate log;
extern crate simplelog;

use laminar::{Socket, SocketEvent};
use simplelog::*;
use std::fs::File;
use std::net::SocketAddr;
use std::{env, process, thread};

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
    const SERVER_ADDR: &'static str = "127.0.0.1:50201";
    let mut socket = Socket::bind(SERVER_ADDR).unwrap();

    let event_receiver = socket.get_event_receiver();
    // Starts the socket, which will start a poll mechanism to receive and send messages.
    let _thread = thread::spawn(move || socket.start_polling());

    loop {
        // Waits until a socket event occurs
        let result = event_receiver.recv();

        match result {
            Ok(socket_event) => match socket_event {
                SocketEvent::Packet(packet) => {
                    let endpoint: SocketAddr = packet.addr();
                    let received_data: &[u8] = packet.payload();
                    info!("Packet received from {}: {:?}", endpoint, String::from_utf8_lossy(received_data));
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Client connected from: {}", connect_event)
                }
                SocketEvent::Timeout(timeout_event) => {
                    info!("Client timed out {}", timeout_event)
                }
                _ => {}
            },
            Err(e) => {
                error!("Something went wrong when receiving, error: {:?}", e);
            }
        }
    }
}
