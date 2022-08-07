use bincode::{deserialize, serialize};
use crossbeam_channel::{Receiver, Sender};
use itertools::Itertools;
use laminar::Packet;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::network::packets::{PlayerRequest, ServerMessage};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub(crate) fn request_all_maps_data(
    sender: &Sender<Packet>,
    server_addr: &str,
    channel_receiver: &Receiver<ServerMessage>,
) -> Option<AllMaps> {
    let get_all_maps_request_packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::GetChunkedAllMaps)
            .expect("Error serializing GetAllMaps request."),
        Some(1),
    );
    rustyhack_lib::network::send_packet(get_all_maps_request_packet, sender);
    info!("Requested all maps data from server.");
    wait_for_all_maps_response(channel_receiver)
}

fn wait_for_all_maps_response(channel_receiver: &Receiver<ServerMessage>) -> Option<AllMaps> {
    let mut all_maps_downloaded = false;
    let mut all_maps = None;
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
                    );
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

fn combine_all_maps_chunks(all_maps_chunks: &HashMap<usize, Vec<u8>>) -> Option<AllMaps> {
    deserialize_all_maps_reply(&combine_chunks(all_maps_chunks))
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

fn deserialize_all_maps_reply(combined_chunks: &[u8]) -> Option<AllMaps> {
    let deserialized_chunks = deserialize::<ServerMessage>(combined_chunks);
    match deserialized_chunks {
        Ok(deserialized) => {
            if let ServerMessage::AllMaps(all_maps) = deserialized {
                Some(all_maps)
            } else {
                warn!("Deserialized message from server was not all maps, will request again.");
                None
            }
        }
        Err(error) => {
            warn!(
                "Error deserializing all maps from server, will request again. {}",
                error.to_string()
            );
            None
        }
    }
}
