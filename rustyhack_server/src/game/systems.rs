use legion::world::SubWorld;
use legion::*;
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{DisplayDetails, PlayerDetails, Position, Velocity};
use std::collections::HashMap;

pub(crate) fn build_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(update_player_input_system())
        .add_system(update_entities_position_system())
        .build();
    info!("Built system schedule.");
    schedule
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
#[write_component(Velocity)]
#[write_component(Position)]
#[read_component(DisplayDetails)]
fn update_entities_position(world: &mut SubWorld, #[resource] all_maps: &AllMaps) {
    let mut query = <(&mut Velocity, &mut Position)>::query();
    let world2 = &world.clone();
    for (velocity, position) in query.iter_mut(world) {
        debug!("Updating world entities positions after velocity updates.");
        let current_map = all_maps.get(&position.map).unwrap_or_else(|| {
            error!(
                "Entity is located on a map that does not exist: {}",
                &position.map
            );
            warn!("Will return the default map, but this may cause problems.");
            all_maps.get(DEFAULT_MAP).unwrap()
        });
        if !entity_is_colliding_with_tile(current_map.get_tile_at(
            (position.x + velocity.x) as usize,
            (position.y + velocity.y) as usize,
        )) && !entity_is_colliding_with_entity(
            position.x + velocity.x,
            position.y + velocity.y,
            world2,
            &position.map,
        ) {
            position.x += velocity.x;
            position.y += velocity.y;
        }
        velocity.x = 0;
        velocity.y = 0;
    }
}

fn entity_is_colliding_with_entity(
    player_x: i32,
    player_y: i32,
    world: &SubWorld,
    current_map: &str,
) -> bool {
    let mut result = false;
    let mut query = <(&Position, &DisplayDetails)>::query();
    for (position, display_details) in query.iter(world) {
        if position.map == current_map
            && display_details.collidable
            && position.x == player_x
            && position.y == player_y
        {
            result = true;
        }
    }
    result
}

fn entity_is_colliding_with_tile(tile: Tile) -> bool {
    match tile {
        Tile::Door(door) => door.collidable == Collidable::True,
        Tile::Wall(wall) => wall.collidable == Collidable::True,
        Tile::Boundary => true,
        _ => false,
    }
}
