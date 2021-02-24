use crate::consts;
use rustyhack_lib::file_utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

pub type AllSpawns = HashMap<String, Spawns>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spawns {
    pub monsters: Vec<MonsterSpawns>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterSpawns {
    pub monster_type: String,
    pub spawn_positions: Vec<PositionWithoutMap>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionWithoutMap {
    pub x: i32,
    pub y: i32,
}

pub(crate) fn initialise_all_spawn_definitions() -> AllSpawns {
    info!("About to initialise all spawn definitions");
    let mut all_spawns: HashMap<String, Spawns> = HashMap::new();
    let paths = file_utils::get_all_files_in_location(spawns_directory_location());
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

fn spawns_directory_location() -> PathBuf {
    let mut file_location = file_utils::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::SPAWNS_DIRECTORY);
    file_location
}

fn get_spawns_definition_from_path(path: &PathBuf) -> Spawns {
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
            "Problem deserialising spawns definition from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    })
}
