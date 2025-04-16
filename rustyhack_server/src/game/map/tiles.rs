use crate::consts;
use crate::game::map::array_utils;
use ndarray::Array2;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::background_map::tiles::{Collidable, Tile};
use rustyhack_lib::background_map::{BackgroundMap, character_map};
use rustyhack_lib::utils::file;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, process};

pub(crate) fn initialise_all_maps() -> AllMaps {
    info!("About to initialise all maps");
    let mut all_maps: AllMaps = HashMap::new();
    let mut file_location = file::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::MAPS_DIRECTORY);
    let paths = file::get_all_files_in_location(&file_location);
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
        error!("Problem getting map data from file: {path:?}, error: {err}");
        process::exit(1);
    })
}

fn process_map_data(data: &str) -> Array2<Tile> {
    let mut processed_data: Vec<Vec<Tile>> = Vec::new();
    let mut row_data: Vec<Tile> = Vec::new();
    let mut entity: Tile;
    let mut current_x = 0;
    let mut max_x_len = 0;
    let mut current_y = 0;
    debug!("Beginning to process map data into Vec.");
    for character in data.chars() {
        entity = character_map::map_character_to_tile(current_x, current_y, character);
        match entity {
            Tile::NewLine => {
                processed_data.push(row_data.clone());
                if row_data.len() > max_x_len {
                    max_x_len = row_data.len();
                }
                row_data.clear();
                current_x = 0;
                current_y += 1;
            }
            Tile::CarriageReturn => {
                //do nothing - handles builds on windows
            }
            Tile::EndOfFile => {
                processed_data.push(row_data.clone());
                if row_data.len() > max_x_len {
                    max_x_len = row_data.len();
                }
            }
            _ => {
                row_data.push(entity);
                current_x += 1;
            }
        }
    }
    array_utils::pad_all_rows(&mut processed_data, max_x_len, Tile::EmptySpace);
    debug!("Finished processing map data into Array2.");
    array_utils::vec_to_array(&processed_data)
}

pub(crate) fn entity_is_colliding_with_tile(tile: &Tile) -> bool {
    match tile {
        Tile::Door(door) => door.collidable == Collidable::True,
        Tile::Wall(wall) => wall.collidable == Collidable::True,
        Tile::Boundary => true,
        _ => false,
    }
}
