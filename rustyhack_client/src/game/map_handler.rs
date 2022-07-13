use crate::networking::message_handler;
use bincode::{deserialize, serialize};
use crossbeam_channel::{Receiver, Sender};
use laminar::Packet;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::message_handler::player_message::{PlayerMessage, PlayerReply};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use itertools::Itertools;

pub(crate) fn request_all_maps_data(
    sender: &Sender<Packet>,
    server_addr: &str,
    channel_receiver: &Receiver<PlayerReply>,
) -> AllMaps {
    let get_all_maps_request_packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerMessage::GetChunkedAllMaps).expect("Error serialising GetAllMaps request."),
        Some(1),
    );
    message_handler::send_packet(get_all_maps_request_packet, sender);
    info!("Requested all maps data from server.");
    wait_for_all_maps_response(channel_receiver)
}

fn wait_for_all_maps_response(channel_receiver: &Receiver<PlayerReply>) -> AllMaps {
    let mut all_maps_downloaded = false;
    let mut all_maps = HashMap::new();
    let mut all_maps_chunks = HashMap::new();
    loop {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                PlayerReply::AllMaps(message) => {
                    info!("All maps downloaded from server.");
                    all_maps = message;
                }
                PlayerReply::AllMapsChunk(message) => {
                    info!("All maps chunk received from server: {}", message.0);
                    all_maps_chunks.insert(message.0, message.1);
                }
                PlayerReply::AllMapsChunksComplete => {
                    info!("All maps chunks downloaded from server.");
                    let mut combined_all_maps_chunks: Vec<u8> = Vec::new();
                    for chunk in all_maps_chunks.keys().sorted() {
                        combined_all_maps_chunks.extend(all_maps_chunks.get(chunk).unwrap());
                    }
                    let deserialized_chunks = deserialize::<PlayerReply>(&combined_all_maps_chunks).unwrap();
                    if let PlayerReply::AllMaps(message) = deserialized_chunks {
                        all_maps = message;
                    }
                    all_maps_downloaded = true;
                }
                _ => {
                    info!(
                        "Ignoring other message types until maps downloaded. {:?}",
                        received_message
                    )
                }
            }
        }
        if all_maps_downloaded {
            info!("Got all maps data.");
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    debug!("All maps is: {:?}", all_maps);
    all_maps
}
