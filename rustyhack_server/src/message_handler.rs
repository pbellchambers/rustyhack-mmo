use bincode::{deserialize, serialize};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::message_handler::player_message::{PlayerMessage, PlayerReply};

pub fn run(
    sender: &Sender<Packet>,
    receiver: &Receiver<SocketEvent>,
    all_maps: &AllMaps,
    channel_sender: Sender<PlayerMessage>,
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

                    let player_message_result = deserialize::<PlayerMessage>(msg);
                    let player_message;
                    match player_message_result {
                        Ok(_) => player_message = player_message_result.unwrap(),
                        Err(error) => {
                            warn!(
                                "Error when deserialising player message from client {}: {}",
                                &packet.addr(),
                                error
                            );
                            //try again with next packet
                            continue;
                        }
                    }
                    debug!("Received {:?} from {:?}", player_message, address);

                    match player_message {
                        PlayerMessage::CreatePlayer(message) => {
                            let response = serialize(&PlayerReply::PlayerCreated)
                                .expect("Error serialising PlayerCreated response.");
                            send_channel_message(
                                PlayerMessage::CreatePlayer(message),
                                &channel_sender,
                            );
                            send_packet(
                                Packet::reliable_unordered(packet.addr(), response),
                                &sender,
                            );
                        }
                        PlayerMessage::UpdateVelocity(message) => {
                            send_channel_message(
                                PlayerMessage::UpdateVelocity(message),
                                &channel_sender,
                            );
                        }
                        PlayerMessage::GetAllMaps => {
                            let response = serialize(&PlayerReply::AllMaps(all_maps.clone()))
                                .expect("Error serialising AllMaps response.");
                            send_packet(
                                Packet::reliable_ordered(packet.addr(), response, Some(2)),
                                &sender,
                            );
                        }
                        _ => {}
                    }
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Client connected from: {}", connect_event)
                }
                SocketEvent::Timeout(address) => {
                    info!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }
}

pub fn send_packet(packet: Packet, sender: &Sender<Packet>) {
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

fn send_channel_message(message: PlayerMessage, sender: &Sender<PlayerMessage>) {
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
