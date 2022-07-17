#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use std::env;

mod client_consts;
mod client_game;
mod client_setup;
mod networking;
mod screens;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    client_setup::initialise_log(env::args().collect());

    let (server_addr, client_addr, player_name) = client_setup::get_player_setup_details();
    let (sender, receiver) = networking::bind_to_socket(&client_addr);

    client_game::run(sender, receiver, &server_addr, &client_addr, &player_name);

    info!("Program terminated.");
}
