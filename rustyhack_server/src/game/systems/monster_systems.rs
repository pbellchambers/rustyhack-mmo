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
use rustyhack_lib::utils::math::i32_from;
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

        //get nearby players and whether monster is currently outside its spawn range
        let nearby_players = get_all_players_nearby(players_positions, position);
        let outside_spawn_range = check_if_outside_spawn_range(&monster.spawn_position, position);

        //check if current target within range and move towards it
        if let Some(target) = monster.current_target {
            if nearby_players.contains_key(&target) {
                debug!("Monster moving towards existing target.");
                move_towards_target(position, nearby_players.get(&target).unwrap());
                moving_towards_existing_target = true;
            }
        }

        //else either return to spawn, pick a new target, or move randomly
        if outside_spawn_range && !moving_towards_existing_target {
            debug!("Monster returning to spawn location.");
            monster.current_target = None;
            move_towards_target(position, &monster.spawn_position);
        } else if !outside_spawn_range
            && !moving_towards_existing_target
            && !nearby_players.is_empty()
        {
            debug!("Monster moving towards new target.");
            let nearest_target = get_nearest_target(&nearby_players, position);
            monster.current_target = Some(nearest_target);
            move_towards_target(position, nearby_players.get(&nearest_target).unwrap());
        } else if !outside_spawn_range
            && !moving_towards_existing_target
            && nearby_players.is_empty()
        {
            debug!("Monster moving randomly.");
            move_randomly(position);
        }
    }
}

fn get_nearest_target(
    nearby_players: &HashMap<Uuid, Position>,
    monster_position: &Position,
) -> Uuid {
    let mut closest_distance: u32 = u32::MAX;
    let mut closest_uuid = Uuid::new_v4();
    for (uuid, target_position) in nearby_players {
        let diff_x: u32 =
            (i32_from(monster_position.pos_x) - i32_from(target_position.pos_x)).unsigned_abs();
        let diff_y: u32 =
            (i32_from(monster_position.pos_y) - i32_from(target_position.pos_y)).unsigned_abs();
        if diff_x < closest_distance {
            closest_distance = diff_x;
            closest_uuid = *uuid;
        }
        if diff_y < closest_distance {
            closest_distance = diff_y;
            closest_uuid = *uuid;
        }
    }
    closest_uuid
}

fn check_if_outside_spawn_range(spawn_position: &Position, current_position: &Position) -> bool {
    let diff_x: i32 = i32_from(current_position.pos_x) - i32_from(spawn_position.pos_x);
    let diff_y: i32 = i32_from(current_position.pos_y) - i32_from(spawn_position.pos_y);

    diff_x.abs() > MONSTER_DISTANCE_ACTIVATION || diff_y.abs() > MONSTER_DISTANCE_ACTIVATION
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

fn move_randomly(monster_position: &mut Position) {
    let mut velocity_x = 0;
    let mut velocity_y = 0;
    let mut rng = rand::thread_rng();
    let random_pick: u8 = rng.gen_range(1..=4);

    if random_pick == 1 {
        velocity_x = 1;
    } else if random_pick == 2 {
        velocity_x = -1;
    } else if random_pick == 3 {
        velocity_y = 1;
    } else if random_pick == 4 {
        velocity_y = -1;
    }

    monster_position.velocity_x = velocity_x;
    monster_position.velocity_y = velocity_y;
}

#[allow(clippy::similar_names)]
fn get_all_players_nearby(
    player_positions: &PlayersPositions,
    monster_position: &Position,
) -> HashMap<Uuid, Position> {
    let mut nearby_players = HashMap::new();
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
            nearby_players.insert(*player_id, position.clone());
        }
    }
    nearby_players
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
