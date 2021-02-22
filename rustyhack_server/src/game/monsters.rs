use crate::consts::MONSTER_DISTANCE_ACTIVATION;
use legion::storage::IntoComponentSource;
use legion::{IntoQuery, World};
use rand::Rng;
use rustyhack_lib::ecs::components::{MonsterDetails, PlayerDetails, Position};
use rustyhack_lib::ecs::monster::Monster;
use std::cmp::Ordering;
use std::collections::HashMap;

pub(crate) fn spawn_monsters() -> impl IntoComponentSource {
    let monster1 = Monster::default();
    vec![(
        monster1.monster_details,
        monster1.display_details,
        monster1.position,
        monster1.velocity,
    )]
}

pub(crate) fn update_positions(world: &mut World) {
    debug!("Updating monster positions.");
    let mut players_positions: HashMap<String, Position> = HashMap::new();
    let mut query = <(&Position, &PlayerDetails)>::query();
    for (position, player) in query.iter(world) {
        //only add online players
        if player.currently_online {
            players_positions.insert(player.player_name.clone(), position.clone());
        }
    }

    let mut query = <(&mut Position, &mut MonsterDetails)>::query();
    for (position, monster) in query.iter_mut(world) {
        let is_player_nearby = is_player_nearby(&players_positions, &position);
        if is_player_nearby != None {
            let (player_name, player_position) = is_player_nearby.unwrap();
            monster.is_active = true;
            monster.current_target = Some(player_name.clone());
            *position = move_towards_target(position, player_position);
        } else {
            debug!("Monster returning to spawn location");
            monster.is_active = false;
            monster.current_target = None;
            *position = move_towards_target(position, &monster.spawn_location);
        }
    }
}

fn move_towards_target(monster_position: &Position, target_position: &Position) -> Position {
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
    Position {
        x: new_pos_x,
        y: new_pos_y,
        map: monster_position.map.clone(),
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

fn is_player_nearby<'a>(
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
