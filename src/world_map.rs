mod character_map;
use crate::entity::Entity;
use std::{env, fs, process};

pub struct WorldMap {
    data: Vec<Vec<Entity>>,
}

impl WorldMap {
    pub fn new(filename: String) -> WorldMap {
        initialise_map(filename)
    }

    pub fn data(&self) -> &Vec<Vec<Entity>> {
        &self.data
    }

    pub fn get_entity_at(&self, x: usize, y: usize) -> Entity {
        self.data[y][x]
    }
}

fn initialise_map(filename: String) -> WorldMap {
    let unprocessed_map_data = load_map_data_from_file(filename);
    let data = process_map_data(&unprocessed_map_data);
    WorldMap { data }
}

fn process_map_data(data: &str) -> Vec<Vec<Entity>> {
    let mut processed_data: Vec<Vec<Entity>> = Vec::new();
    let mut row_data: Vec<Entity> = Vec::new();
    let mut entity: Entity;
    let mut current_x = 0;
    let mut current_y = 0;
    for character in data.chars() {
        entity = character_map::map_character_to_entity(current_x, current_y, character);
        match entity {
            Entity::NewLine => {
                processed_data.push(row_data.clone());
                row_data.clear();
                current_x = 0;
                current_y += 1;
            }
            Entity::CarriageReturn => {
                //do nothing - handles builds on windows
            }
            Entity::EndOfFile => processed_data.push(row_data.clone()),
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
