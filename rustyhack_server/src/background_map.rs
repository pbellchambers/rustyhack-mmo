use rustyhack_lib::background_map::tiles::Tile;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::background_map::{character_map, BackgroundMap};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs, process};

pub fn initialise_all_maps() -> AllMaps {
    let mut all_maps: AllMaps = HashMap::new();
    let file_location = get_maps_directory_location();
    let paths = fs::read_dir(file_location.as_path()).unwrap();
    for path in paths {
        let unwrapped_path = path.unwrap();
        let filename = String::from(unwrapped_path.file_name().to_str().unwrap());
        let map = initialise_map(&unwrapped_path.path());
        info!("Initialised map: {:?}", &filename);
        all_maps.insert(filename, map);
    }
    info!("Finished initialising all maps.");
    all_maps
}

fn get_maps_directory_location() -> PathBuf {
    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push("assets");
    file_location.push("maps");
    file_location
}

fn initialise_map(path: &PathBuf) -> BackgroundMap {
    let unprocessed_map_data = load_map_data_from_file(path);
    let data = process_map_data(&unprocessed_map_data);
    BackgroundMap { data }
}

fn load_map_data_from_file(path: &PathBuf) -> String {
    fs::read_to_string(path.as_path()).unwrap_or_else(|err| {
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
    processed_data
}
