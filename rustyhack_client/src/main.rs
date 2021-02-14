mod consts;
mod engine;
mod message_handler;
mod player;
mod viewport;

#[macro_use]
extern crate log;
extern crate simplelog;

use laminar::Socket;
use simplelog::*;
use std::fs::File;
use std::net::SocketAddr;
use std::{env, io, process, thread};

fn main() {
    initialise_log();
    let (server_addr, client_addr) = get_server_addr();
    info!("Server address is set to: {}", &server_addr);
    info!("Client listen address is set to: {}", &client_addr);

    let player_name = get_player_name();

    let mut socket = Socket::bind(&client_addr).unwrap();
    let sender = socket.get_packet_sender();
    let receiver = socket.get_event_receiver();
    let _thread = thread::spawn(move || socket.start_polling());

    engine::run(sender, receiver, &server_addr, &client_addr, &player_name);
}

fn get_server_addr() -> (String, String) {
    println!("--Rustyhack MMO Client Setup--");

    let mut server_addr = String::new();
    loop {
        println!("Connect to which server? (default: 127.0.0.1:55301)");
        io::stdin()
            .read_line(&mut server_addr)
            .expect("Failed to read line");

        if server_addr.trim() == "" {
            println!("Using default server address.");
            server_addr = String::from("127.0.0.1:55301");
            break;
        }

        let server_socket_addr: SocketAddr = match server_addr.trim().parse() {
            Ok(value) => value,
            Err(err) => {
                println!(
                    "Not a valid socket address (e.g. 127.0.0.1:55301 ): {}",
                    err
                );
                continue;
            }
        };
        server_addr = server_socket_addr.to_string();
        break;
    }

    let mut client_addr = String::new();
    loop {
        println!(
            "What is the client receive address (local listen address)? (default: 127.0.0.1:55302)"
        );
        io::stdin()
            .read_line(&mut client_addr)
            .expect("Failed to read line");

        if client_addr.trim() == "" {
            println!("Using default client listen address.");
            client_addr = String::from("127.0.0.1:55302");
            break;
        }

        let client_socket_addr: SocketAddr = match client_addr.trim().parse() {
            Ok(value) => value,
            Err(err) => {
                println!(
                    "Not a valid socket address (e.g. 127.0.0.1:55302 ): {}",
                    err
                );
                continue;
            }
        };
        client_addr = client_socket_addr.to_string();
        break;
    }

    (server_addr, client_addr)
}

fn get_player_name() -> String {
    let mut player_name = String::new();
    loop {
        println!("What is your character name?");
        io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

        let parsed_player_name: String = match player_name.trim().parse() {
            Ok(value) => value,
            Err(err) => {
                println!("Must be a valid String: {}", err);
                continue;
            }
        };
        player_name = parsed_player_name;
        break;
    }

    player_name
}

fn initialise_log() {
    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push(consts::LOG_NAME);
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
