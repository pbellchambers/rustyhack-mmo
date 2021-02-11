use std::fs::File;
use std::{env, process};

use simplelog::*;

mod background_map;
mod engine;
mod message_handler;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    initialise_log();
    engine::run();
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
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(file_location.as_path()).unwrap(),
        ),
    ])
    .unwrap();
}
