use bincode::{deserialize, serialize};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::message_handler::messages::{PlayerRequest, ServerMessage};
use std::net::SocketAddr;
use std::thread;

pub(crate) fn spawn_message_handler_thread(
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    all_maps: AllMaps,
    channel_sender: Sender<PlayerRequest>,
) {
    thread::spawn(move || run(sender, receiver, all_maps, channel_sender));
}

pub(crate) fn run(
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    all_maps: AllMaps,
    channel_sender: Sender<PlayerRequest>,
) {
    info!("Spawned message handler thread.");
    loop {
        debug!("Waiting for packet to be received.");
        if let Ok(event) = receiver.recv() {
            debug!("Packet received. Processing...");
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let address = packet.addr();

                    let player_request = deserialize_player_request(msg, address);
                    debug!("Received {:?} from {:?}", player_request, address);

                    match player_request {
                        PlayerRequest::PlayerJoin(message) => {
                            let mut create_player_request = message.clone();
                            create_player_request.client_addr = address.to_string();
                            send_channel_message(
                                PlayerRequest::PlayerJoin(create_player_request),
                                &channel_sender,
                            );
                        }
                        PlayerRequest::UpdateVelocity(message) => {
                            send_channel_message(
                                PlayerRequest::UpdateVelocity(message),
                                &channel_sender,
                            );
                        }
                        PlayerRequest::GetChunkedAllMaps => {
                            send_all_maps_chunks(
                                serialize_all_maps(all_maps.clone()),
                                address,
                                &sender,
                            );
                        }
                        PlayerRequest::PlayerLogout(message) => {
                            send_channel_message(
                                PlayerRequest::PlayerLogout(message),
                                &channel_sender,
                            );
                        }
                        _ => {}
                    }
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Client connected from: {}", connect_event)
                }
                SocketEvent::Disconnect(address) => {
                    info!("Client disconnected from: {}", address);
                    send_channel_message(
                        PlayerRequest::Timeout(address.to_string()),
                        &channel_sender,
                    );
                }
                SocketEvent::Timeout(address) => {
                    info!("Client timed out from: {}", address);
                    send_channel_message(
                        PlayerRequest::Timeout(address.to_string()),
                        &channel_sender,
                    );
                }
            }
        }
    }
}

fn serialize_all_maps(all_maps: AllMaps) -> Vec<u8> {
    serialize(&ServerMessage::AllMaps(all_maps)).expect("Error serializing AllMaps data.")
}

fn send_all_maps_chunks(
    all_maps_serialized: Vec<u8>,
    address: SocketAddr,
    sender: &Sender<Packet>,
) {
    let all_maps_chunks = all_maps_serialized
        .chunks(1450)
        .map(|s| s.into())
        .enumerate();
    let chunked_response_length = all_maps_chunks.size_hint();

    for (i, chunk) in all_maps_chunks {
        let chunk_packet = serialize(&ServerMessage::AllMapsChunk((i, chunk)))
            .expect("Error serializing AllMapsChunk.");
        if i == 0 {
            info!("Sending first AllMapsChunk packet {} to: {}", i, address);
            send_packet(
                Packet::reliable_ordered(address, chunk_packet, Some(i as u8)),
                sender,
            );
        } else if i
            == chunked_response_length
                .1
                .expect("Error: chunked all maps length is zero")
                - 1
        {
            info!("Sending last AllMapsChunk packet {} to: {}", i, address);
            send_packet(
                Packet::reliable_ordered(address, chunk_packet, Some(i as u8)),
                sender,
            );

            let complete_response = serialize(&ServerMessage::AllMapsChunksComplete)
                .expect("Error serializing AllMapsChunksComplete response.");
            send_packet(
                Packet::reliable_ordered(address, complete_response, Some(i as u8 + 1)),
                sender,
            );
        } else {
            debug!("Sending AllMapsChunk packet {} to: {}", i, address);
            send_packet(
                Packet::reliable_ordered(address, chunk_packet, Some(i as u8)),
                sender,
            );
        }
    }
}

fn deserialize_player_request(msg: &[u8], address: SocketAddr) -> PlayerRequest {
    let player_request_result = deserialize::<PlayerRequest>(msg);
    match player_request_result {
        Ok(_) => player_request_result.unwrap(),
        Err(error) => {
            warn!(
                "Error when deserializing player request from client {}: {}",
                &address, error
            );
            PlayerRequest::Undefined
        }
    }
}

pub(crate) fn send_packet(packet: Packet, sender: &Sender<Packet>) {
    let send_result = sender.send(packet);
    match send_result {
        Ok(_) => {
            //send successful
        }
        Err(message) => {
            warn!("Error sending packet: {}", message);
            warn!("Will try to continue, but things may be broken.");
        }
    }
}

fn send_channel_message(message: PlayerRequest, sender: &Sender<PlayerRequest>) {
    let send_result = sender.send(message);
    match send_result {
        Ok(_) => {
            //send successful
        }
        Err(message) => {
            warn!("Error sending channel message: {}", message);
            warn!("Will try to continue, but things may be broken.");
        }
    }
}
