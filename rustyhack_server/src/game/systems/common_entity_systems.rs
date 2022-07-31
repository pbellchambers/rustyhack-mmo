use crate::consts::{
    BASE_HEALTH_REGEN_PERCENT, HEALTH_REGEN_CON_PERCENT, HEALTH_REGEN_CON_STATIC_FACTOR,
};
use crate::game::map_state;
use crate::game::map_state::{AllMapStates, EntityPositionMap};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{maybe_changed, system, Entity, Query};
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::{AllMaps, BackgroundMap};
use rustyhack_lib::consts::{
    DEAD_MAP, DEFAULT_MAP, DEFAULT_PLAYER_POSITION_X, DEFAULT_PLAYER_POSITION_Y,
};
use rustyhack_lib::ecs::components::{
    Dead, DisplayDetails, ItemDetails, MonsterDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::item::{get_item_name, Item};
use rustyhack_lib::math_utils::{i32_from, u32_from};

#[system]
pub(crate) fn update_entities_position(
    world: &mut SubWorld,
    query: &mut Query<&mut Position>,
    #[resource] all_maps: &AllMaps,
) {
    for position in query.iter_mut(world) {
        debug!("Checking for possible movement after velocity updates and combat check.");
        if position.velocity_x == 0 && position.velocity_y == 0 {
            //no velocity, no updates
            continue;
        }
        let current_map = get_current_map(all_maps, &position.current_map);
        let potential_pos_x = u32_from(i32_from(position.pos_x) + position.velocity_x);
        let potential_pos_y = u32_from(i32_from(position.pos_y) + position.velocity_y);

        if !entity_is_colliding_with_tile(current_map.get_tile_at(potential_pos_x, potential_pos_y))
        {
            position.prev_pos_x = position.pos_x;
            position.prev_pos_y = position.pos_y;
            position.prev_map = position.current_map.clone();
            position.pos_x = potential_pos_x;
            position.pos_y = potential_pos_y;
            position.update_available = true;
        }
        position.velocity_x = 0;
        position.velocity_y = 0;
    }
}

#[system]
pub(crate) fn resolve_conflicting_positions(
    world: &mut SubWorld,
    query: &mut Query<(
        &mut Position,
        Option<&PlayerDetails>,
        Option<&MonsterDetails>,
    )>,
    #[resource] all_map_states: &mut AllMapStates,
) {
    for (position, player_details_option, monster_details_option) in query.iter_mut(world) {
        debug!("Checking for possible conflicting positions where entities have moved into same location.");
        let map_state = all_map_states.get(&position.current_map).unwrap();
        if let Some(player_details) = player_details_option {
            if position.pos_x != DEFAULT_PLAYER_POSITION_X
                && position.pos_y != DEFAULT_PLAYER_POSITION_Y
                && map_state::contains_multiple_collidable_entities(
                    position.pos_x,
                    position.pos_y,
                    map_state,
                )
            {
                warn!(
                    "Conflicting position detected, moving player {} back to previous position.",
                    player_details.player_name
                );
                position.pos_x = position.prev_pos_x;
                position.pos_y = position.prev_pos_y;
                position.current_map = position.prev_map.clone();
            }
        } else if let Some(monster_details) = monster_details_option {
            if map_state::contains_multiple_collidable_entities(
                position.pos_x,
                position.pos_y,
                map_state,
            ) {
                warn!(
                    "Conflicting position detected, moving monster {} back to previous position.",
                    monster_details.monster_type
                );
                position.pos_x = position.prev_pos_x;
                position.pos_y = position.prev_pos_y;
                position.current_map = position.prev_map.clone();
            }
        }
    }
}

fn get_current_map<'a>(all_maps: &'a AllMaps, map: &String) -> &'a BackgroundMap {
    all_maps.get(map).unwrap_or_else(|| {
        error!("Entity is located on a map that does not exist: {}", &map);
        warn!("Will return the default map, but this may cause problems.");
        all_maps.get(DEFAULT_MAP).unwrap()
    })
}

fn entity_is_colliding_with_tile(tile: Tile) -> bool {
    match tile {
        Tile::Door(door) => door.collidable == Collidable::True,
        Tile::Wall(wall) => wall.collidable == Collidable::True,
        Tile::Boundary => true,
        _ => false,
    }
}

#[system]
pub(crate) fn apply_health_regen(world: &mut SubWorld, query: &mut Query<&mut Stats>) {
    for stats in query.iter_mut(world) {
        debug!("Applying health to all injured but still alive entities.");
        if stats.current_hp > 0.0 && stats.current_hp < stats.max_hp {
            let regen_amount = calculate_regen_amount(stats.max_hp, stats.con);
            debug!(
                "Current hp: {}/{}, regen amount is: {}, update_available is {}",
                stats.current_hp,
                stats.max_hp,
                regen_amount.round(),
                stats.update_available
            );
            stats.current_hp += regen_amount.round();
            //don't heal more than max hp
            if stats.current_hp > stats.max_hp {
                stats.current_hp = stats.max_hp;
            }
            stats.update_available = true;
        }
    }
}

fn calculate_regen_amount(max_hp: f32, con: f32) -> f32 {
    // Current regen calculation is as follows, this is just a first pass, it may not make sense.
    // current hp
    // + (max hp * BASE_HEALTH_REGEN_PERCENT)
    // + (con * HEALTH_REGEN_CON_PERCENT)
    // + (con / HEALTH_REGEN_CON_STATIC_FACTOR)
    (max_hp * (BASE_HEALTH_REGEN_PERCENT / 100.0))
        + (con * (HEALTH_REGEN_CON_PERCENT / 100.0))
        + (con / HEALTH_REGEN_CON_STATIC_FACTOR)
}

#[system(for_each)]
#[filter(maybe_changed::<Position>())]
pub(crate) fn collate_all_player_positions(
    player_details: &PlayerDetails,
    position: &Position,
    display_details: &DisplayDetails,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    debug!("Getting all players positions");
    if player_details.currently_online {
        entity_position_map.insert(
            player_details.id,
            (
                position.clone(),
                *display_details,
                player_details.player_name.clone(),
            ),
        );
    }
}

#[system(for_each)]
#[filter(maybe_changed::<Position>())]
pub(crate) fn collate_all_monster_positions(
    monster_details: &MonsterDetails,
    position: &Position,
    display_details: &DisplayDetails,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    debug!("Getting all monster positions");
    entity_position_map.insert(
        monster_details.id,
        (
            position.clone(),
            *display_details,
            monster_details.monster_type.clone(),
        ),
    );
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[system(for_each)]
#[filter(maybe_changed::<Position>())]
pub(crate) fn collate_all_item_positions(
    entity: &Entity,
    item_details: &ItemDetails,
    item: &Item,
    position: &Position,
    display_details: &DisplayDetails,
    commands: &mut CommandBuffer,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    debug!("Getting all item positions");
    if item_details.has_been_picked_up {
        let dead_map = position.current_map.clone() + DEAD_MAP;
        let dead_position = Position {
            current_map: dead_map,
            ..Dead::dead()
        };
        debug!(
            "Removing item id {} to dead map: {:?}",
            item_details.id, dead_position
        );
        entity_position_map.insert(
            item_details.id,
            (
                dead_position,
                DisplayDetails::dead(),
                "picked_up_item".to_string(),
            ),
        );
        debug!("Removing item id {} from world.", item_details.id);
        commands.remove(*entity);
    } else {
        let item_name: String = get_item_name(item);
        entity_position_map.insert(
            item_details.id,
            (position.clone(), *display_details, item_name),
        );
    }
}
