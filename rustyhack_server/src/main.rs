use simplelog::*;
use std::fs::File;
use std::net::SocketAddr;
use std::{env, io, process};

mod background_map;
mod consts;
mod engine;
mod message_handler;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() {
    let args: Vec<String> = env::args().collect();
    initialise_log(&args);
    let server_addr = get_server_addr();
    info!("Server listen port is set to: {}", &server_addr);
    engine::run(&server_addr);
    info!("Program terminated.");
}

fn get_server_addr() -> String {
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
    server_addr
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
