use crate::network_messages::send_message_to_player;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::consts::{DEFAULT_ITEM_COLOUR, DEFAULT_ITEM_ICON};
use rustyhack_lib::ecs::components::{
    DisplayDetails, Inventory, ItemDetails, PlayerDetails, Position,
};
use rustyhack_lib::ecs::item::{Item, get_item_name};
use rustyhack_lib::network::packets::PositionMessage;
use uuid::Uuid;

pub(crate) fn drop_item(
    world: &mut World,
    item_index: u16,
    position_message: &PositionMessage,
    sender: &Sender<Packet>,
) {
    //remove item from player inventory and add it to world
    let mut query = <(&PlayerDetails, &Position, &mut Inventory)>::query();
    for (player_details, position, player_inventory) in query.iter_mut(world) {
        if player_details.player_name == position_message.player_name {
            if !player_inventory.carried.is_empty() {
                let dropped_item: (ItemDetails, DisplayDetails, Position, Item) = (
                    ItemDetails {
                        id: Uuid::new_v4(),
                        has_been_picked_up: false,
                    },
                    DisplayDetails {
                        icon: DEFAULT_ITEM_ICON,
                        colour: DEFAULT_ITEM_COLOUR,
                        visible: true,
                        collidable: false,
                    },
                    Position {
                        update_available: true,
                        pos_x: position.pos_x,
                        pos_y: position.pos_y,
                        current_map: position.current_map.clone(),
                        velocity_x: 0,
                        velocity_y: 0,
                    },
                    player_inventory.carried[item_index as usize].clone(),
                );
                let item_name = get_item_name(&dropped_item.3);
                player_inventory.carried.remove(item_index as usize);
                player_inventory.update_available = true;
                send_message_to_player(
                    &player_details.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    &("Dropped ".to_string() + &item_name + "."),
                    None,
                    sender,
                );
                world.push(dropped_item);
            }
            break;
        }
    }
}
