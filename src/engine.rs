use crate::entity::player::Player;
use crate::entity::Entity;
use crate::viewport::Viewport;
use crate::world_map::WorldMap;
use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use std::collections::HashMap;

pub fn run(width: u32, height: u32, target_fps: u32) {
    let viewport = Viewport::new(width, height, target_fps);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);
    let mut entity_map = HashMap::new();
    let world_map = WorldMap::new(String::from("default.txt"));
    info!("Initialised default map.");

    let current_player_uuid = Player::create_player(&mut entity_map);
    info!("Initialised player: {}", &current_player_uuid);

    loop {
        console.wait_frame();
        console.clear_screen();
        Entity::update_entities(&console, &mut entity_map, &world_map);
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
