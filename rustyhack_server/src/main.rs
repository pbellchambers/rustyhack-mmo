#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

mod consts;
mod game;
mod network_messages;
mod setup;

use std::env;

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

    let udp_socket_addr = setup::get_server_addr();
    let tcp_socket_addr = setup::get_server_tcp_addr();
    info!("Server udp listen port is set to: {}", &udp_socket_addr);
    info!("Server tcp listen port is set to: {}", &tcp_socket_addr);

    let (sender, receiver) = network_messages::bind_to_socket(&udp_socket_addr);
    let (tcp_handler, tcp_listener) = network_messages::bind_to_tcp_socket(&tcp_socket_addr);

    game::run(&sender, receiver, tcp_handler, tcp_listener);

    info!("Program terminated.");
}
