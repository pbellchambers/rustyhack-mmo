use crate::game::ecs::queries;
use crate::game::ecs::queries::{common_player, player_joined, player_left};
use crossbeam_channel::{Receiver, Sender};
use laminar::Packet;
use legion::World;
use rustyhack_lib::network::packets::PlayerRequest;

//todo resolve this clippy warning
#[allow(clippy::too_many_lines)]
pub(super) fn process_player_messages(
    world: &mut World,
    channel_receiver: &Receiver<PlayerRequest>,
    sender: &Sender<Packet>,
) -> bool {
    let mut has_player_updates = false;
    while !channel_receiver.is_empty() {
        debug!("Player messages are present.");
        let received = channel_receiver.try_recv();
        if let Ok(received_message) = received {
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
                    debug!(
                        "Velocity update received for {}",
                        &position_message.player_name
                    );
                    common_player::set_player_velocity(world, &position_message);
                    debug!("Processed velocity update.");
                }
                PlayerRequest::PickupItem(position_message) => {
                    debug!(
                        "Pickup item request received from {} at ({},{}) on {} map.",
                        &position_message.player_name,
                        &position_message.position.pos_x,
                        &position_message.position.pos_y,
                        &position_message.position.current_map,
                    );
                    queries::pickup_item::pickup_item(world, &position_message, sender);
                    debug!("Processed item pickup request.");
                }
                PlayerRequest::DropItem(drop_item_details) => {
                    debug!(
                        "Drop item request received from {} at ({},{}) on {} map.",
                        &drop_item_details.1.player_name,
                        &drop_item_details.1.position.pos_x,
                        &drop_item_details.1.position.pos_y,
                        &drop_item_details.1.position.current_map,
                    );
                    queries::drop_item::drop_item(
                        world,
                        drop_item_details.0,
                        &drop_item_details.1,
                        sender,
                    );
                    debug!("Processed item pickup request.");
                }
                PlayerRequest::StatUp(stat_up_details) => {
                    debug!(
                        "Stat up request received from {} for {}.",
                        &stat_up_details.1, &stat_up_details.0,
                    );
                    queries::increase_stat::increase_stat(
                        world,
                        &stat_up_details.0,
                        &stat_up_details.1,
                        sender,
                    );
                    debug!("Processed stat up request.");
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
            has_player_updates = true;
        } else {
            debug!("Player messages channel receiver is now empty.");
        }
    }
    has_player_updates
}
