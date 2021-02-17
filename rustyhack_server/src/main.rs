use std::env;

mod consts;
mod game;
mod networking;
mod setup;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    setup::initialise_log(env::args().collect());

    let (sender, receiver) = networking::bind_to_socket(setup::get_server_addr());

    game::run(sender, receiver);

    info!("Program terminated.");
}
