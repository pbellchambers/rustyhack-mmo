use std::fs::File;
use std::{env, io, process};

use crate::consts::LOG_NAME;
use simplelog::*;
use std::net::SocketAddr;

mod background_map;
mod consts;
mod engine;
mod message_handler;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    initialise_log();
    let server_addr = get_server_addr();
    engine::run(&server_addr);
    info!("Program terminated.");
}

fn get_server_addr() -> String {
    println!("--Rustyhack Server Setup--");

    let mut server_addr = String::new();
    loop {
        println!("What is the server listen address? (ip.address:port)");
        io::stdin()
            .read_line(&mut server_addr)
            .expect("Failed to read line");

        let server_socket_addr: SocketAddr = match server_addr.trim().parse() {
            Ok(value) => value,
            Err(err) => {
                println!(
                    "Not a valid socket address (e.g. 127.0.0.1:50001 ): {}",
                    err
                );
                continue;
            }
        };
        server_addr = server_socket_addr.to_string();
        break;
    }
    server_addr
}

fn initialise_log() {
    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push(LOG_NAME);
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
