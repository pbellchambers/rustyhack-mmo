use crate::game::combat::Defender;
use ndarray::Array2;
use rayon::prelude::*;
use rustyhack_lib::background_map::{AllMaps, BackgroundMap};
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{DisplayDetails, EntityType, Position};
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) type MapState = Array2<Vec<EntityType>>;
pub(crate) type AllMapStates = HashMap<String, MapState>;
pub(crate) type EntityPositionMap = HashMap<Uuid, (Position, DisplayDetails, String)>;

pub(crate) fn initialise_all_map_states(all_maps: &AllMaps) -> AllMapStates {
    info!("About to initialise empty map state for all maps");
    let mut all_map_states: AllMapStates = HashMap::new();
    for (map_name, background_map) in all_maps {
        let map_state = Array2::from_elem(
            (background_map.data.nrows(), background_map.data.ncols()),
            vec![],
        );
        info!(
            "Initialised map state for {} map, total rows {}, total cols {}",
            &map_name,
            &map_state.nrows(),
            &map_state.ncols()
        );
        all_map_states.insert(map_name.clone(), map_state);
    }
    info!("Finished initialising all map states.");
    all_map_states
}

pub(crate) fn insert_entity_at(map: &mut MapState, entity: EntityType, x: u32, y: u32) {
    match map.get_mut((y as usize, x as usize)) {
        None => {
            warn!(
                "Tried to insert entity at invalid position, x: {x}, y: {y}, will try to continue."
            );
        }
        Some(entity_vec) => {
            entity_vec.push(entity);
        }
    }
}

pub(crate) fn remove_entity_at(map: &mut MapState, entity: &EntityType, x: u32, y: u32) {
    let entity_index = map
        .get((y as usize, x as usize))
        .expect("Tried to remove entity at invalid position")
        .par_iter()
        .enumerate()
        .find_any(|(_index, map_state_entity)| *map_state_entity == entity);
    match entity_index {
        None => {
            //do nothing
        }
        Some((index, _entity_type)) => match map.get_mut((y as usize, x as usize)) {
            None => {
                warn!(
                    "Tried to remove entity at invalid position, x: {x}, y: {y}, will try to continue."
                );
            }
            Some(entity_vec) => {
                entity_vec.remove(index);
            }
        },
    }
}

pub(crate) fn clear_all_entities(map_states: &mut AllMapStates) -> &mut AllMapStates {
    map_states
        .par_iter_mut()
        .for_each(|(_map_name, map_state)| {
            map_state.par_map_inplace(Vec::clear);
        });
    map_states
}

pub(crate) fn is_colliding_with_entity(x: u32, y: u32, map_state: &MapState) -> (bool, Defender) {
    let mut colliding = false;
    let mut defender = Defender::default();
    if y == 0 {
        //don't bother checking for collisions at y == 0 as map_state overflows
        (colliding, defender)
    } else {
        let defending_entity = match &map_state.get((y as usize, x as usize)) {
            None => {
                warn!(
                    "Tried to check for entity collision at invalid position, x: {x}, y: {y}, will try to continue."
                );
                None
            }
            Some(entity_vec) => entity_vec.par_iter().find_any(|entity_type| {
                if let EntityType::Player(player) = entity_type {
                    player.player_details.currently_online && player.display_details.collidable
                } else if let EntityType::Monster(monster) = entity_type {
                    monster.display_details.collidable
                } else {
                    false
                }
            }),
        };

        match defending_entity {
            None => {}
            Some(entity_type) => {
                if let EntityType::Player(player) = entity_type {
                    colliding = true;
                    defender = Defender {
                        id: player.player_details.id,
                        name: player.player_details.player_name.clone(),
                        client_addr: player.player_details.client_addr.clone(),
                        currently_online: player.player_details.currently_online,
                        is_player: true,
                    }
                } else if let EntityType::Monster(monster) = entity_type {
                    colliding = true;
                    defender = Defender {
                        id: monster.monster_details.id,
                        name: monster.monster_details.monster_type.clone(),
                        client_addr: String::new(),
                        currently_online: true,
                        is_player: false,
                    };
                }
            }
        }
        (colliding, defender)
    }
}

pub(crate) fn get_current_map<'a>(all_maps: &'a AllMaps, map: &str) -> &'a BackgroundMap {
    all_maps.get(map).unwrap_or_else(|| {
        error!("Entity is located on a map that does not exist: {}", &map);
        warn!("Will return the default map, but this may cause problems.");
        all_maps.get(DEFAULT_MAP).unwrap()
    })
}

pub(crate) fn get_current_map_states<'a>(
    all_map_states: &'a mut AllMapStates,
    map: &str,
) -> &'a mut MapState {
    if all_map_states.contains_key(map) {
        all_map_states.get_mut(map).unwrap()
    } else {
        warn!("Tried to get map state for map that doesn't exist.");
        warn!("Will return default map, but things might break.");
        all_map_states.get_mut(DEFAULT_MAP).unwrap()
    }
}
