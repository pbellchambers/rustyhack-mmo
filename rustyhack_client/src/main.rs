mod consts;
mod engine;
mod message_handler;
mod viewport;

#[macro_use]
extern crate log;
extern crate simplelog;

use crate::consts::VALID_NAME_REGEX;
use laminar::Socket;
use regex::Regex;
use simplelog::*;
use std::fs::File;
use std::net::SocketAddr;
use std::{env, io, process, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    initialise_log(&args);
    let (server_addr, client_addr) = get_server_addr();
    info!("Server address is set to: {}", &server_addr);
    info!("Client listen address is set to: {}", &client_addr);

    let player_name = get_player_name();

    info!("Attempting to bind listen socket to: {}", &client_addr);
    let mut socket = Socket::bind_with_config(&client_addr, message_handler::get_laminar_config())
        .unwrap_or_else(|err| {
            error!("Unable to bind socket to {}, error: {}", &client_addr, err);
            process::exit(1);
        });
    info!("Successfully bound socket.");

    let sender = socket.get_packet_sender();
    let receiver = socket.get_event_receiver();
    let _thread = thread::spawn(move || socket.start_polling());
    info!("Spawned socket polling thread.");

    engine::run(sender, receiver, &server_addr, &client_addr, &player_name);
}

fn get_server_addr() -> (String, String) {
    println!("--Rustyhack MMO Client Setup--");

    let mut server_addr;
    loop {
        server_addr = String::new();
        println!("1) Connect to which server? (default: 127.0.0.1:50201)");
        io::stdin()
            .read_line(&mut server_addr)
            .expect("Failed to read input");

        if server_addr.trim() == "" {
            println!("Using default server address.");
            println!();
            server_addr = String::from("127.0.0.1:50201");
            break;
        }

        let server_socket_addr: SocketAddr = match server_addr.trim().parse() {
            Ok(value) => value,
            Err(err) => {
                println!(
                    "Not a valid socket address (e.g. 127.0.0.1:50201 ): {}",
                    err
                );
                continue;
            }
        };
        server_addr = server_socket_addr.to_string();
        break;
    }

    //handle client port allocation automatically
    //maybe need to revisit in future if it causes problems
    let client_addr = String::from("0.0.0.0:0");

    (server_addr, client_addr)
}

fn get_player_name() -> String {
    let mut player_name;
    loop {
        player_name = String::new();
        println!("3) What is your character name?");
        io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read input");

        let parsed_player_name: String = match player_name.trim().parse() {
            Ok(value) => value,
            Err(err) => {
                println!("Must be a valid String: {}", err);
                println!();
                continue;
            }
        };

        //must be 20 characters or less
        if parsed_player_name.chars().count() > 20 {
            println!("Character name must be 20 characters or less.");
            println!();
            continue;
        }

        //must only contain letters
        let regex = Regex::new(VALID_NAME_REGEX).expect("Player name regex is invalid.");
        if !regex.is_match(&parsed_player_name) {
            println!("Character name must only contain letters.");
            println!();
            continue;
        }

        player_name = parsed_player_name;
        break;
    }
    player_name
}

fn initialise_log(args: &[String]) {
    let mut log_level = LevelFilter::Info;
    if args.len() > 1 && args[1] == "--debug" {
        println!("Debug logging enabled.");
        log_level = LevelFilter::Debug;
    }

    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        eprintln!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push(consts::LOG_NAME);
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            simplelog::Config::default(),
            TerminalMode::Mixed,
        ),
        WriteLogger::new(
            log_level,
            simplelog::Config::default(),
            File::create(file_location.as_path()).unwrap_or_else(|err| {
                eprintln!("Unable to create log file: {}", err);
                process::exit(1);
            }),
        ),
    ])
    .unwrap_or_else(|err| {
        eprintln!(
            "Something went wrong when initialising the logging system: {}",
            err
        );
        process::exit(1);
    });
}
