mod character_map;
pub(crate) mod tiles;

use crate::background_map::tiles::Tile;
use std::{env, fs, process};

pub struct BackgroundMap {
    data: Vec<Vec<Tile>>,
}

impl BackgroundMap {
    pub fn new(filename: String) -> BackgroundMap {
        initialise_map(filename)
    }

    pub fn data(&self) -> &Vec<Vec<Tile>> {
        &self.data
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Tile {
        self.data[y][x]
    }
}

fn initialise_map(filename: String) -> BackgroundMap {
    let unprocessed_map_data = load_map_data_from_file(filename);
    let data = process_map_data(&unprocessed_map_data);
    BackgroundMap { data }
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

fn load_map_data_from_file(filename: String) -> String {
    let mut file_location = env::current_exe().unwrap_or_else(|err| {
        error!("Problem getting current executable location: {}", err);
        process::exit(1);
    });
    file_location.pop();
    file_location.push("assets");
    file_location.push("maps");
    file_location.push(filename);
    fs::read_to_string(&file_location.as_path()).unwrap_or_else(|err| {
        error!(
            "Problem getting map data from file: {:?}, error: {}",
            file_location, err
        );
        process::exit(1);
    })
}
