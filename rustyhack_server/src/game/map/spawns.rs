use crate::consts;
use rustyhack_lib::utils::file;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

pub(crate) type AllSpawnsMap = HashMap<String, Spawns>;
pub(crate) type AllSpawnCounts = HashMap<String, HashMap<String, u32>>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Spawns {
    pub(crate) monsters: Vec<MonsterSpawnPositions>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MonsterSpawnPositions {
    pub(crate) monster_type: String,
    pub(crate) spawn_positions: Vec<PositionWithoutMap>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PositionWithoutMap {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

pub(crate) fn initialise_all_spawn_definitions() -> (AllSpawnCounts, AllSpawnsMap) {
    let all_spawns_map = get_all_spawns_positions();
    (get_default_spawn_counts(&all_spawns_map), all_spawns_map)
}

fn get_all_spawns_positions() -> AllSpawnsMap {
    info!("About to initialise all spawn positions");
    let mut all_spawns: HashMap<String, Spawns> = HashMap::new();
    let mut file_location = file::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::SPAWNS_DIRECTORY);
    let paths = file::get_all_files_in_location(&file_location);
    for path in paths {
        let unwrapped_path = path.unwrap();
        let map = String::from(
            unwrapped_path
                .file_name()
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .unwrap(),
        );
        let spawns: Spawns = get_spawns_definition_from_path(&unwrapped_path.path());
        info!("Initialised spawn definitions for map: {:?}", &map);
        all_spawns.insert(map, spawns);
    }
    all_spawns
}

fn get_default_spawn_counts(all_spawns_map: &AllSpawnsMap) -> AllSpawnCounts {
    info!("About to initialise all spawn counts");
    let mut default_spawn_counts: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for (map, spawns) in all_spawns_map {
        let mut map_spawn_counts: HashMap<String, u32> = HashMap::new();
        for monster in &spawns.monsters {
            let mut monster_spawn_count: u32 = 0;
            for _spawn_position in &monster.spawn_positions {
                monster_spawn_count += 1;
            }
            map_spawn_counts.insert(monster.monster_type.clone(), monster_spawn_count);
        }
        default_spawn_counts.insert(map.clone(), map_spawn_counts);
    }
    info!("All spawn counts are: {default_spawn_counts:?}");
    default_spawn_counts
}

fn get_spawns_definition_from_path(path: &Path) -> Spawns {
    let file = File::open(path).unwrap_or_else(|err| {
        error!("Problem getting spawns definition from file: {path:?}, error: {err}");
        process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap_or_else(|err| {
        error!("Problem deserializing spawns definition from file: {path:?}, error: {err}");
        process::exit(1);
    })
}
