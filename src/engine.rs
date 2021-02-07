use crate::background_map;
use crate::background_map::tiles::{Collidable, Tile};
use crate::background_map::BackgroundMap;
use crate::consts::{DEFAULT_MAP, DEFAULT_PLAYER_POSITION};
use crate::ecs::components;
use crate::ecs::components::*;
use crate::viewport::Viewport;
use console_engine::{Color, ConsoleEngine, KeyCode, KeyModifiers};
use legion::*;
use std::collections::HashMap;
use uuid::Uuid;

pub fn run(width: u32, height: u32, target_fps: u32) {
    let viewport = Viewport::new(width, height, target_fps);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);
    let mut world = World::default();
    let all_maps: HashMap<String, BackgroundMap> = background_map::initialise_all_maps();

    let current_player_uuid = create_player(&mut world);
    info!("Initialised player: {}", &current_player_uuid);

    loop {
        console.wait_frame();
        console.clear_screen();
        world = update_player_input(world, &console, &current_player_uuid);
        world = update_entities(world, all_maps.get(DEFAULT_MAP).unwrap());
        viewport.draw_viewport_contents(
            &mut console,
            &world,
            all_maps.get(DEFAULT_MAP).unwrap(),
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

fn create_player(world: &mut World) -> Uuid {
    let uuid = Uuid::new_v4();
    let player = world.push((
        IsPlayer { is_player: true },
        DEFAULT_PLAYER_POSITION,
        components::Velocity { x: 0, y: 0 },
        CollisionState { collidable: true },
        Character { icon: '@' },
        EntityColour {
            colour: Color::Magenta,
        },
        VisibleState { visible: true },
        PlayerId { uuid },
    ));
    info!("Created player: {:?}, with uuid: {:?}", player, uuid);
    uuid
}

fn update_player_input(
    mut world: World,
    console: &ConsoleEngine,
    current_player_uuid: &Uuid,
) -> World {
    let mut query = <(&mut Velocity, &PlayerId)>::query();

    for (velocity, player_id) in query.iter_mut(&mut world) {
        if player_id.uuid == *current_player_uuid {
            if console.is_key_held(KeyCode::Up) {
                velocity.y = -1;
            } else if console.is_key_held(KeyCode::Down) {
                velocity.y = 1;
            } else if console.is_key_held(KeyCode::Left) {
                velocity.x = -1;
            } else if console.is_key_held(KeyCode::Right) {
                velocity.x = 1;
            }
            break;
        } else {
            // do nothing
        }
    }
    world
}

fn update_entities(mut world: World, background_map: &BackgroundMap) -> World {
    let mut query = <(&mut Velocity, &mut Position)>::query();

    for (velocity, position) in query.iter_mut(&mut world) {
        if !entity_is_colliding(background_map.get_tile_at(
            (position.x + velocity.x) as usize,
            (position.y + velocity.y) as usize,
        )) {
            position.x += velocity.x;
            position.y += velocity.y;
        }
        velocity.x = 0;
        velocity.y = 0;
    }
    world
}

fn entity_is_colliding(tile: Tile) -> bool {
    match tile {
        Tile::Door(door) => door.collidable == Collidable::True,
        Tile::Wall(wall) => wall.collidable == Collidable::True,
        Tile::Boundary => true,
        _ => false,
    }
}
