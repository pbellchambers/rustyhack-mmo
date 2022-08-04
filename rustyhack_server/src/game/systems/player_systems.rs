use crate::consts::{BASE_HP_TABLE, CUMULATIVE_EXP_TABLE, EXP_LOSS_ON_DEATH_PERCENTAGE};
use crate::game::player_updates::send_message_to_player;
use crate::game::players::PlayersPositions;
use crossbeam_channel::Sender;
use crossterm::style::Color;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, IntoQuery, Query, World};
use rustyhack_lib::consts::{DEFAULT_ITEM_COLOUR, DEFAULT_ITEM_ICON};
use rustyhack_lib::ecs::components::{
    DisplayDetails, Inventory, ItemDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::item::{get_item_name, Item};
use rustyhack_lib::message_handler::messages::PositionMessage;
use uuid::Uuid;

#[system]
pub(crate) fn resolve_player_deaths(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &mut Position, &mut Stats)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (player_details, position, stats) in query.iter_mut(world) {
        if stats.current_hp <= 0.0 {
            let mut exp_loss = 0;
            if stats.exp > 100 {
                exp_loss = (stats.exp * EXP_LOSS_ON_DEATH_PERCENTAGE) / 100;
                stats.exp -= exp_loss;
            }
            stats.current_hp = stats.max_hp;
            stats.in_combat = false;
            stats.update_available = true;
            *position = Position::default();
            position.update_available = true;
            if exp_loss > 0 {
                send_message_to_player(
                    &player_details.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    &("You lost ".to_string() + &exp_loss.to_string() + " exp."),
                    Some(Color::DarkYellow),
                    sender,
                );
            }
            send_message_to_player(
                &player_details.player_name,
                &player_details.client_addr,
                player_details.currently_online,
                "Now respawning at respawn point...",
                None,
                sender,
            );
        }
    }
}

#[system]
pub(crate) fn update_player_positions_resource(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &Position)>,
    #[resource] players_positions: &mut PlayersPositions,
) {
    players_positions.clear();
    for (player_details, position) in query.iter(world) {
        if player_details.currently_online {
            players_positions.insert(player_details.id, position.clone());
        }
    }
}

#[system]
pub(crate) fn level_up(
    world: &mut SubWorld,
    query: &mut Query<(&mut Stats, Option<&PlayerDetails>)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (mut stats, player_details_option) in query.iter_mut(world) {
        if let Some(player_details) = player_details_option {
            if stats.exp >= stats.exp_next && stats.level < 100 {
                info!(
                    "Player {} levelled up from {} to {}!",
                    player_details.player_name,
                    stats.level,
                    stats.level + 1
                );
                stats.level += 1;
                if stats.level >= 100 {
                    stats.exp_next = 0;
                } else {
                    stats.exp_next = CUMULATIVE_EXP_TABLE[(stats.level - 1) as usize];
                }
                stats = calculate_new_stats(stats);
                stats.update_available = true;
                send_message_to_player(
                    &player_details.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    "You levelled up, 2 new stat points available to spend!",
                    Some(Color::Cyan),
                    sender,
                );
            }
        }
    }
}

fn calculate_new_stats(stats: &mut Stats) -> &mut Stats {
    // 2 new stat points are given on each level up
    // HP increases by 25
    // HP is increased by Constitution %
    // need to recalculate HP whenever player increases con
    stats.stat_points += 2;
    stats.max_hp =
        (BASE_HP_TABLE[(stats.level - 1) as usize] * (1.0 + (stats.con / 100.0))).round();
    stats
}

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

pub(crate) fn drop_item(
    world: &mut World,
    item_index: u16,
    position_message: &PositionMessage,
    sender: &Sender<Packet>,
) {
    //remove item from player inventory and add it to world
    let mut player_query = <(&PlayerDetails, &Position, &mut Inventory)>::query();
    for (player_details, position, player_inventory) in player_query.iter_mut(world) {
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

pub(crate) fn increase_stat(
    world: &mut World,
    stat: &str,
    player_name: &str,
    sender: &Sender<Packet>,
) {
    let mut player_query = <(&PlayerDetails, &mut Stats)>::query();
    for (player_details, stats) in player_query.iter_mut(world) {
        if player_details.player_name == player_name && stats.stat_points > 0 {
            let mut updated_stat = false;
            match stat {
                "Str" => {
                    if stats.str < 100.0 {
                        stats.str += 1.0;
                        stats.stat_points -= 1;
                        updated_stat = true;
                    }
                }
                "Dex" => {
                    if stats.dex < 100.0 {
                        stats.dex += 1.0;
                        stats.stat_points -= 1;
                        updated_stat = true;
                    }
                }
                "Con" => {
                    if stats.con < 100.0 {
                        stats.con += 1.0;
                        stats.stat_points -= 1;
                        stats.max_hp = (BASE_HP_TABLE[(stats.level - 1) as usize]
                            * (1.0 + (stats.con / 100.0)))
                            .round();
                        updated_stat = true;
                    }
                }
                _ => {
                    break;
                }
            }

            if updated_stat {
                stats.update_available = true;
                send_message_to_player(
                    &player_details.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    &("Increased ".to_string() + stat + "."),
                    None,
                    sender,
                );
            }
            break;
        }
    }
}
