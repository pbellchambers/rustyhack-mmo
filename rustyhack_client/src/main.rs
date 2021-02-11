mod engine;
mod player;
mod viewport;
mod consts;

#[macro_use]
extern crate log;
extern crate simplelog;

use laminar::Socket;
use simplelog::*;
use std::fs::File;
use std::{env, process, thread};
use crate::consts::CLIENT_ADDR;

fn main() {
    initialise_log();

    let mut socket = Socket::bind(CLIENT_ADDR).unwrap();
    let sender = socket.get_packet_sender();
    let receiver = socket.get_event_receiver();
    let _thread = thread::spawn(move || socket.start_polling());

    engine::run(&sender, &receiver);
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
