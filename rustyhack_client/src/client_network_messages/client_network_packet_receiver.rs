use bincode::deserialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::SocketEvent;
use rustyhack_lib::network::packets::ServerMessage;
use std::{process, thread};

pub(crate) fn spawn_network_packet_receiver_thread(
    receiver: Receiver<SocketEvent>,
    incoming_server_messages: Sender<ServerMessage>,
) {
    thread::spawn(move || {
        run(&receiver, &incoming_server_messages);
    });
}

pub(crate) fn run(
    receiver: &Receiver<SocketEvent>,
    incoming_server_messages: &Sender<ServerMessage>,
) {
    info!("Spawned network packet receiver thread.");
    loop {
        debug!("Waiting for packet to be received.");
        if let Ok(event) = receiver.recv() {
            debug!("Packet received. Processing...");
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let address = packet.addr();

                    let player_reply_result = deserialize::<ServerMessage>(msg);
                    let player_reply = match player_reply_result {
                        Ok(_) => player_reply_result.unwrap(),
                        Err(error) => {
                            warn!(
                                "Error when deserializing player reply packet from server: {}",
                                error
                            );
                            //try again with next packet
                            continue;
                        }
                    };
                    debug!("Received {:?} from {:?}", player_reply, address);

                    let channel_send_status =
                        match player_reply {
                            ServerMessage::PlayerJoined(player) => {
                                incoming_server_messages.send(ServerMessage::PlayerJoined(player))
                            }
                            ServerMessage::AllMaps(all_maps) => {
                                incoming_server_messages.send(ServerMessage::AllMaps(all_maps))
                            }
                            ServerMessage::AllMapsChunk(all_maps_chunk) => incoming_server_messages
                                .send(ServerMessage::AllMapsChunk(all_maps_chunk)),
                            ServerMessage::AllMapsChunksComplete => {
                                incoming_server_messages.send(ServerMessage::AllMapsChunksComplete)
                            }
                            ServerMessage::UpdatePosition(position) => incoming_server_messages
                                .send(ServerMessage::UpdatePosition(position)),
                            ServerMessage::UpdateOtherEntities(entity_position_broadcast) => {
                                incoming_server_messages.send(ServerMessage::UpdateOtherEntities(
                                    entity_position_broadcast,
                                ))
                            }
                            ServerMessage::PlayerAlreadyOnline => {
                                incoming_server_messages.send(ServerMessage::PlayerAlreadyOnline)
                            }
                            ServerMessage::UpdateStats(stats) => {
                                incoming_server_messages.send(ServerMessage::UpdateStats(stats))
                            }
                            ServerMessage::UpdateInventory(inventory) => incoming_server_messages
                                .send(ServerMessage::UpdateInventory(inventory)),
                            ServerMessage::SystemMessage(message) => {
                                incoming_server_messages.send(ServerMessage::SystemMessage(message))
                            }
                        };

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
                    info!("Server connected at: {}", connect_event);
                }
                SocketEvent::Timeout(address) => {
                    error!("Server connection timed out: {}", address);
                    warn!("Please check that the server is online and the address is correct.");
                    process::exit(1);
                }
                SocketEvent::Disconnect(address) => {
                    error!("Server connection disconnected: {}", address);
                    warn!("Please check that the server is online and the address is correct.");
                    process::exit(1);
                }
            }
        }
    }
}
