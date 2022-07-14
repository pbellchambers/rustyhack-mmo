use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::components::EntityType;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) type MapState = Vec<Vec<Vec<EntityType>>>;

pub(crate) type AllMapStates = HashMap<String, MapState>;

pub(crate) fn initialise_all_map_states(all_maps: &AllMaps) -> AllMapStates {
    info!("About to initialise empty map state for all maps");
    let mut all_map_states: AllMapStates = HashMap::new();
    for (map_name, background_map) in all_maps {
        let mut map_state: MapState = vec![vec![vec![]]];
        for row in background_map.data.iter() {
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

pub(crate) fn insert_entity_at(map: &mut MapState, entity: EntityType, x: usize, y: usize) {
    map[y][x].push(entity);
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

pub(crate) fn is_colliding_with_other_player(
    x: usize,
    y: usize,
    map_state: &MapState,
) -> (bool, String) {
    let mut colliding = false;
    let mut player_name = "".to_string();
    for entity_type in map_state[y][x].iter() {
        if let EntityType::Player(player) = entity_type {
            colliding = player.player_details.currently_online && player.display_details.collidable;
            player_name = player.player_details.player_name.clone();
        }
    }
    (colliding, player_name)
}

pub(crate) fn is_colliding_with_monster(x: usize, y: usize, map_state: &MapState) -> (bool, Uuid) {
    let mut colliding = false;
    let mut monster_id = Uuid::new_v4();
    for entity_type in map_state[y][x].iter() {
        if let EntityType::Monster(monster) = entity_type {
            colliding = monster.display_details.collidable;
            monster_id = monster.monster_details.id;
        }
    }
    (colliding, monster_id)
}
