use crate::entity::player::Player;
use crate::entity::Entity;
use crate::viewport::Viewport;
use crate::world_map::WorldMap;
use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use std::collections::HashMap;
use uuid::Uuid;

pub fn run(width: u32, height: u32, target_fps: u32) {
    let viewport = Viewport::new(width, height, target_fps);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);
    let mut entity_map = HashMap::new();
    let world_map = WorldMap::new(String::from("default.txt"));
    info!("Initialised default map.");

    let current_player_uuid = create_player(&mut entity_map);
    info!("Initialised player: {}", &current_player_uuid);

    loop {
        console.wait_frame();
        console.clear_screen();
        update_entities(&console, &mut entity_map, &world_map);
        Viewport::draw_viewport_contents(
            &mut console,
            &entity_map,
            &viewport,
            &world_map,
            &current_player_uuid,
        );
        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            break;
        }
    }
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}

fn create_player(entity_map: &mut HashMap<Uuid, Entity>) -> Uuid {
    let uuid = Uuid::new_v4();
    entity_map.insert(uuid, Entity::Player(Player::new(5, 5)));
    uuid
}

fn update_player_location(player: &mut Player, console: &ConsoleEngine, world_map: &WorldMap) {
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

fn update_entities(
    console: &ConsoleEngine,
    entity_map: &mut HashMap<Uuid, Entity>,
    world_map: &WorldMap,
) {
    for entity in entity_map.values_mut() {
        if let Entity::Player(ref mut player) = entity {
            update_player_location(player, &console, &world_map)
        }
    }
}
