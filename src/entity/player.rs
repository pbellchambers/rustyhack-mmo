use crate::entity::{Collidable, Entity, Location};
use crate::world_map::WorldMap;
use console_engine::{Color, ConsoleEngine, KeyCode};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct Player {
    pub location: Location,
    pub character_icon: char,
    pub collidable: Collidable,
    pub colour: Color,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            location: Location { x, y },
            character_icon: '@',
            collidable: Collidable::True,
            colour: Color::Magenta,
        }
    }

    pub fn create_player(entity_map: &mut HashMap<Uuid, Entity>) -> Uuid {
        let uuid = Uuid::new_v4();
        entity_map.insert(uuid, Entity::Player(Player::new(5, 5)));
        uuid
    }

    pub fn update_player_location(
        player: &mut Player,
        console: &ConsoleEngine,
        world_map: &WorldMap,
    ) {
        if console.is_key_held(KeyCode::Up) {
            player.location.y = check_lower_bounds(0, player.location.y, -1);
        } else if console.is_key_held(KeyCode::Down) {
            player.location.y = check_upper_bounds(world_map.boundary_y(), player.location.y, 1);
        } else if console.is_key_held(KeyCode::Left) {
            player.location.x = check_lower_bounds(0, player.location.x, -1);
        } else if console.is_key_held(KeyCode::Right) {
            player.location.x = check_upper_bounds(world_map.boundary_x(), player.location.x, 1);
        }
    }
}

fn check_upper_bounds(boundary: &u32, current_location: i32, movement: i32) -> i32 {
    if current_location + movement >= (boundary - 1) as i32 {
        info!("Player hit the map boundary.");
        return current_location;
    }
    current_location + movement
}

fn check_lower_bounds(boundary: u32, current_location: i32, movement: i32) -> i32 {
    if current_location + movement <= boundary as i32 {
        info!("Player hit the map boundary.");
        return current_location;
    }
    current_location + movement
}
