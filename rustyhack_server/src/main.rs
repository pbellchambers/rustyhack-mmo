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

    let previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("{}", panic_info.to_string());
        previous_panic_hook(panic_info);
        std::process::exit(1);
    }));

    let (sender, receiver) = networking::bind_to_socket(&setup::get_server_addr());

    game::run(sender, receiver);

    info!("Program terminated.");
}
