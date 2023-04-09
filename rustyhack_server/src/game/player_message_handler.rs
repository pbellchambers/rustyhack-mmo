use crate::game::ecs::queries;
use crate::game::ecs::queries::{common_player, player_joined, player_left};
use crate::game::map::exits::AllMapExits;
use crossbeam_channel::{Receiver, Sender};
use laminar::Packet;
use legion::World;
use rustyhack_lib::network::packets::PlayerRequest;

pub(super) fn process_player_messages(
    world: &mut World,
    all_map_exits: &AllMapExits,
    channel_receiver: &Receiver<PlayerRequest>,
    sender: &Sender<Packet>,
) -> bool {
    let mut has_player_updates = false;
    while !channel_receiver.is_empty() {
        debug!("Player messages are present.");
        let received = channel_receiver.try_recv();
        if let Ok(received_message) = received {
            match_received_message(received_message, world, all_map_exits, sender);
            has_player_updates = true;
        } else {
            debug!("Player messages channel receiver is now empty.");
        }
    }
    has_player_updates
}

fn match_received_message(
    received_message: PlayerRequest,
    world: &mut World,
    all_map_exits: &AllMapExits,
    sender: &Sender<Packet>,
) {
    match received_message {
        PlayerRequest::PlayerJoin(client_details) => {
            info!(
                "Player joined request received for {} from: {}",
                &client_details.player_name, &client_details.client_addr
            );
            player_joined::join_player(
                world,
                &client_details.player_name,
                client_details.client_addr,
                sender,
            );
        }
        PlayerRequest::UpdateVelocity(position_message) => {
            common_player::set_player_velocity(world, &position_message);
        }
        PlayerRequest::PickupItem(position_message) => {
            queries::pickup_item::pickup_item(world, &position_message, sender);
        }
        PlayerRequest::DropItem(drop_item_details) => {
            queries::drop_item::drop_item(world, drop_item_details.0, &drop_item_details.1, sender);
        }
        PlayerRequest::ChangeMap(position_message) => {
            queries::change_map::change_map_request(
                world,
                all_map_exits,
                &position_message,
                sender,
            );
        }
        PlayerRequest::StatUp(stat_up_details) => {
            queries::increase_stat::increase_stat(
                world,
                &stat_up_details.0,
                &stat_up_details.1,
                sender,
            );
        }
        PlayerRequest::PlayerLogout(client_details) => {
            info!(
                "Player logout notification received for {} from: {}",
                &client_details.player_name, &client_details.client_addr
            );
            let (logged_out_player_id, logged_out_map) = player_left::set_player_logged_out(
                world,
                &client_details.client_addr,
                &client_details.player_name,
            );
            player_left::broadcast_player_logged_out(
                world,
                sender,
                logged_out_player_id,
                &logged_out_map,
            );
        }
        PlayerRequest::Timeout(address) => {
            let (logged_out_player_id, logged_out_map) =
                player_left::set_player_disconnected(world, &address);
            player_left::broadcast_player_logged_out(
                world,
                sender,
                logged_out_player_id,
                &logged_out_map,
            );
        }
        _ => {
            warn!("Didn't match any known message to process.");
        }
    }
}
