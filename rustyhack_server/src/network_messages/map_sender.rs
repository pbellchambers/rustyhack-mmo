use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::network::packets::ServerMessage;
use std::net::SocketAddr;

pub(super) fn serialize_all_maps(all_maps: AllMaps) -> Vec<u8> {
    serialize(&ServerMessage::AllMaps(all_maps)).expect("Error serializing AllMaps data.")
}

pub(super) fn send_all_maps_chunks(
    all_maps_serialized: &[u8],
    address: SocketAddr,
    sender: &Sender<Packet>,
) {
    let all_maps_chunks = all_maps_serialized
        .chunks(1450)
        .map(std::convert::Into::into)
        .enumerate();
    let chunked_response_length = all_maps_chunks.size_hint();

    let mut stream_id: u8 = 1;
    for (chunk_count, chunk) in all_maps_chunks {
        let chunk_packet = serialize(&ServerMessage::AllMapsChunk((chunk_count, chunk)))
            .expect("Error serializing AllMapsChunk.");
        if chunk_count == 0 {
            info!(
                "Sending first AllMapsChunk packet {} to: {}",
                chunk_count, address
            );
            rustyhack_lib::network::send_packet(
                Packet::reliable_ordered(address, chunk_packet, Some(stream_id)),
                sender,
            );
        } else if chunk_count
            == chunked_response_length
                .1
                .expect("Error: chunked all maps length is zero")
                - 1
        {
            info!(
                "Sending last AllMapsChunk packet {} to: {}",
                chunk_count, address
            );
            rustyhack_lib::network::send_packet(
                Packet::reliable_ordered(address, chunk_packet, Some(stream_id)),
                sender,
            );

            let complete_response = serialize(&ServerMessage::AllMapsChunksComplete)
                .expect("Error serializing AllMapsChunksComplete response.");
            rustyhack_lib::network::send_packet(
                Packet::reliable_ordered(address, complete_response, Some(stream_id + 1)),
                sender,
            );
        } else {
            debug!(
                "Sending AllMapsChunk packet {} to: {}",
                chunk_count, address
            );
            rustyhack_lib::network::send_packet(
                Packet::reliable_ordered(address, chunk_packet, Some(stream_id)),
                sender,
            );
        }
        stream_id += 1;
        if stream_id > (u8::MAX - 1) {
            stream_id = 1;
        }
    }
}
