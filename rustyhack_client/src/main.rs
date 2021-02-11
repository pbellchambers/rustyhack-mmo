mod engine;
mod player;
mod viewport;

#[macro_use]
extern crate log;
extern crate simplelog;

use laminar::Socket;
use simplelog::*;
use std::fs::File;
use std::{env, process, thread};

fn main() {
    pub const SERVER_ADDR: &str = "127.0.0.1:50201";
    pub const CLIENT_ADDR: &str = "127.0.0.1:50202";

    initialise_log();

    let mut socket = Socket::bind(CLIENT_ADDR).unwrap();
    let sender = socket.get_packet_sender();
    let receiver = socket.get_event_receiver();
    let _thread = thread::spawn(move || socket.start_polling());

    engine::run(41, 15, 15, &sender, &receiver, SERVER_ADDR, CLIENT_ADDR);
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
