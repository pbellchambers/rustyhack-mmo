use crate::consts;
use rustyhack_lib::file_utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

pub type AllSpawnsMap = HashMap<String, Spawns>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spawns {
    pub monsters: Vec<MonsterSpawnPositions>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterSpawnPositions {
    pub monster_type: String,
    pub spawn_positions: Vec<PositionWithoutMap>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionWithoutMap {
    pub x: usize,
    pub y: usize,
}

pub(crate) fn initialise_all_spawn_definitions() -> AllSpawnsMap {
    info!("About to initialise all spawn definitions");
    let mut all_spawns: HashMap<String, Spawns> = HashMap::new();
    let mut file_location = file_utils::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::SPAWNS_DIRECTORY);
    let paths = file_utils::get_all_files_in_location(&file_location);
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
        let spawns: Spawns = get_spawns_definition_from_path(&unwrapped_path.path());
        info!("Initialised spawn definitions for map: {:?}", &name);
        all_spawns.insert(name, spawns);
    }
    all_spawns
}

fn get_spawns_definition_from_path(path: &Path) -> Spawns {
    let file = File::open(path).unwrap_or_else(|err| {
        error!(
            "Problem getting spawns definition from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap_or_else(|err| {
        error!(
            "Problem deserializing spawns definition from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    })
}
