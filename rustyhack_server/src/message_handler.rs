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
    loop {
        info!("Waiting for packet to be received.");
        if let Ok(event) = receiver.recv() {
            info!("Packet received. Processing...");
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let address = packet.addr();
                    let player_message =
                        deserialize::<PlayerMessage>(msg).expect(&*String::from_utf8_lossy(msg));
                    info!("Received {:?} from {:?}", player_message, address);

                    match player_message {
                        PlayerMessage::CreatePlayer(message) => {
                            let response = serialize(&PlayerReply::PlayerCreated).unwrap();
                            channel_sender
                                .send(PlayerMessage::CreatePlayer(message))
                                .expect("Create player thread message didn't send.");
                            sender
                                .send(Packet::reliable_unordered(packet.addr(), response))
                                .expect("Player created reply didn't send.");
                        }
                        PlayerMessage::UpdateVelocity(message) => {
                            channel_sender
                                .send(PlayerMessage::UpdateVelocity(message))
                                .expect("Update velocity thread message didn't send.");
                        }
                        PlayerMessage::GetAllMaps => {
                            let response =
                                serialize(&PlayerReply::AllMaps(all_maps.clone())).unwrap();
                            sender
                                .send(Packet::reliable_ordered(packet.addr(), response, Some(2)))
                                .expect("Get all maps reply didn't send.");
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
