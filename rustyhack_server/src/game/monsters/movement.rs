use crate::consts::MONSTER_DISTANCE_ACTIVATION;
use crate::game::players::PlayersPositions;
use rand::Rng;
use rustyhack_lib::ecs::components::Position;
use rustyhack_lib::utils::math::i32_from;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) fn move_towards_target(monster_position: &mut Position, target_position: &Position) {
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let diff_x: i32 = monster_position_x - i32_from(target_position.pos_x);
    let diff_y: i32 = monster_position_y - i32_from(target_position.pos_y);
    let mut new_pos_x = monster_position_x;
    let mut new_pos_y = monster_position_y;

    if (diff_x.abs() >= 1 && diff_y.abs() >= 1) || (diff_x == 0 && diff_y == 0) {
        //far away, move randomly towards
        let mut rng = rand::thread_rng();
        if rng.gen::<bool>() {
            new_pos_x = move_towards(diff_x, monster_position_x);
        } else {
            new_pos_y = move_towards(diff_y, monster_position_y);
        }
    } else if diff_x.abs() > 1 && diff_y.abs() == 0 {
        //in line, should mostly move towards, but sometimes randomly
        let mut rng = rand::thread_rng();
        if rng.gen_range(1..=6) > 1 {
            new_pos_x = move_towards(diff_x, monster_position_x);
        } else if rng.gen::<bool>() {
            new_pos_y = move_towards(diff_y + 1, monster_position_y);
        } else {
            new_pos_y = move_towards(diff_y - 1, monster_position_y);
        }
    } else if diff_x.abs() == 0 && diff_y.abs() > 1 {
        //in line, should mostly move towards, but sometimes randomly
        let mut rng = rand::thread_rng();
        if rng.gen_range(1..=6) > 1 {
            new_pos_y = move_towards(diff_y, monster_position_y);
        } else if rng.gen::<bool>() {
            new_pos_x = move_towards(diff_x + 1, monster_position_x);
        } else {
            new_pos_x = move_towards(diff_x - 1, monster_position_x);
        }
    } else if diff_x.abs() == 1 && diff_y.abs() == 0 {
        //should be an attack
        new_pos_x = move_towards(diff_x, monster_position_x);
    } else if diff_x.abs() == 0 && diff_y.abs() == 1 {
        //should be an attack
        new_pos_y = move_towards(diff_y, monster_position_y);
    }

    monster_position.velocity_x = new_pos_x - monster_position_x;
    monster_position.velocity_y = new_pos_y - monster_position_y;
}

pub(crate) fn get_nearest_target(
    nearby_players: &HashMap<Uuid, Position>,
    monster_position: &Position,
) -> Uuid {
    let mut closest_distance: u32 = u32::MAX;
    let mut closest_uuid = Uuid::new_v4();
    for (uuid, target_position) in nearby_players {
        let diff_x: u32 =
            (i32_from(monster_position.pos_x) - i32_from(target_position.pos_x)).unsigned_abs();
        let diff_y: u32 =
            (i32_from(monster_position.pos_y) - i32_from(target_position.pos_y)).unsigned_abs();
        if diff_x < closest_distance {
            closest_distance = diff_x;
            closest_uuid = *uuid;
        }
        if diff_y < closest_distance {
            closest_distance = diff_y;
            closest_uuid = *uuid;
        }
    }
    closest_uuid
}

pub(crate) fn check_if_outside_spawn_range(
    spawn_position: &Position,
    current_position: &Position,
) -> bool {
    let diff_x: i32 = i32_from(current_position.pos_x) - i32_from(spawn_position.pos_x);
    let diff_y: i32 = i32_from(current_position.pos_y) - i32_from(spawn_position.pos_y);

    diff_x.abs() > MONSTER_DISTANCE_ACTIVATION || diff_y.abs() > MONSTER_DISTANCE_ACTIVATION
}

fn move_towards(diff: i32, position: i32) -> i32 {
    if diff.is_positive() {
        position - 1
    } else {
        position + 1
    }
}

pub(crate) fn move_randomly(monster_position: &mut Position) {
    let mut velocity_x = 0;
    let mut velocity_y = 0;
    let mut rng = rand::thread_rng();
    let random_pick: u8 = rng.gen_range(1..=4);

    if random_pick == 1 {
        velocity_x = 1;
    } else if random_pick == 2 {
        velocity_x = -1;
    } else if random_pick == 3 {
        velocity_y = 1;
    } else if random_pick == 4 {
        velocity_y = -1;
    }

    monster_position.velocity_x = velocity_x;
    monster_position.velocity_y = velocity_y;
}

#[allow(clippy::similar_names)]
pub(crate) fn get_all_players_nearby(
    player_positions: &PlayersPositions,
    monster_position: &Position,
) -> HashMap<Uuid, Position> {
    let mut nearby_players = HashMap::new();
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let monster_x_range = (monster_position_x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position_y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_y + MONSTER_DISTANCE_ACTIVATION);

    for (player_id, position) in player_positions {
        if monster_position.current_map == position.current_map
            && monster_x_range.contains(&(i32_from(position.pos_x)))
            && monster_y_range.contains(&(i32_from(position.pos_y)))
        {
            debug!("There is a player near a monster");
            nearby_players.insert(*player_id, position.clone());
        }
    }
    nearby_players
}
