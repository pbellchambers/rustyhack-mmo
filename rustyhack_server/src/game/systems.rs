use crate::game::map_state::{AllMapStates, MapState};
use crate::game::{combat, map_state};
use legion::world::SubWorld;
use legion::*;
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::{AllMaps, BackgroundMap};
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{
    DisplayDetails, EntityType, MonsterDetails, PlayerDetails, Position, Stats, Velocity,
};
use rustyhack_lib::ecs::monster::Monster;
use rustyhack_lib::ecs::player::Player;
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
    velocity: &Velocity,
    display_detals: &DisplayDetails,
    monster_details_option: Option<&MonsterDetails>,
    player_details_option: Option<&PlayerDetails>,
    stats_option: Option<&Stats>,
    #[resource] all_map_states: &mut AllMapStates,
) {
    debug!("Adding current entity positions to map state.");
    if let Some(monster_details) = monster_details_option {
        let monster = Monster {
            monster_details: monster_details.clone(),
            display_details: *display_detals,
            position: position.clone(),
            velocity: *velocity,
            stats: *stats_option.unwrap(),
        };
        map_state::insert_entity_at(
            all_map_states.get_mut(&position.map).unwrap(),
            EntityType::Monster(monster),
            position.x as usize,
            position.y as usize,
        );
    }
    if let Some(player_details) = player_details_option {
        let player = Player {
            player_details: player_details.clone(),
            display_details: *display_detals,
            position: position.clone(),
            stats: *stats_option.unwrap(),
        };
        map_state::insert_entity_at(
            all_map_states.get_mut(&position.map).unwrap(),
            EntityType::Player(player),
            position.x as usize,
            position.y as usize,
        );
    }
}

#[system(par_for_each)]
fn update_player_input(
    player_details: &PlayerDetails,
    velocity: &mut Velocity,
    #[resource] player_updates: &HashMap<String, Velocity>,
) {
    debug!("Adding player velocity updates to world.");
    for (update_entity_name, update_velocity) in player_updates {
        if update_entity_name == &player_details.player_name {
            velocity.x = update_velocity.x;
            velocity.y = update_velocity.y;
        }
    }
}

#[system]
#[read_component(DisplayDetails)]
fn resolve_combat(
    world: &mut SubWorld,
    velocity_query: &mut Query<(&mut Velocity, &mut Position)>,
    #[resource] all_map_states: &AllMapStates,
) {
    for (velocity, position) in velocity_query.iter_mut(world) {
        debug!("Updating world entities positions after velocity updates.");
        if velocity.x == 0 && velocity.y == 0 {
            //no velocity, no updates
            continue;
        }
        let current_map_states = get_current_map_states(all_map_states, &position.map);

        let player_collision_status = map_state::is_colliding_with_other_player(
            (position.x + velocity.x) as usize,
            (position.y + velocity.y) as usize,
            current_map_states,
        );

        let monster_collision_status = map_state::is_colliding_with_monster(
            (position.x + velocity.x) as usize,
            (position.y + velocity.y) as usize,
            current_map_states,
        );

        if player_collision_status.0 {
            combat::resolve_player_combat(
                player_collision_status.1,
                position.x,
                position.y,
                velocity.x,
                velocity.y,
            );
            velocity.x = 0;
            velocity.y = 0;
        } else if monster_collision_status.0 {
            combat::resolve_monster_combat(
                monster_collision_status.1,
                position.x,
                position.y,
                velocity.x,
                velocity.y,
            );
            velocity.x = 0;
            velocity.y = 0;
        }
    }
}

fn get_current_map_states<'a>(all_map_states: &'a AllMapStates, map: &String) -> &'a MapState {
    all_map_states.get(map).unwrap_or_else(|| {
        error!("Entity is located on a map that does not exist: {}", &map);
        warn!("Will return the default map, but this may cause problems.");
        all_map_states.get(DEFAULT_MAP).unwrap()
    })
}

#[system]
#[read_component(DisplayDetails)]
fn update_entities_position(
    world: &mut SubWorld,
    velocity_query: &mut Query<(&mut Velocity, &mut Position)>,
    #[resource] all_maps: &AllMaps,
) {
    for (velocity, position) in velocity_query.iter_mut(world) {
        debug!("Updating world entities positions after velocity updates.");
        if velocity.x == 0 && velocity.y == 0 {
            //no velocity, no updates
            continue;
        }
        let current_map = get_current_map(all_maps, &position.map);

        if !entity_is_colliding_with_tile(current_map.get_tile_at(
            (position.x + velocity.x) as usize,
            (position.y + velocity.y) as usize,
        )) {
            position.x += velocity.x;
            position.y += velocity.y;
        }
        velocity.x = 0;
        velocity.y = 0;
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
