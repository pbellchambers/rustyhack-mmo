use crate::consts;
use crate::consts::MONSTER_DISTANCE_ACTIVATION;
use crate::game::map_state::EntityPositionMap;
use crate::game::monsters;
use crate::game::players::PlayersPositions;
use crate::game::spawns::{AllSpawnCounts, AllSpawnsMap};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, Query};
use rand::Rng;
use rustyhack_lib::consts::{DEAD_MAP, DEFAULT_ITEM_COLOUR, DEFAULT_ITEM_ICON};
use rustyhack_lib::ecs::components::{
    Dead, DisplayDetails, Inventory, ItemDetails, MonsterDetails, Position, Stats,
};
use rustyhack_lib::ecs::item::Item;
use rustyhack_lib::ecs::monster::AllMonsterDefinitions;
use rustyhack_lib::math_utils::i32_from;
use std::collections::HashMap;
use uuid::Uuid;

#[system]
pub(crate) fn resolve_monster_deaths(
    world: &mut SubWorld,
    query: &mut Query<(Entity, &MonsterDetails, &Stats, &Position, &Inventory)>,
    commands: &mut CommandBuffer,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    debug!("Removing dead monsters.");
    for (entity, monster, stats, position, inventory) in query.iter(world) {
        if stats.current_hp <= 0.0 {
            debug!(
                "Monster {} {:?} {} died.",
                monster.id, entity, monster.monster_type
            );
            //drop inventory items
            let mut items_vec: Vec<(ItemDetails, DisplayDetails, Position, Item)> = vec![];
            debug!("Monster inventory was: {:?}", inventory);
            for item in &inventory.carried {
                items_vec.push((
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
                        prev_pos_x: position.pos_x,
                        prev_pos_y: position.pos_y,
                        prev_map: position.current_map.clone(),
                        velocity_x: 0,
                        velocity_y: 0,
                    },
                    item.clone(),
                ));
            }
            //add dropped item entities to world
            debug!("Items being added to world are: {:?}", items_vec);
            commands.extend(items_vec);
            let dead_position = Position {
                current_map: position.current_map.clone() + DEAD_MAP,
                ..Dead::dead()
            };
            entity_position_map.insert(
                monster.id,
                (
                    dead_position,
                    DisplayDetails::dead(),
                    "dead_monster".to_string(),
                ),
            );
            //remove monster from world
            commands.remove(*entity);
        }
    }
}

#[system]
pub(crate) fn update_monster_velocities(
    world: &mut SubWorld,
    query: &mut Query<(&mut MonsterDetails, &mut Position)>,
    #[resource] players_positions: &PlayersPositions,
) {
    debug!("Updating monster velocities - checking for movement to player positions");
    for (monster, position) in query.iter_mut(world) {
        let mut moving_towards_existing_target = false;

        if let Some(target) = monster.current_target {
            if let Some(current_target_position) = players_positions.get(&target) {
                if is_specific_player_nearby(current_target_position, position) {
                    move_towards_target(position, current_target_position);
                    moving_towards_existing_target = true;
                }
            }
        }

        if !moving_towards_existing_target {
            let nearby_player = is_any_player_nearby(players_positions, position);
            match nearby_player {
                Some((player_id, player_position)) => {
                    monster.is_active = true;
                    monster.current_target = Some(*player_id);
                    move_towards_target(position, player_position);
                }
                None => {
                    debug!("Monster returning to spawn location");
                    monster.is_active = false;
                    monster.current_target = None;
                    move_towards_target(position, &monster.spawn_position);
                }
            }
        }
    }
}

fn move_towards_target(monster_position: &mut Position, target_position: &Position) {
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let diff_x: i32 = monster_position_x - i32_from(target_position.pos_x);
    let diff_y: i32 = monster_position_y - i32_from(target_position.pos_y);
    let mut new_pos_x = monster_position_x;
    let mut new_pos_y = monster_position_y;

    if (diff_x.abs() >= 1 && diff_y.abs() >= 1) || (diff_x == 0 && diff_y == 0) {
        //far away, move randomly towards
        let mut rng = rand::thread_rng();
        if rng.gen::<bool>() {
            new_pos_x = move_towards(diff_x, monster_position_x);
        } else {
            new_pos_y = move_towards(diff_y, monster_position_y);
        }
    } else if diff_x.abs() > 1 && diff_y.abs() == 0 {
        //in line, should mostly move towards, but sometimes randomly
        let mut rng = rand::thread_rng();
        if rng.gen_range(1..=6) > 1 {
            new_pos_x = move_towards(diff_x, monster_position_x);
        } else if rng.gen::<bool>() {
            new_pos_y = move_towards(diff_y + 1, monster_position_y);
        } else {
            new_pos_y = move_towards(diff_y - 1, monster_position_y);
        }
    } else if diff_x.abs() == 0 && diff_y.abs() > 1 {
        //in line, should mostly move towards, but sometimes randomly
        let mut rng = rand::thread_rng();
        if rng.gen_range(1..=6) > 1 {
            new_pos_y = move_towards(diff_y, monster_position_y);
        } else if rng.gen::<bool>() {
            new_pos_x = move_towards(diff_x + 1, monster_position_x);
        } else {
            new_pos_x = move_towards(diff_x - 1, monster_position_x);
        }
    } else if diff_x.abs() == 1 && diff_y.abs() == 0 {
        //should be an attack
        new_pos_x = move_towards(diff_x, monster_position_x);
    } else if diff_x.abs() == 0 && diff_y.abs() == 1 {
        //should be an attack
        new_pos_y = move_towards(diff_y, monster_position_y);
    }

    monster_position.velocity_x = new_pos_x - monster_position_x;
    monster_position.velocity_y = new_pos_y - monster_position_y;
}

