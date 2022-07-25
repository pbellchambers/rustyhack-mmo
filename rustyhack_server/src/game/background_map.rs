use crate::consts;
use rustyhack_lib::background_map::tiles::Tile;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::background_map::{character_map, BackgroundMap};
use rustyhack_lib::file_utils;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, process};

pub(crate) fn initialise_all_maps() -> AllMaps {
    info!("About to initialise all maps");
    let mut all_maps: AllMaps = HashMap::new();
    let mut file_location = file_utils::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::MAPS_DIRECTORY);
    let paths = file_utils::get_all_files_in_location(&file_location);
    for path in paths {
        let unwrapped_path = path.unwrap();
        let filename = String::from(
            unwrapped_path
                .file_name()
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .unwrap(),
        );
        let map = initialise_map(&unwrapped_path.path());
        info!("Initialised map: {:?}", &filename);
        all_maps.insert(filename, map);
    }
    info!("Finished initialising all maps.");
    all_maps
}

fn initialise_map(path: &Path) -> BackgroundMap {
    let unprocessed_map_data = load_map_data_from_file(path);
    let data = process_map_data(&unprocessed_map_data);
    BackgroundMap { data }
}

fn load_map_data_from_file(path: &Path) -> String {
    info!("Loading map data from file: {:?}", &path);
    fs::read_to_string(path).unwrap_or_else(|err| {
        error!(
            "Problem getting map data from file: {:?}, error: {}",
            path, err
        );
        process::exit(1);
    })
}

fn process_map_data(data: &str) -> Vec<Vec<Tile>> {
    let mut processed_data: Vec<Vec<Tile>> = Vec::new();
    let mut row_data: Vec<Tile> = Vec::new();
    let mut entity: Tile;
    let mut current_x = 0;
    let mut current_y = 0;
    debug!("Beginning to process map data into Vec.");
    for character in data.chars() {
        entity = character_map::map_character_to_tile(current_x, current_y, character);
        match entity {
            Tile::NewLine => {
                processed_data.push(row_data.clone());
                row_data.clear();
                current_x = 0;
                current_y += 1;
            }
            Tile::CarriageReturn => {
                //do nothing - handles builds on windows
            }
            Tile::EndOfFile => processed_data.push(row_data.clone()),
            _ => {
                row_data.push(entity);
                current_x += 1;
            }
        }
    }
    debug!("Finished processing map data into Vec.");
    processed_data
}
