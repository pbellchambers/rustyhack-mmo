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

    let (server_addr, client_addr, player_name) = setup::get_player_setup_details();
    let (sender, receiver) = networking::bind_to_socket(&client_addr);

    game::run(sender, receiver, &server_addr, &client_addr, &player_name);

    info!("Program terminated.");
}
