pub(super) mod combat_updates;
pub(super) mod map_sender;
pub(super) mod packet_receiver;

use bincode::{config, encode_to_vec};
use crossbeam_channel::{Receiver, Sender};
use crossterm::style::Color;
use laminar::{Packet, Socket, SocketEvent};
use message_io::network::Transport;
use message_io::node;
use message_io::node::{NodeHandler, NodeListener};
use rustyhack_lib::network::packets::{ServerMessage, SystemMessage};
use std::time::Duration;
use std::{process, thread};

pub(super) fn bind_to_socket(server_addr: &str) -> (Sender<Packet>, Receiver<SocketEvent>) {
    info!("Attempting to bind socket to: {}", &server_addr);
    let mut socket =
        Socket::bind_with_config(server_addr, get_laminar_config()).unwrap_or_else(|err| {
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

pub(super) fn bind_to_tcp_socket(server_addr: &str) -> (NodeHandler<()>, NodeListener<()>) {
    info!("Attempting to bind tcp socket to: {}", &server_addr);

    let (handler, listener) = node::split::<()>();
    handler
        .network()
        .listen(Transport::FramedTcp, server_addr)
        .unwrap_or_else(|err| {
            error!(
                "Unable to bind tcp socket to {}, error: {}",
                &server_addr, err
            );
            process::exit(1);
        });
    info!("Bound to socket successfully.");

    (handler, listener)
}

fn get_laminar_config() -> laminar::Config {
    laminar::Config {
        idle_connection_timeout: Duration::from_secs(10),
        ..Default::default()
    }
}

pub(super) fn send_message_to_player(
    player_name: &str,
    client_addr: &str,
    currently_online: bool,
    message: &str,
    colour: Option<Color>,
    sender: &Sender<Packet>,
) {
    if currently_online && !client_addr.is_empty() {
        debug!(
            "Sending system message to player {} at: {}",
            &player_name, &client_addr
        );
        let system_message = SystemMessage {
            message: message.to_string(),
            colour,
        };
        let response = encode_to_vec(
            ServerMessage::SystemMessage(system_message),
            config::standard(),
        )
        .unwrap_or_else(|err| {
            error!(
                "Failed to encode system message: {}, error: {}",
                message, err
            );
            process::exit(1);
        });
        rustyhack_lib::network::send_packet(
            Packet::reliable_ordered(client_addr.parse().unwrap(), response, Some(23)),
            sender,
        );
    }
}
