use crate::consts::{BASE_HP_TABLE, CUMULATIVE_EXP_TABLE};
use crate::game::player_updates::send_message_to_player;
use crate::game::players::PlayersPositions;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Entity, IntoQuery, Query, World};
use rand::Rng;
use rustyhack_lib::ecs::components::{
    DisplayDetails, Inventory, ItemDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::item::Item;
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
            stats.current_hp = stats.max_hp;
            stats.update_available = true;
            *position = Position::default();
            position.update_available = true;
            send_message_to_player(
                &player_details.player_name,
                &player_details.client_addr,
                player_details.currently_online,
                "Now respawning at respawn point...",
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
        players_positions.insert(player_details.id, position.clone());
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
                    "You levelled up!",
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
    //todo make stat selection a player choice rather than randomly assigning 2 stat upgrades here
    let mut stat_upgrades = 2;
    while stat_upgrades > 0 {
        let mut rng = rand::thread_rng();
        let random_choice = rng.gen_range(1..=3);
        if random_choice == 1 {
            stats.str += 1.0;
        } else if random_choice == 2 {
            stats.dex += 1.0;
        } else {
            stats.con += 1.0;
        }
        stat_upgrades -= 1;
    }
    stats.max_hp =
        (BASE_HP_TABLE[(stats.level - 1) as usize] * (1.0 + (stats.con / 100.0))).round();
    stats
}

pub(crate) fn pickup_item(world: &mut World, position_message: &PositionMessage) {
    let mut item_id_option: Option<Uuid> = None;
    let mut item_option: Option<Item> = None;
    let mut entity_option: Option<Entity> = None;
    let mut item_query = <(Entity, &ItemDetails, &DisplayDetails, &Position, &Item)>::query();

    //confirm item exists at that position and get details
    for (
        requested_item_entity,
        requested_item_details,
        _requested_item_display_details,
        requested_item_position,
        requested_item,
    ) in item_query.iter(world)
    {
        if position_message.position.pos_x == requested_item_position.pos_x
            && position_message.position.pos_y == requested_item_position.pos_y
            && position_message.position.current_map == requested_item_position.current_map
        {
            item_id_option = Some(requested_item_details.id);
            item_option = Some(requested_item.clone());
            entity_option = Some(*requested_item_entity);
            break;
        }
    }
    //if there was a matching item, then remove it from world
    match entity_option {
        None => {
            debug!("No matching item found.");
        }
        Some(entity) => {
            debug!(
                "Item found, id: {}, removing entity from world: {:?}",
                item_id_option.unwrap(),
                entity
            );
            //remove item from world
            world.remove(entity);
        }
    }

    //add item to player carried inventory
    let mut player_query = <(&PlayerDetails, &mut Inventory)>::query();
    for (player_details, player_inventory) in player_query.iter_mut(world) {
        if player_details.player_name == position_message.player_name {
            match item_option {
                None => {
                    debug!("No matching item found.");
                }
                Some(item) => {
                    debug!(
                        "Item found, added to player {} inventory.",
                        player_details.player_name
                    );
                    player_inventory.carried.push(item);
                    player_inventory.update_available = true;
                }
            }
            break;
        }
    }
}
