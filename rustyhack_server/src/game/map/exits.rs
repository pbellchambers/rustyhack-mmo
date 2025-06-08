use crate::consts;
use rustyhack_lib::utils::file;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

pub(crate) type AllMapExits = HashMap<String, Vec<MapExitPositions>>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MapExits {
    pub(crate) map_exits: Vec<MapExitPositions>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MapExitPositions {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) new_map: String,
    pub(crate) new_x: u32,
    pub(crate) new_y: u32,
}

pub(crate) fn initialise_all_map_exit_definitions() -> AllMapExits {
    info!("About to initialise all map exit positions");
    let mut all_map_exits: AllMapExits = HashMap::new();
    let mut file_location = file::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::MAP_EXITS_DIRECTORY);
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
        let exits: Vec<MapExitPositions> =
            get_map_exits_from_map_exit_file(&unwrapped_path.path()).map_exits;
        info!("Initialised map exit positions for map: {:?}", &map);
        all_map_exits.insert(map, exits);
    }
    all_map_exits
}

fn get_map_exits_from_map_exit_file(path: &Path) -> MapExits {
    let file = File::open(path).unwrap_or_else(|err| {
        error!(
            "Problem getting map exits definition from file: {}, error: {err}",
            path.display()
        );
        process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap_or_else(|err| {
        error!(
            "Problem deserializing map exits definition from file: {}, error: {err}",
            path.display()
        );
        process::exit(1);
    })
}
