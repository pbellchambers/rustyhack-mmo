use crate::entity::player::Player;
use crate::entity::Entity;
use crate::viewport::Viewport;
use console_engine::{pixel, ConsoleEngine, KeyCode, KeyModifiers};
use std::collections::HashMap;
use uuid::Uuid;

pub fn run(width: u32, height: u32, target_fps: u32) {
    let viewport = Viewport::new(width, height, target_fps);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);
    let mut entity_map = HashMap::new();

    create_player(&mut entity_map);
    loop {
        console.wait_frame();
        console.clear_screen();
        draw_viewport_boundary(&mut console, &viewport);
        update_entities(&console, &mut entity_map, &viewport);
        draw_entities(&mut console, &entity_map);
        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            break;
        }
    }
}

fn create_player(entity_map: &mut HashMap<Uuid, Entity>) {
    entity_map.insert(Uuid::new_v4(), Entity::Player(Player::new()));
}

fn update_entities(
    console: &ConsoleEngine,
    entity_map: &mut HashMap<Uuid, Entity>,
    viewport: &Viewport,
) {
    for (_, entity) in entity_map {
        match entity {
            Entity::Player(ref mut player) => update_player_location(player, &console, &viewport),
        }
    }
}

fn update_player_location(player: &mut Player, console: &ConsoleEngine, viewport: &Viewport) {
    if console.is_key_pressed(KeyCode::Up) {
        player.location.y = check_lower_bounds(0, player.location.y, -1);
    } else if console.is_key_pressed(KeyCode::Down) {
        player.location.y = check_upper_bounds(viewport.height, player.location.y, 1);
    } else if console.is_key_pressed(KeyCode::Left) {
        player.location.x = check_lower_bounds(0, player.location.x, -1);
    } else if console.is_key_pressed(KeyCode::Right) {
        player.location.x = check_upper_bounds(viewport.width, player.location.x, 1);
    }
}

fn check_upper_bounds(boundary: u32, current_location: i32, movement: i32) -> i32 {
    if current_location + movement >= (boundary - 1) as i32 {
        info!("Entity hit the boundary.");
        return current_location;
    }
    current_location + movement
}

fn check_lower_bounds(boundary: u32, current_location: i32, movement: i32) -> i32 {
    if current_location + movement <= boundary as i32 {
        info!("Entity hit the boundary.");
        return current_location;
    }
    current_location + movement
}

fn draw_entities(console: &mut ConsoleEngine, entity_map: &HashMap<Uuid, Entity>) {
    for (_, entity) in entity_map {
        match entity {
            Entity::Player(player) => console.print(
                player.location.x,
                player.location.y,
                &player.character_icon.to_string(),
            ),
        }
    }
    console.draw();
}

fn draw_viewport_boundary(console: &mut ConsoleEngine, viewport: &Viewport) {
    console.rect(
        0,
        0,
        (viewport.width - 1) as i32,
        (viewport.height - 1) as i32,
        pixel::pxl('#'),
    );
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}
