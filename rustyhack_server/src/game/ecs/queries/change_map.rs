use crate::game::map::exits::AllMapExits;
use crate::network_messages::send_message_to_player;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::PlayerDetails;
use rustyhack_lib::network::packets::PositionMessage;

pub(crate) fn change_map_request(
    world: &mut World,
    all_map_exits: &AllMapExits,
    position_message: &PositionMessage,
    sender: &Sender<Packet>,
) {
    let no_exits_vec = vec![];
    let current_map_exits = all_map_exits
        .get(&position_message.position.current_map)
        .unwrap_or(&no_exits_vec);
    let mut changed_map = false;

    let mut query = <&PlayerDetails>::query();
    for player_details in query.iter_mut(world) {
        if player_details.player_name == position_message.player_name
            && player_details.currently_online
        {
            for exit in current_map_exits {
                if position_message.position.pos_x == exit.x
                    && position_message.position.pos_y == exit.y
                {
                    //todo actually change map
                    changed_map = true;
                    break;
                }
            }

            if !changed_map {
                debug!(
                    "No map exit found at this location for player {}.",
                    position_message.player_name
                );
                send_message_to_player(
                    &position_message.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    "No map exit found here.",
                    None,
                    sender,
                );
            }
            break;
        }
    }
}
