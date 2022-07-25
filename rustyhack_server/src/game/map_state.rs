use crate::game::combat::Defender;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::components::{DisplayDetails, EntityType, Position};
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) type MapState = Vec<Vec<Vec<EntityType>>>;
pub(crate) type AllMapStates = HashMap<String, MapState>;
pub type EntityPositionMap = HashMap<Uuid, (Position, DisplayDetails, String)>;

pub(crate) fn initialise_all_map_states(all_maps: &AllMaps) -> AllMapStates {
    info!("About to initialise empty map state for all maps");
    let mut all_map_states: AllMapStates = HashMap::new();
    for (map_name, background_map) in all_maps {
        let mut map_state: MapState = vec![vec![vec![]]];
        for row in &background_map.data {
            let mut row_vec: Vec<Vec<EntityType>> = vec![vec![]];
            for _tile in row.iter() {
                //push an empty map state vector for each tile
                row_vec.push(vec![]);
            }
            map_state.push(row_vec);
        }
        info!("Initialised map state for {} map.", &map_name);
        all_map_states.insert(map_name.clone(), map_state);
    }
    info!("Finished initialising all map states.");
    all_map_states
}

pub(crate) fn insert_entity_at(map: &mut MapState, entity: EntityType, x: u32, y: u32) {
    map[y as usize][x as usize].push(entity);
}

pub(crate) fn clear_all_entities(map_states: &mut AllMapStates) -> &mut AllMapStates {
    for map_state in &mut map_states.values_mut() {
        for map_row in map_state.iter_mut() {
            for map_tile in map_row.iter_mut() {
                map_tile.clear();
            }
        }
    }
    map_states
}

pub(crate) fn is_colliding_with_entity(x: u32, y: u32, map_state: &MapState) -> (bool, Defender) {
    let mut colliding = false;
    let mut defending_entity = Defender::default();
    if y == 0 {
        //don't bother checking for collisions at y == 0 as map_state overflows
        (colliding, defending_entity)
    } else {
        for entity_type in &map_state[y as usize][x as usize] {
            if let EntityType::Player(player) = entity_type {
                colliding =
                    player.player_details.currently_online && player.display_details.collidable;
                defending_entity = Defender {
                    id: player.player_details.id,
                    name: player.player_details.player_name.clone(),
                    client_addr: player.player_details.client_addr.clone(),
                    currently_online: player.player_details.currently_online,
                }
            } else if let EntityType::Monster(monster) = entity_type {
                colliding = monster.display_details.collidable;
                defending_entity = Defender {
                    id: monster.monster_details.id,
                    name: monster.monster_details.monster_type.clone(),
                    client_addr: "".to_string(),
                    currently_online: true,
                }
            }
        }
        (colliding, defending_entity)
    }
}
