#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

use std::env;

mod consts;
mod game;
mod networking;
mod setup;
mod world_backup;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    let args: Vec<String> = env::args().collect();
    setup::initialise_log(&args);

    let (sender, receiver) = networking::bind_to_socket(&setup::get_server_addr());

    game::run(sender, receiver);

    info!("Program terminated.");
}
