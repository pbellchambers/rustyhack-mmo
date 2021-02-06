use crate::entity::player::Player;
use crate::entity::{Collidable, Entity, Velocity};
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

fn update_player_input(player: &mut Player, console: &ConsoleEngine) {
    if console.is_key_held(KeyCode::Up) {
        player.velocity = Velocity::Up(1);
    } else if console.is_key_held(KeyCode::Down) {
        player.velocity = Velocity::Down(1);
    } else if console.is_key_held(KeyCode::Left) {
        player.velocity = Velocity::Left(1);
    } else if console.is_key_held(KeyCode::Right) {
        player.velocity = Velocity::Right(1);
    }
}

fn update_entities(
    console: &ConsoleEngine,
    entity_map: &mut HashMap<Uuid, Entity>,
    world_map: &WorldMap,
) {
    for entity in entity_map.values_mut() {
        if let Entity::Player(ref mut player) = entity {
            update_player_input(player, &console);
            resolve_movement(player, &world_map);
        }
    }
}

fn resolve_movement(player: &mut Player, world_map: &WorldMap) {
    match player.velocity {
        Velocity::Up(distance) => {
            if !entity_is_colliding(world_map.get_entity_at(
                player.location.x as usize,
                (player.location.y - distance) as usize,
            )) {
                player.location.y -= distance;
                player.velocity = Velocity::Stationary;
            }
        }
        Velocity::Down(distance) => {
            if !entity_is_colliding(world_map.get_entity_at(
                player.location.x as usize,
                (player.location.y + distance) as usize,
            )) {
                player.location.y += distance;
                player.velocity = Velocity::Stationary;
            }
        }
        Velocity::Left(distance) => {
            if !entity_is_colliding(world_map.get_entity_at(
                (player.location.x - distance) as usize,
                player.location.y as usize,
            )) {
                player.location.x -= distance;
                player.velocity = Velocity::Stationary;
            }
        }
        Velocity::Right(distance) => {
            if !entity_is_colliding(world_map.get_entity_at(
                (player.location.x + distance) as usize,
                player.location.y as usize,
            )) {
                player.location.x += distance;
                player.velocity = Velocity::Stationary;
            }
        }
        Velocity::Stationary => {}
    }
}

fn entity_is_colliding(entity: Entity) -> bool {
    match entity {
        Entity::Door(door) => door.collidable == Collidable::True,
        Entity::Wall(wall) => wall.collidable == Collidable::True,
        Entity::Boundary => true,
        _ => false,
    }
}
