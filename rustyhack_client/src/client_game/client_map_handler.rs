use crate::networking::client_message_handler;
use bincode::{deserialize, serialize};
use crossbeam_channel::{Receiver, Sender};
use itertools::Itertools;
use laminar::Packet;
use rustyhack_lib::background_map::{AllMaps, BackgroundMap};
use rustyhack_lib::message_handler::messages::{PlayerRequest, ServerMessage};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub(crate) fn request_all_maps_data(
    sender: &Sender<Packet>,
    server_addr: &str,
    channel_receiver: &Receiver<ServerMessage>,
) -> AllMaps {
    let get_all_maps_request_packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::GetChunkedAllMaps)
            .expect("Error serializing GetAllMaps request."),
        Some(1),
    );
    client_message_handler::send_packet(get_all_maps_request_packet, sender);
    info!("Requested all maps data from server.");
    wait_for_all_maps_response(channel_receiver)
}

fn wait_for_all_maps_response(channel_receiver: &Receiver<ServerMessage>) -> AllMaps {
    let mut all_maps_downloaded = false;
    let mut all_maps = HashMap::new();
    let mut all_maps_chunks = HashMap::new();
    loop {
        let received = channel_receiver.recv();
        if let Ok(received_message) = received {
            match received_message {
                ServerMessage::AllMapsChunk(message) => {
                    info!("All maps chunk received from server: {}", message.0);
                    all_maps_chunks.insert(message.0, message.1);
                }
                ServerMessage::AllMapsChunksComplete => {
                    info!("All maps chunks downloaded from server.");
                    all_maps = combine_all_maps_chunks(&all_maps_chunks);
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

fn combine_all_maps_chunks(
    all_maps_chunks: &HashMap<usize, Vec<u8>>,
) -> HashMap<String, BackgroundMap> {
    deserialize_all_maps_reply(combine_chunks(all_maps_chunks))
}

fn combine_chunks(all_maps_chunks: &HashMap<usize, Vec<u8>>) -> Vec<u8> {
    let mut combined_all_maps_chunks: Vec<u8> = Vec::new();
    for chunk in all_maps_chunks.keys().sorted() {
        combined_all_maps_chunks.extend(
            all_maps_chunks
                .get(chunk)
                .expect("Error combining all maps chunks on chunk."),
        );
    }
    combined_all_maps_chunks
}

fn deserialize_all_maps_reply(combined_chunks: Vec<u8>) -> HashMap<String, BackgroundMap> {
    let deserialized_chunks = deserialize::<ServerMessage>(&combined_chunks)
        .expect("Error deserializing combined all maps chunks.");
    return if let ServerMessage::AllMaps(message) = deserialized_chunks {
        message
    } else {
        panic!("Combined all maps chunks did not make a valid PlayerReply::AllMaps message");
    };
}
