use bincode::{config, encode_to_vec};
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::{ClientDetails, PlayerRequest};

pub(crate) fn send_logout_notification(sender: &Sender<Packet>, player: Player, server_addr: &str) {
    let logout_notification_packet = Packet::reliable_unordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        encode_to_vec(
            PlayerRequest::PlayerLogout(ClientDetails {
                client_addr: player.player_details.client_addr,
                player_name: player.player_details.player_name,
            }),
            config::standard(),
        )
        .unwrap(),
    );
    rustyhack_lib::network::send_packet(logout_notification_packet, sender);
    info!("Logout notification sent to server.");
}
