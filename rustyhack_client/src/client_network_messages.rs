pub(super) mod client_network_packet_receiver;
pub(super) mod map_downloader;
pub(super) mod new_player;
pub(super) mod player_logout;

use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::time::Duration;
use std::{process, thread};

pub(super) fn bind_to_socket(client_addr: &str) -> (Sender<Packet>, Receiver<SocketEvent>) {
    info!("Attempting to bind listen socket to: {}", &client_addr);
    let socket =
        Socket::bind_with_config(client_addr, get_laminar_config()).unwrap_or_else(|err| {
            error!("Unable to bind socket to {}, error: {}", &client_addr, err);
            process::exit(1);
        });
    info!("Successfully bound socket.");

    let sender = socket.get_packet_sender();
    let receiver = socket.get_event_receiver();

    start_polling(socket);

    (sender, receiver)
}

fn get_laminar_config() -> laminar::Config {
    laminar::Config {
        idle_connection_timeout: Duration::from_secs(10),
        heartbeat_interval: Some(Duration::from_secs(2)),
        ..Default::default()
    }
}

fn start_polling(mut socket: Socket) {
    let _thread = thread::spawn(move || socket.start_polling());
    info!("Spawned socket polling thread.");
}
