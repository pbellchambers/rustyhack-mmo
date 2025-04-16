use crate::consts;
use crate::game::map::spawns::{AllSpawnCounts, AllSpawnsMap, PositionWithoutMap};
use legion::World;
use legion::systems::CommandBuffer;
use rand::Rng;
use rand::prelude::IndexedRandom;
use rustyhack_lib::ecs::components::{DisplayDetails, Inventory, MonsterDetails, Position, Stats};
use rustyhack_lib::ecs::monster::AllMonsterDefinitions;
use std::collections::HashMap;
use std::process;
use uuid::Uuid;

pub(crate) fn count_monsters_needing_respawn(
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
    debug!("Monsters needing respawn are: {monsters_needing_respawn:?}");
    monsters_needing_respawn
}

pub(crate) fn count_alive_monsters(
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
    debug!("Current monsters alive are: {current_monsters_count:?}");
    current_monsters_count
}

pub(crate) fn respawn_monsters(
    monsters_needing_respawn: &AllSpawnCounts,
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns_map: &AllSpawnsMap,
    commands: &mut CommandBuffer,
) {
    for (map, spawns) in monsters_needing_respawn {
        for (monster_type, count) in spawns {
            //only spawn at most 1 of each monster_type per map per tick
            if should_respawn_this_tick() && count > &0 {
                spawn_single_monster(
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
    let mut rng = rand::rng();
    consts::TICK_SPAWN_CHANCE_PERCENTAGE >= rng.random_range(0..=101)
}

pub(crate) fn spawn_initial_monsters(
    world: &mut World,
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns_map: &AllSpawnsMap,
) {
    info!("Spawning initial monsters.");
    let mut monsters_vec: Vec<(MonsterDetails, DisplayDetails, Position, Stats, Inventory)> =
        vec![];
    for (map, spawns) in all_spawns_map {
        for monster in &spawns.monsters {
            let mut current_monster = all_monster_definitions
                .get(&monster.monster_type)
                .unwrap_or_else(|| {
                    error!(
                        "Monster {} missing from all_monster_definitions.",
                        &monster.monster_type,
                    );
                    process::exit(1);
                })
                .clone();
            for spawn_position in &monster.spawn_positions {
                let position = Position {
                    update_available: false,
                    pos_x: spawn_position.x,
                    pos_y: spawn_position.y,
                    current_map: map.clone(),
                    velocity_x: 0,
                    velocity_y: 0,
                };
                current_monster.monster_details.id = Uuid::new_v4();
                current_monster.monster_details.spawn_position = position.clone();
                current_monster.position = position;
                info!(
                    "Spawned monster {} at position {:?}",
                    current_monster.monster_details.monster_type, current_monster.position
                );
                monsters_vec.push((
                    current_monster.monster_details.clone(),
                    current_monster.display_details,
                    current_monster.position,
                    current_monster.stats,
                    current_monster.inventory.clone(),
                ));
            }
        }
    }
    world.extend(monsters_vec);
}

fn spawn_single_monster(
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns_map: &AllSpawnsMap,
    map: &str,
    monster_type: &str,
    commands: &mut CommandBuffer,
) {
    info!("Spawning single monster.");
    let mut current_monster = all_monster_definitions
        .get(monster_type)
        .unwrap_or_else(|| {
            error!("Monster {monster_type} missing from all_monster_definitions.");
            process::exit(1);
        })
        .clone();
    let all_spawn_positions = all_spawns_map.get(map).unwrap();
    let mut random_spawn_position: PositionWithoutMap;
    for monster_spawn_positions in &all_spawn_positions.monsters {
        if monster_spawn_positions.monster_type.eq(monster_type) {
            random_spawn_position = *monster_spawn_positions
                .spawn_positions
                .choose(&mut rand::rng())
                .unwrap();

            let position = Position {
                update_available: false,
                pos_x: random_spawn_position.x,
                pos_y: random_spawn_position.y,
                current_map: map.to_string(),
                velocity_x: 0,
                velocity_y: 0,
            };
            current_monster.monster_details.id = Uuid::new_v4();
            current_monster.monster_details.spawn_position = position.clone();
            current_monster.position = position.clone();
            commands.push((
                current_monster.monster_details.clone(),
                current_monster.display_details,
                current_monster.position,
                current_monster.stats,
                current_monster.inventory.clone(),
            ));
            info!(
                "Spawned {} at position: ({} {})",
                current_monster.monster_details.monster_type, position.pos_x, position.pos_y
            );
        }
    }
}
