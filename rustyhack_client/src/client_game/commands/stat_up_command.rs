use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::PlayerRequest;

pub(crate) fn send_stat_up_request(
    sender: &Sender<Packet>,
    player: &Player,
    server_addr: &str,
    stat: &str,
) {
    let packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::StatUp((
            stat.to_string(),
            player.player_details.player_name.clone(),
        )))
        .unwrap(),
        Some(13),
    );
    rustyhack_lib::message_handler::send_packet(packet, sender);
    info!("Sent stat up request packet to server for {}.", stat);
}
