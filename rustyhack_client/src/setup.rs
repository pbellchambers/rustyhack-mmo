use crate::consts;
use regex::Regex;
use rustyhack_lib::file_utils;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::net::SocketAddr;
use std::{io, process};

pub(crate) fn initialise_log(args: Vec<String>) {
    let mut log_level = LevelFilter::Info;
    if args.len() > 1 && args[1] == "--debug" {
        println!("Debug logging enabled.");
        log_level = LevelFilter::Debug;
    }

    let mut file_location = file_utils::current_exe_location();
    file_location.pop();
    file_location.push(consts::LOG_NAME);
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
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

pub(crate) fn get_player_setup_details() -> (String, String, String) {
    let (server_addr, client_addr) = get_server_addr();
    let player_name = get_player_name();
    (server_addr, client_addr, player_name)
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

    info!("Server address is set to: {}", &server_addr);
    info!("Client listen address is set to: {}", &client_addr);
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
        let regex = Regex::new(consts::VALID_NAME_REGEX).expect("Player name regex is invalid.");
        if !regex.is_match(&parsed_player_name) {
            println!("Character name must only contain letters.");
            println!();
            continue;
        }

        player_name = parsed_player_name;
        break;
    }
    info!("Requested player name is: {}", &player_name);
    player_name
}
