use crate::networking::message_handler;
use bincode::serialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::player_message::{
    CreatePlayerMessage, PlayerMessage, PlayerReply,
};
use std::time::Duration;
use std::{process, thread};

pub(crate) fn send_new_player_request(
    sender: &Sender<Packet>,
    player_name: &str,
    server_addr: &str,
    client_addr: &str,
    channel_receiver: &Receiver<PlayerReply>,
) -> Player {
    let create_player_request_packet = Packet::reliable_unordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerMessage::PlayerJoin(CreatePlayerMessage {
            client_addr: client_addr.to_string(),
            player_name: player_name.to_string(),
        }))
        .unwrap(),
    );
    message_handler::send_packet(create_player_request_packet, sender);
    info!("Sent new player request to server.");
    wait_for_new_player_response(channel_receiver)
}

fn wait_for_new_player_response(channel_receiver: &Receiver<PlayerReply>) -> Player {
    let mut new_player_confirmed = false;
    let mut player = Player::default();
    loop {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerReply::PlayerJoined(message) => {
                    info!("New player creation confirmed.");
                    new_player_confirmed = true;
                    player = message;
                }
                PlayerReply::PlayerAlreadyOnline => {
                    error!(
                        "This player name is already taken, and the player is currently online."
                    );
                    process::exit(1);
                }
                _ => {
                    info!(
                        "Ignoring other message types until new player confirmed. {:?}",
                        received_message
                    )
                }
            }
        }
        if new_player_confirmed {
            info!("Got all data needed to begin game.");
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    info!("player_name is: {}", player.player_details.player_name);
    player
}
