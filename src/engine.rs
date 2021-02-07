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

pub fn run(width: u32, height: u32, target_fps: u32) {
    let viewport = Viewport::new(width, height, target_fps);
    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);
    let mut world = World::default();
    let all_maps: HashMap<String, BackgroundMap> = background_map::initialise_all_maps();

    let current_player_entity = create_player(&mut world);

    loop {
        console.wait_frame();
        console.clear_screen();

        let mut player_updates = get_player_updates(current_player_entity, &console);

        let mut schedule = Schedule::builder()
            .add_system(update_player_input_system())
            .build();

        schedule.execute(&mut world, &mut player_updates);

        world = update_entities(world, all_maps.get(DEFAULT_MAP).unwrap());
        viewport.draw_viewport_contents(&mut console, &world, all_maps.get(DEFAULT_MAP).unwrap());
        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            break;
        }
    }
}

fn get_player_updates(current_player_entity: Entity, console: &ConsoleEngine) -> Resources {
    let mut resources = Resources::default();
    let mut updates = HashMap::new();
    if console.is_key_held(KeyCode::Up) {
        updates.insert(current_player_entity, Velocity { x: 0, y: -1 });
    } else if console.is_key_held(KeyCode::Down) {
        updates.insert(current_player_entity, Velocity { x: 0, y: 1 });
    } else if console.is_key_held(KeyCode::Left) {
        updates.insert(current_player_entity, Velocity { x: -1, y: 0 });
    } else if console.is_key_held(KeyCode::Right) {
        updates.insert(current_player_entity, Velocity { x: 1, y: 0 });
    }
    resources.insert(updates);
    resources
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}

fn create_player(world: &mut World) -> Entity {
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
    ));
    info!("Created player: {:?}", player);
    player
}

#[system(par_for_each)]
fn update_player_input(
    entity: &Entity,
    velocity: &mut Velocity,
    #[resource] player_updates: &HashMap<Entity, Velocity>,
) {
    for (update_entity, update_velocity) in player_updates {
        if update_entity == entity {
            velocity.x = update_velocity.x;
            velocity.y = update_velocity.y;
        }
    }
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
