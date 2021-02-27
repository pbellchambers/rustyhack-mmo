use crate::consts;
use rustyhack_lib::file_utils;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
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
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            log_level,
            Config::default(),
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

pub(crate) fn get_server_addr() -> String {
    println!("--Rustyhack MMO Server Setup--");

    let mut server_addr;
    loop {
        server_addr = String::new();
        println!("1) What is the server listen port? (default: 50201)");
        io::stdin()
            .read_line(&mut server_addr)
            .expect("Failed to read line");

        if server_addr.trim() == "" {
            println!("Using default server listen port.");
            server_addr = String::from("0.0.0.0:50201");
            break;
        }

        server_addr = String::from("0.0.0.0:") + &*server_addr;
        let server_socket_addr: SocketAddr = match server_addr.trim().parse() {
            Ok(value) => value,
            Err(err) => {
                println!("Not a valid port (e.g. 50201 ): {}", err);
                continue;
            }
        };
        server_addr = server_socket_addr.to_string();
        break;
    }
    info!("Server listen port is set to: {}", &server_addr);
    server_addr
}
