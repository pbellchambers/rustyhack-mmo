mod engine;
mod entity;
mod viewport;
mod world_map;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;
use std::{env, process};

fn main() {
    initialise_log();
    engine::run(40, 15, 15);
    info!("Program terminated.");
}

fn initialise_log() {
    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push("rustybox.log");
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
