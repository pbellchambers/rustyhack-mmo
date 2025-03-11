use crate::network_messages::packet_receiver::decode_player_request;
use bincode::{config, encode_to_vec};
use message_io::network::NetEvent;
use message_io::node::{NodeHandler, NodeListener};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::network::packets::{PlayerRequest, ServerMessage};
use std::thread;

pub(crate) fn spawn_map_sender_thread(
    tcp_handler: NodeHandler<()>,
    tcp_listener: NodeListener<()>,
    all_maps: AllMaps,
) {
    thread::spawn(move || run(tcp_handler, tcp_listener, &all_maps));
}

fn run(tcp_handler: NodeHandler<()>, tcp_listener: NodeListener<()>, all_maps: &AllMaps) {
    info!("Spawned tcp listener thread.");
    tcp_listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
        NetEvent::Accepted(endpoint, _listener) => {
            info!("Client {} connected via tcp.", endpoint.addr());
        }
        NetEvent::Message(endpoint, data) => {
            let decoded_data = decode_player_request(data, endpoint.addr());
            match decoded_data {
                PlayerRequest::GetAllMaps => {
                    info!("Sending all_maps data to {}.", endpoint.addr());
                    tcp_handler
                        .network()
                        .send(endpoint, &encode_all_maps(all_maps.clone()));
                }
                _ => {
                    warn!("Ignoring unexpected player request type on tcp connection.");
                }
            }
        }
        NetEvent::Disconnected(endpoint) => info!(
            "Client {} disconnected from tcp connection.",
            endpoint.addr()
        ),
    });
}

fn encode_all_maps(all_maps: AllMaps) -> Vec<u8> {
    encode_to_vec(ServerMessage::AllMaps(all_maps), config::standard())
        .expect("Error encoding AllMaps data.")
}
