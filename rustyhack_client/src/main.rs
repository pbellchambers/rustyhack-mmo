#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

mod client_consts;
mod client_game;
mod client_network_messages;
mod client_setup;

use std::env;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    let args: Vec<String> = env::args().collect();
    client_setup::initialise_log(&args);

    let previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("{panic_info}");
        previous_panic_hook(panic_info);
        std::process::exit(1);
    }));

    let (server_udp_addr, server_tcp_addr, client_addr, player_name) =
        client_setup::get_player_setup_details();
    let (sender, receiver) = client_network_messages::bind_to_socket(&client_addr);

    client_game::run(
        &sender,
        receiver,
        &server_udp_addr,
        &server_tcp_addr,
        &client_addr,
        &player_name,
    );

    info!("Program terminated.");
}