fn move_towards(diff: i32, position: i32) -> i32 {
    if diff.is_positive() {
        position - 1
    } else {
        position + 1
    }
}

#[allow(clippy::similar_names)]
fn is_any_player_nearby<'a>(
    player_positions: &'a PlayersPositions,
    monster_position: &Position,
) -> Option<(&'a Uuid, &'a Position)> {
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let monster_x_range = (monster_position_x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position_y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_y + MONSTER_DISTANCE_ACTIVATION);
    for (player_id, position) in player_positions {
        if monster_position.current_map == position.current_map
            && monster_x_range.contains(&(i32_from(position.pos_x)))
            && monster_y_range.contains(&(i32_from(position.pos_y)))
        {
            debug!("There is a player near a monster");
            return Some((player_id, position));
        }
    }
    None
}

#[allow(clippy::similar_names)]
fn is_specific_player_nearby(
    current_target_position: &Position,
    monster_position: &Position,
) -> bool {
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let monster_x_range = (monster_position_x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position_y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_y + MONSTER_DISTANCE_ACTIVATION);

    if monster_x_range.contains(&(i32_from(current_target_position.pos_x)))
        && monster_y_range.contains(&(i32_from(current_target_position.pos_y)))
        && monster_position.current_map == current_target_position.current_map
    {
        return true;
    }
    false
}

#[system]
pub(crate) fn spawn_monsters(
    world: &mut SubWorld,
    query: &mut Query<(&mut MonsterDetails, &mut Position)>,
    commands: &mut CommandBuffer,
    #[resource] all_spawns_map: &AllSpawnsMap,
    #[resource] default_spawn_counts: &AllSpawnCounts,
    #[resource] all_monster_definitions: &AllMonsterDefinitions,
) {
    debug!("Checking whether replacement monsters need to spawn.");
    let mut current_monsters_count: AllSpawnCounts = HashMap::new();
    for (monster, position) in query.iter_mut(world) {
        current_monsters_count = count_alive_monsters(current_monsters_count, monster, position);
    }
    let monsters_needing_respawn: AllSpawnCounts =
        count_monsters_needing_respawn(&current_monsters_count, default_spawn_counts);
    respawn_monsters(
        &monsters_needing_respawn,
        all_monster_definitions,
        all_spawns_map,
        commands,
    );
}

fn respawn_monsters(
    monsters_needing_respawn: &AllSpawnCounts,
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns_map: &AllSpawnsMap,
    commands: &mut CommandBuffer,
) {
    for (map, spawns) in monsters_needing_respawn {
        for (monster_type, count) in spawns {
            //only spawn at most 1 of each monster_type per map per tick
            if should_respawn_this_tick() && count > &0 {
                monsters::spawn_single_monster(
                    all_monster_definitions,
                    all_spawns_map,
                    map,
                    monster_type,
                    commands,
                );
            }
        }
    }
}

fn should_respawn_this_tick() -> bool {
    //random chance for respawning each chick
    let mut rng = rand::thread_rng();
    consts::TICK_SPAWN_CHANCE_PERCENTAGE >= rng.gen_range(0..=101)
}

fn count_monsters_needing_respawn(
    current_monsters_count: &AllSpawnCounts,
    default_spawn_counts: &AllSpawnCounts,
) -> AllSpawnCounts {
    //get diff between default spawns and current alive monsters
    let mut monsters_needing_respawn: AllSpawnCounts = HashMap::new();
    for (map, spawns) in default_spawn_counts {
        let mut map_monsters_needing_respawn: HashMap<String, u32> = HashMap::new();
        for (monster, count) in spawns {
            let needing_respawn_count = count
                - current_monsters_count
                    .get(map)
                    .unwrap_or(&HashMap::new())
                    .get(monster)
                    .unwrap_or(&0);
            map_monsters_needing_respawn.insert(monster.clone(), needing_respawn_count);
        }
        monsters_needing_respawn.insert(map.clone(), map_monsters_needing_respawn);
    }
    debug!(
        "Monsters needing respawn are: {:?}",
        monsters_needing_respawn
    );
    monsters_needing_respawn
}

fn count_alive_monsters(
    mut current_monsters_count: AllSpawnCounts,
    monster: &MonsterDetails,
    position: &Position,
) -> HashMap<String, HashMap<String, u32>> {
    if current_monsters_count.contains_key(&position.current_map) {
        let mut map_monster_count: HashMap<String, u32> = current_monsters_count
            .get(&position.current_map)
            .unwrap()
            .clone();
        if map_monster_count.contains_key(&monster.monster_type) {
            let mut count: u32 = *map_monster_count.get(&monster.monster_type).unwrap();
            count += 1;
            map_monster_count.insert(monster.monster_type.clone(), count);
            current_monsters_count.insert(position.current_map.clone(), map_monster_count.clone());
        } else {
            map_monster_count.insert(monster.monster_type.clone(), 1);
            current_monsters_count.insert(position.current_map.clone(), map_monster_count.clone());
        }
    } else {
        let mut monster_count: HashMap<String, u32> = HashMap::new();
        monster_count.insert(monster.monster_type.clone(), 1);
        current_monsters_count.insert(position.current_map.clone(), monster_count);
    }
    debug!("Current monsters alive are: {:?}", current_monsters_count);
    current_monsters_count
}
