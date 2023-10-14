use bincode::deserialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::SocketEvent;
use rustyhack_lib::network::packets::PlayerRequest;
use std::net::SocketAddr;
use std::thread;

pub(crate) fn spawn_packet_receiver_thread(
    receiver: Receiver<SocketEvent>,
    channel_sender: Sender<PlayerRequest>,
) {
    thread::spawn(move || run(&receiver, &channel_sender));
}

fn run(receiver: &Receiver<SocketEvent>, channel_sender: &Sender<PlayerRequest>) {
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

                    handle_player_request(player_request, address, channel_sender);
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Client connected from: {}", connect_event);
                }
                SocketEvent::Disconnect(address) => {
                    info!("Client disconnected from: {}", address);
                    send_channel_message(
                        PlayerRequest::Timeout(address.to_string()),
                        channel_sender,
                    );
                }
                SocketEvent::Timeout(address) => {
                    info!("Client timed out from: {}", address);
                    send_channel_message(
                        PlayerRequest::Timeout(address.to_string()),
                        channel_sender,
                    );
                }
            }
        }
    }
}

fn handle_player_request(
    player_request: PlayerRequest,
    address: SocketAddr,
    channel_sender: &Sender<PlayerRequest>,
) {
    match player_request {
        PlayerRequest::PlayerJoin(message) => {
            let mut create_player_request = message;
            create_player_request.client_addr = address.to_string();
            send_channel_message(
                PlayerRequest::PlayerJoin(create_player_request),
                channel_sender,
            );
        }
        PlayerRequest::UpdateVelocity(position_message) => {
            send_channel_message(
                PlayerRequest::UpdateVelocity(position_message),
                channel_sender,
            );
        }
        PlayerRequest::PickupItem(position_message) => {
            send_channel_message(PlayerRequest::PickupItem(position_message), channel_sender);
        }
        PlayerRequest::ChangeMap(position_message) => {
            send_channel_message(PlayerRequest::ChangeMap(position_message), channel_sender);
        }
        PlayerRequest::DropItem(drop_item_details) => {
            send_channel_message(PlayerRequest::DropItem(drop_item_details), channel_sender);
        }
        PlayerRequest::StatUp(stat_up_details) => {
            send_channel_message(PlayerRequest::StatUp(stat_up_details), channel_sender);
        }
        PlayerRequest::GetAllMaps => {
            warn!("Ignoring unexpected GetAllMaps request on udp port.");
        }
        PlayerRequest::PlayerLogout(client_details) => {
            send_channel_message(PlayerRequest::PlayerLogout(client_details), channel_sender);
        }
        PlayerRequest::Timeout(_) => {
            info!("Client timed out from: {}", address);
            send_channel_message(PlayerRequest::Timeout(address.to_string()), channel_sender);
        }
        PlayerRequest::Undefined => {
            warn!("Undefined message received from {}", address);
        }
    }
}

pub(super) fn deserialize_player_request(msg: &[u8], address: SocketAddr) -> PlayerRequest {
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

fn send_channel_message(message: PlayerRequest, channel_sender: &Sender<PlayerRequest>) {
    let send_result = channel_sender.send(message);

    #[allow(clippy::ignored_unit_patterns)]
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
