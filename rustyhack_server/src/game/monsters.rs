use crate::consts::MONSTER_DISTANCE_ACTIVATION;
use crate::game::players;
use console_engine::Color;
use legion::storage::IntoComponentSource;
use legion::{IntoQuery, World};
use rand::Rng;
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{DisplayDetails, MonsterDetails, Position, Velocity};
use rustyhack_lib::ecs::monster::Monster;
use std::cmp::Ordering;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) fn spawn_monsters() -> impl IntoComponentSource {
    //todo load monsters from metadata file
    let zombie1 = Monster {
        monster_details: MonsterDetails {
            id: Uuid::new_v4(),
            monster_type: "Zombie".to_string(),
            spawn_location: Position {
                x: 50,
                y: 27,
                map: DEFAULT_MAP.to_string(),
            },
            is_active: false,
            current_target: None,
        },
        display_details: DisplayDetails {
            icon: 'z',
            colour: Color::Grey,
            visible: true,
            collidable: true,
        },
        position: Position {
            x: 50,
            y: 27,
            map: DEFAULT_MAP.to_string(),
        },
        velocity: Velocity { x: 0, y: 0 },
    };
    let zombie2 = Monster {
        monster_details: MonsterDetails {
            id: Uuid::new_v4(),
            monster_type: "Zombie".to_string(),
            spawn_location: Position {
                x: 65,
                y: 27,
                map: DEFAULT_MAP.to_string(),
            },
            is_active: false,
            current_target: None,
        },
        display_details: DisplayDetails {
            icon: 'z',
            colour: Color::Grey,
            visible: true,
            collidable: true,
        },
        position: Position {
            x: 65,
            y: 27,
            map: DEFAULT_MAP.to_string(),
        },
        velocity: Velocity { x: 0, y: 0 },
    };
    let snake1 = Monster {
        monster_details: MonsterDetails {
            id: Uuid::new_v4(),
            monster_type: "Snake".to_string(),
            spawn_location: Position {
                x: 88,
                y: 3,
                map: DEFAULT_MAP.to_string(),
            },
            is_active: false,
            current_target: None,
        },
        display_details: DisplayDetails {
            icon: 's',
            colour: Color::DarkGreen,
            visible: true,
            collidable: true,
        },
        position: Position {
            x: 88,
            y: 3,
            map: DEFAULT_MAP.to_string(),
        },
        velocity: Velocity { x: 0, y: 0 },
    };
    let snake2 = Monster {
        monster_details: MonsterDetails {
            id: Uuid::new_v4(),
            monster_type: "Snake".to_string(),
            spawn_location: Position {
                x: 96,
                y: 5,
                map: DEFAULT_MAP.to_string(),
            },
            is_active: false,
            current_target: None,
        },
        display_details: DisplayDetails {
            icon: 's',
            colour: Color::DarkGreen,
            visible: true,
            collidable: true,
        },
        position: Position {
            x: 96,
            y: 5,
            map: DEFAULT_MAP.to_string(),
        },
        velocity: Velocity { x: 0, y: 0 },
    };
    info!("UUIDs: {} {} {} {}", zombie1.monster_details.id.to_string(),zombie2.monster_details.id.to_string(),snake1.monster_details.id.to_string(),snake2.monster_details.id.to_string());

    vec![
        (
            zombie1.monster_details,
            zombie1.display_details,
            zombie1.position,
            zombie1.velocity,
        ),
        (
            zombie2.monster_details,
            zombie2.display_details,
            zombie2.position,
            zombie2.velocity,
        ),
        (
            snake1.monster_details,
            snake1.display_details,
            snake1.position,
            snake1.velocity,
        ),
        (
            snake2.monster_details,
            snake2.display_details,
            snake2.position,
            snake2.velocity,
        ),
    ]
}

pub(crate) fn update_velocities(world: &mut World) {
    debug!("Updating monster velocities.");
    let players_positions = players::get_current_player_positions(world);

    let mut query = <(&Position, &mut Velocity, &mut MonsterDetails)>::query();
    for (position, velocity, monster) in query.iter_mut(world) {
        let mut moving_towards_existing_target = false;

        if let Some(target) = monster.current_target.clone() {
            if let Some(current_target_position) = players_positions.get(&target) {
                if is_specific_player_nearby(&current_target_position, &position) {
                    *velocity = move_towards_target(position, current_target_position);
                    moving_towards_existing_target = true;
                }
            }
        }

        if !moving_towards_existing_target {
            let nearby_player = is_any_player_nearby(&players_positions, &position);
            match nearby_player {
                Some((player_name, player_position)) => {
                    monster.is_active = true;
                    monster.current_target = Some(player_name.clone());
                    *velocity = move_towards_target(position, player_position);
                }
                None => {
                    debug!("Monster returning to spawn location");
                    monster.is_active = false;
                    monster.current_target = None;
                    *velocity = move_towards_target(position, &monster.spawn_location);
                }
            }
        }
    }
}

fn move_towards_target(monster_position: &Position, target_position: &Position) -> Velocity {
    let diff_x = monster_position.x - target_position.x;
    let diff_y = monster_position.y - target_position.y;
    let mut new_pos_x = monster_position.x;
    let mut new_pos_y = monster_position.y;

    match diff_x.abs().cmp(&diff_y.abs()) {
        Ordering::Greater => new_pos_x = move_towards(diff_x, monster_position.x),
        Ordering::Less => new_pos_y = move_towards(diff_y, monster_position.y),
        Ordering::Equal => {
            let mut rng = rand::thread_rng();
            if rng.gen::<bool>() {
                new_pos_x = move_towards(diff_x, monster_position.x)
            } else {
                new_pos_y = move_towards(diff_y, monster_position.y)
            }
        }
    }
    Velocity {
        x: new_pos_x - monster_position.x,
        y: new_pos_y - monster_position.y,
    }
}

fn move_towards(diff: i32, position: i32) -> i32 {
    if diff.abs() as u32 > 1 {
        return match diff.is_positive() {
            true => position - 1,
            false => position + 1,
        };
    }
    position
}

fn is_any_player_nearby<'a>(
    player_positions: &'a HashMap<String, Position>,
    monster_position: &Position,
) -> Option<(&'a String, &'a Position)> {
    let monster_x_range = (monster_position.x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position.y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.y + MONSTER_DISTANCE_ACTIVATION);
    for (player_name, position) in player_positions {
        if monster_x_range.contains(&position.x)
            && monster_y_range.contains(&position.y)
            && monster_position.map == position.map
        {
            debug!("There is a player near a monster");
            return Some((player_name, position));
        }
    }
    None
}

fn is_specific_player_nearby(
    current_target_position: &Position,
    monster_position: &Position,
) -> bool {
    let monster_x_range = (monster_position.x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position.y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position.y + MONSTER_DISTANCE_ACTIVATION);

    if monster_x_range.contains(&current_target_position.x)
        && monster_y_range.contains(&current_target_position.y)
        && monster_position.map == current_target_position.map
    {
        return true;
    }
    false
}
