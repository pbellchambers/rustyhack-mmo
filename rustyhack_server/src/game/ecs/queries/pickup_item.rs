use crate::network_messages::send_message_to_player;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::{Inventory, ItemDetails, PlayerDetails, Position};
use rustyhack_lib::ecs::item::{get_item_name, Item};
use rustyhack_lib::network::packets::PositionMessage;

pub(crate) fn pickup_item(
    world: &mut World,
    position_message: &PositionMessage,
    sender: &Sender<Packet>,
) {
    let mut item_option: Option<Item> = None;
    let mut item_query = <(&mut ItemDetails, &Position, &Item)>::query();

    //confirm item exists at that position and get details
    for (requested_item_details, requested_item_position, requested_item) in
        item_query.iter_mut(world)
    {
        if position_message.position.pos_x == requested_item_position.pos_x
            && position_message.position.pos_y == requested_item_position.pos_y
            && position_message.position.current_map == requested_item_position.current_map
        {
            item_option = Some(requested_item.clone());
            requested_item_details.has_been_picked_up = true;
            break;
        }
    }

    //add item to player carried inventory
    let mut player_query = <(&PlayerDetails, &mut Inventory)>::query();
    for (player_details, player_inventory) in player_query.iter_mut(world) {
        if player_details.player_name == position_message.player_name {
            match item_option {
                None => {
                    debug!("No matching item found.");
                    send_message_to_player(
                        &player_details.player_name,
                        &player_details.client_addr,
                        player_details.currently_online,
                        "No item to pickup.",
                        None,
                        sender,
                    );
                }
                Some(item) => {
                    debug!(
                        "Item found, added to player {} inventory.",
                        player_details.player_name
                    );
                    let item_name = get_item_name(&item);
                    player_inventory.carried.push(item);
                    player_inventory.update_available = true;
                    send_message_to_player(
                        &player_details.player_name,
                        &player_details.client_addr,
                        player_details.currently_online,
                        &("Picked up ".to_string() + &item_name + "."),
                        None,
                        sender,
                    );
                }
            }
            break;
        }
    }
}
