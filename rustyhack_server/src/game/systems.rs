use crate::game::combat::{CombatAttackerStats, CombatParties};
use crate::game::map_state::{AllMapStates, MapState};
use crate::game::{combat, map_state};
use legion::world::SubWorld;
use legion::{system, Query, Schedule};
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::{AllMaps, BackgroundMap};
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{
    DisplayDetails, EntityType, MonsterDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::monster::Monster;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::math_utils::{i32_from, u32_from};
use std::collections::HashMap;

pub(crate) fn build_map_state_update_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(reset_map_state_system())
        .add_system(add_entities_to_map_state_system())
        .build();
    info!("Built map state update system schedule.");
    schedule
}

pub(crate) fn build_player_update_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(update_player_input_system())
        .add_system(check_for_combat_system())
        .add_system(resolve_combat_system())
        .add_system(update_entities_position_system())
        .build();
    info!("Built player update system schedule.");
    schedule
}

pub(crate) fn build_monster_update_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(update_entities_position_system())
        .build();
    info!("Built monster update system schedule.");
    schedule
}

#[system]
fn reset_map_state(#[resource] all_map_states: &mut AllMapStates) {
    debug!("Clearing map state.");
    map_state::clear_all_entities(all_map_states);
}

#[system(for_each)]
fn add_entities_to_map_state(
    position: &Position,
    display_details: &DisplayDetails,
    monster_details_option: Option<&MonsterDetails>,
    player_details_option: Option<&PlayerDetails>,
    stats_option: Option<&Stats>,
    #[resource] all_map_states: &mut AllMapStates,
) {
    debug!("Adding current entity positions to map state.");
    if let Some(monster_details) = monster_details_option {
        let monster = Monster {
            monster_details: monster_details.clone(),
            display_details: *display_details,
            position: position.clone(),
            stats: *stats_option.unwrap(),
        };
        map_state::insert_entity_at(
            all_map_states.get_mut(&position.current_map).unwrap(),
            EntityType::Monster(monster),
            position.pos_x,
            position.pos_y,
        );
    }
    if let Some(player_details) = player_details_option {
        let player = Player {
            player_details: player_details.clone(),
            display_details: *display_details,
            position: position.clone(),
            stats: *stats_option.unwrap(),
        };
        map_state::insert_entity_at(
            all_map_states.get_mut(&position.current_map).unwrap(),
            EntityType::Player(player),
            position.pos_x,
            position.pos_y,
        );
    }
}

#[system(par_for_each)]
fn update_player_input(
    player_details: &PlayerDetails,
    position: &mut Position,
    #[resource] player_updates: &HashMap<String, Position>,
) {
    debug!("Adding player velocity updates to world.");
    for (update_entity_name, update_position) in player_updates {
        if update_entity_name == &player_details.player_name {
            position.velocity_x = update_position.velocity_x;
            position.velocity_y = update_position.velocity_y;
        }
    }
}

#[system]
fn check_for_combat(
    world: &mut SubWorld,
    query: &mut Query<(
        &mut Position,
        Option<&MonsterDetails>,
        Option<&PlayerDetails>,
        &Stats,
    )>,
    #[resource] all_map_states: &AllMapStates,
    #[resource] combat_parties: &mut CombatParties,
    #[resource] combat_attacker_stats: &mut CombatAttackerStats,
) {
    for (position, monster_details_option, player_details_option, stats) in query.iter_mut(world) {
        debug!("Checking for possible combat after velocity updates.");
        if position.velocity_x == 0 && position.velocity_y == 0 {
            //no velocity, no updates
            continue;
        }
        let current_map_states = get_current_map_states(all_map_states, &position.current_map);
        let potential_pos_x = u32_from(i32_from(position.pos_x) + position.velocity_x);
        let potential_pos_y = u32_from(i32_from(position.pos_y) + position.velocity_y);

        let player_collision_status = map_state::is_colliding_with_other_player(
            potential_pos_x,
            potential_pos_y,
            current_map_states,
        );

        let monster_collision_status = map_state::is_colliding_with_monster(
            potential_pos_x,
            potential_pos_y,
            current_map_states,
        );

        let attacker_name_or_id =
            get_attacker_name_or_id(player_details_option, monster_details_option);
        debug!("Combat detected, attacker is: {}", attacker_name_or_id);

        if player_collision_status.0 {
            combat_parties.insert(player_collision_status.1, attacker_name_or_id.clone());
            combat_attacker_stats.insert(attacker_name_or_id.clone(), *stats);
            position.velocity_x = 0;
            position.velocity_y = 0;
        } else if monster_collision_status.0 {
            combat_parties.insert(monster_collision_status.1, attacker_name_or_id.clone());
            combat_attacker_stats.insert(attacker_name_or_id.clone(), *stats);
            position.velocity_x = 0;
            position.velocity_y = 0;
        }
    }
}

fn get_attacker_name_or_id(
    player_details_option: Option<&PlayerDetails>,
    monster_details_option: Option<&MonsterDetails>,
) -> String {
    if let Some(player_details) = player_details_option {
        player_details.player_name.clone()
    } else if let Some(monster_details) = monster_details_option {
        monster_details.id.to_string()
    } else {
        error!("Attacker was somehow not a player or monster, returning empty string.");
        "".to_string()
    }
}

#[system]
fn resolve_combat(
    world: &mut SubWorld,
    query: &mut Query<(&mut Stats, Option<&MonsterDetails>, Option<&PlayerDetails>)>,
    #[resource] combat_parties: &mut CombatParties,
    #[resource] combat_attacker_stats: &mut CombatAttackerStats,
) {
    for (stats, monster_details_option, player_details_option) in query.iter_mut(world) {
        debug!("Resolving combat.");
        if let Some(player_details) = player_details_option {
            if combat_parties.contains_key(&player_details.player_name) {
                let attacker_name_or_id = combat_parties.get(&player_details.player_name).unwrap();
                let damage = combat::resolve_combat(
                    combat_attacker_stats.get(attacker_name_or_id).unwrap(),
                    stats,
                );
                stats.current_hp -= damage;
                stats.update_available = true;
            }
        } else if let Some(monster_details) = monster_details_option {
            if combat_parties.contains_key(&monster_details.id.to_string()) {
                let attacker_name_or_id =
                    combat_parties.get(&monster_details.id.to_string()).unwrap();
                let damage = combat::resolve_combat(
                    combat_attacker_stats.get(attacker_name_or_id).unwrap(),
                    stats,
                );
                stats.current_hp -= damage;
            }
        }
    }
    combat_parties.clear();
    combat_attacker_stats.clear();
}

fn get_current_map_states<'a>(all_map_states: &'a AllMapStates, map: &String) -> &'a MapState {
    all_map_states.get(map).unwrap_or_else(|| {
        error!("Entity is located on a map that does not exist: {}", &map);
        warn!("Will return the default map, but this may cause problems.");
        all_map_states.get(DEFAULT_MAP).unwrap()
    })
}

#[system]
fn update_entities_position(
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
            position.pos_x = potential_pos_x;
            position.pos_y = potential_pos_y;
        }
        position.velocity_x = 0;
        position.velocity_y = 0;
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
