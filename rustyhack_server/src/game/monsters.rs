use crate::consts;
use crate::consts::MONSTER_DISTANCE_ACTIVATION;
use crate::game::players;
use crate::game::spawns::AllSpawns;
use legion::{IntoQuery, World};
use rand::Rng;
use rustyhack_lib::ecs::components::{DisplayDetails, MonsterDetails, Position, Stats, Velocity};
use rustyhack_lib::ecs::monster::{AllMonsterDefinitions, Monster};
use rustyhack_lib::file_utils;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process;
use uuid::Uuid;

pub(crate) fn initialise_all_monster_definitions() -> AllMonsterDefinitions {
    info!("About to initialise all monster definitions");
    let mut all_monster_definitions: AllMonsterDefinitions = HashMap::new();
    let paths = file_utils::get_all_files_in_location(monsters_directory_location());
    for path in paths {
        let unwrapped_path = path.unwrap();
        let name = String::from(
            unwrapped_path
                .file_name()
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .unwrap(),
        );
        let monster: Monster = get_monster_definition_from_path(&unwrapped_path.path());
        info!("Initialised monster: {:?}", &name);
        all_monster_definitions.insert(name, monster);
    }
    all_monster_definitions
}

fn monsters_directory_location() -> PathBuf {
    let mut file_location = file_utils::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::MONSTERS_DIRECTORY);
    file_location
}

fn get_monster_definition_from_path(path: &Path) -> Monster {
    let file = File::open(path).unwrap_or_else(|err| {
        error!(
            "Problem getting monster definition from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap_or_else(|err| {
        error!(
            "Problem deserialising monster definition from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    })
}

pub(crate) fn spawn_initial_monsters(
    world: &mut World,
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns: &AllSpawns,
) {
    info!("Spawning initial monsters.");
    let mut monsters_vec: Vec<(MonsterDetails, DisplayDetails, Position, Velocity, Stats)> = vec![];
    for (map, spawns) in all_spawns {
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
                    x: spawn_position.x,
                    y: spawn_position.y,
                    map: map.clone(),
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
                    current_monster.velocity,
                    current_monster.stats,
                ));
            }
        }
    }
    world.extend(monsters_vec);
}

pub(crate) fn update_velocities(world: &mut World) {
    debug!("Updating monster velocities.");
    let players_positions = players::get_current_player_positions(world);

    let mut query = <(&Position, &mut Velocity, &mut MonsterDetails)>::query();
    for (position, velocity, monster) in query.iter_mut(world) {
        let mut moving_towards_existing_target = false;

        if let Some(target) = monster.current_target.clone() {
            if let Some(current_target_position) = players_positions.get(&target) {
                if is_specific_player_nearby(current_target_position, position) {
                    *velocity = move_towards_target(position, current_target_position);
                    moving_towards_existing_target = true;
                }
            }
        }

        if !moving_towards_existing_target {
            let nearby_player = is_any_player_nearby(&players_positions, position);
            match nearby_player {
                Some((player_name, player_position)) => {
                    monster.is_active = true;
                    monster.current_target = Some(player_name.clone());
                    *velocity = move_towards_target(position, player_position);
                }
                None => {
                    debug!("Monster returning to spawn location");
                    monster.is_active = false;
                    monster.current_target = None;
                    *velocity = move_towards_target(position, &monster.spawn_position);
                }
            }
        }
    }
}

fn move_towards_target(monster_position: &Position, target_position: &Position) -> Velocity {
    let diff_x = monster_position.x - target_position.x;
    let diff_y = monster_position.y - target_position.y;
    let mut new_pos_x = monster_position.x;
    let mut new_pos_y = monster_position.y;

    match diff_x.abs().cmp(&diff_y.abs()) {
        Ordering::Greater => new_pos_x = move_towards(diff_x, monster_position.x),
        Ordering::Less => new_pos_y = move_towards(diff_y, monster_position.y),
        Ordering::Equal => {
            let mut rng = rand::thread_rng();
            if rng.gen::<bool>() {
                new_pos_x = move_towards(diff_x, monster_position.x)
            } else {
                new_pos_y = move_towards(diff_y, monster_position.y)
            }
        }
    }
    Velocity {
        x: new_pos_x - monster_position.x,
        y: new_pos_y - monster_position.y,
    }
}

fn move_towards(diff: i32, position: i32) -> i32 {
    if diff.abs() as u32 > 1 {
        return match diff.is_positive() {
            true => position - 1,
            false => position + 1,
        };
    }
    position
}

fn is_any_player_nearby<'a>(
    player_positions: &'a HashMap<String, Position>,
    monster_position: &Position,
) -> Option<(&'a String, &'a Position)> {
    let monster_x_range = (monster_position.x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position.y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.y + MONSTER_DISTANCE_ACTIVATION);
    for (player_name, position) in player_positions {
        if monster_x_range.contains(&position.x)
            && monster_y_range.contains(&position.y)
            && monster_position.map == position.map
        {
            debug!("There is a player near a monster");
            return Some((player_name, position));
        }
    }
    None
}

fn is_specific_player_nearby(
    current_target_position: &Position,
    monster_position: &Position,
) -> bool {
    let monster_x_range = (monster_position.x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position.y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.y + MONSTER_DISTANCE_ACTIVATION);

    if monster_x_range.contains(&current_target_position.x)
        && monster_y_range.contains(&current_target_position.y)
        && monster_position.map == current_target_position.map
    {
        return true;
    }
    false
}
