use crate::consts::{
    BASE_HEALTH_REGEN_PERCENT, HEALTH_REGEN_CON_PERCENT, HEALTH_REGEN_CON_STATIC_FACTOR,
};
use crate::game::map_state::EntityPositionMap;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{maybe_changed, system, Entity, Query};
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::{AllMaps, BackgroundMap};
use rustyhack_lib::consts::{DEAD_MAP, DEFAULT_MAP};
use rustyhack_lib::ecs::components::{
    Dead, DisplayDetails, ItemDetails, MonsterDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::item::{get_item_name, Item};
use rustyhack_lib::math_utils::{i32_from, u32_from};

#[system]
pub(crate) fn check_for_tile_collision(
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

        if entity_is_colliding_with_tile(current_map.get_tile_at(potential_pos_x, potential_pos_y))
        {
            debug!("Entity colliding with tile, setting velocity to 0.");
            position.velocity_x = 0;
            position.velocity_y = 0;
        } else {
            debug!("Entity not colliding with tile, continuing to combat check.");
            continue;
        }
    }
}

#[system]
pub(crate) fn update_entities_position(world: &mut SubWorld, query: &mut Query<&mut Position>) {
    for position in query.iter_mut(world) {
        debug!("Updating all final positions.");
        if position.velocity_x == 0 && position.velocity_y == 0 {
            //no velocity, no updates
            continue;
        }
        position.pos_x = u32_from(i32_from(position.pos_x) + position.velocity_x);
        position.pos_y = u32_from(i32_from(position.pos_y) + position.velocity_y);
        position.velocity_x = 0;
        position.velocity_y = 0;
        position.update_available = true;
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
        //only apply health regen if out of combat
        if !stats.in_combat {
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
