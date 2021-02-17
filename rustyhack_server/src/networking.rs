use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::time::Duration;
use std::{process, thread};

pub(crate) mod message_handler;

pub(crate) fn bind_to_socket(server_addr: String) -> (Sender<Packet>, Receiver<SocketEvent>) {
    info!("Attempting to bind socket to: {}", &server_addr);
    let mut socket =
        Socket::bind_with_config(&server_addr, get_laminar_config()).unwrap_or_else(|err| {
            error!("Unable to bind socket to {}, error: {}", &server_addr, err);
            process::exit(1);
        });
    info!("Bound to socket successfully.");

    let sender = socket.get_packet_sender();
    let receiver = socket.get_event_receiver();

    thread::spawn(move || socket.start_polling());
    info!("Spawned socket polling thread.");

    (sender, receiver)
}

fn get_laminar_config() -> laminar::Config {
    laminar::Config {
        idle_connection_timeout: Duration::from_secs(10),
        max_fragments: 255,
        ..Default::default()
    }
}
