mod engine;
mod entity;
mod viewport;
mod world_map;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;

fn main() {
    initialise_log();
    engine::run(40, 15, 15);
    info!("Program terminated.");
}

fn initialise_log() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("rustybox.log").unwrap(),
        ),
    ])
    .unwrap();
}
