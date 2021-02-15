use bincode::deserialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use rustyhack_lib::message_handler::player_message::PlayerReply;
use std::process;
use std::time::Duration;

pub fn run(
    _sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    player_update_sender: Sender<PlayerReply>,
    entity_update_sender: Sender<PlayerReply>,
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

                    let player_reply_result = deserialize::<PlayerReply>(msg);
                    let player_reply;
                    match player_reply_result {
                        Ok(_) => player_reply = player_reply_result.unwrap(),
                        Err(error) => {
                            warn!(
                                "Error when deserialising player reply packet from server: {}",
                                error
                            );
                            //try again with next packet
                            continue;
                        }
                    }
                    debug!("Received {:?} from {:?}", player_reply, address);

                    let channel_send_status;
                    match player_reply {
                        PlayerReply::PlayerCreated(message) => {
                            channel_send_status =
                                player_update_sender.send(PlayerReply::PlayerCreated(message));
                        }
                        PlayerReply::AllMaps(message) => {
                            channel_send_status =
                                player_update_sender.send(PlayerReply::AllMaps(message));
                        }
                        PlayerReply::UpdatePosition(message) => {
                            channel_send_status =
                                player_update_sender.send(PlayerReply::UpdatePosition(message));
                        }
                        PlayerReply::UpdateOtherEntities(message) => {
                            channel_send_status = entity_update_sender
                                .send(PlayerReply::UpdateOtherEntities(message));
                        }
                    }

                    match channel_send_status {
                        Ok(_) => {
                            //do nothing
                        }
                        Err(message) => {
                            warn!("Failed to send message via thread channel: {}", &message);
                        }
                    }
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Server connected at: {}", connect_event)
                }
                SocketEvent::Timeout(address) => {
                    error!("Server connection timed out: {}", address);
                    warn!("Please check that the server is online and the address is correct.");
                    process::exit(1);
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
            //packet send successful
        }
        Err(message) => {
            warn!("Error sending packet: {}", message);
            warn!("Will try to continue, but things may be broken.");
        }
    }
}

pub fn get_laminar_config() -> laminar::Config {
    laminar::Config {
        idle_connection_timeout: Duration::from_secs(10),
        heartbeat_interval: Some(Duration::from_secs(2)),
        max_fragments: 255,
        ..Default::default()
    }
}
