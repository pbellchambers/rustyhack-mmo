use crate::consts::MONSTER_DISTANCE_ACTIVATION;
use crate::game::players::PlayersPositions;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, Query};
use rand::Rng;
use rustyhack_lib::ecs::components::{DisplayDetails, MonsterDetails, Position, Stats};
use rustyhack_lib::math_utils::i32_from;
use std::cmp::Ordering;
use uuid::Uuid;

#[system]
pub(crate) fn resolve_monster_deaths(
    world: &mut SubWorld,
    query: &mut Query<(Entity, &MonsterDetails, &DisplayDetails, &Position, &Stats)>,
    commands: &mut CommandBuffer,
) {
    debug!("Removing dead monsters.");
    for (entity, monster, _display_details, _position, stats) in query.iter(world) {
        if stats.current_hp <= 0.0 {
            debug!("Monster {} {} died.", monster.id, monster.monster_type);
            commands.remove(*entity);
        }
    }
}

#[system]
pub(crate) fn update_monster_velocities(
    world: &mut SubWorld,
    query: &mut Query<(&mut MonsterDetails, &mut Position)>,
    #[resource] players_positions: &PlayersPositions,
) {
    debug!("Updating monster velocities - checking for movement to player positions");
    for (monster, position) in query.iter_mut(world) {
        let mut moving_towards_existing_target = false;

        if let Some(target) = monster.current_target {
            if let Some(current_target_position) = players_positions.get(&target) {
                if is_specific_player_nearby(current_target_position, position) {
                    move_towards_target(position, current_target_position);
                    moving_towards_existing_target = true;
                }
            }
        }

        if !moving_towards_existing_target {
            let nearby_player = is_any_player_nearby(players_positions, position);
            match nearby_player {
                Some((player_id, player_position)) => {
                    monster.is_active = true;
                    monster.current_target = Some(*player_id);
                    move_towards_target(position, player_position);
                }
                None => {
                    debug!("Monster returning to spawn location");
                    monster.is_active = false;
                    monster.current_target = None;
                    move_towards_target(position, &monster.spawn_position);
                }
            }
        }
    }
}

fn move_towards_target(monster_position: &mut Position, target_position: &Position) {
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let diff_x: i32 = monster_position_x - i32_from(target_position.pos_x);
    let diff_y: i32 = monster_position_y - i32_from(target_position.pos_y);
    let mut new_pos_x = monster_position_x;
    let mut new_pos_y = monster_position_y;

    match diff_x.abs().cmp(&diff_y.abs()) {
        Ordering::Greater => new_pos_x = move_towards(diff_x, monster_position_x),
        Ordering::Less => new_pos_y = move_towards(diff_y, monster_position_y),
        Ordering::Equal => {
            let mut rng = rand::thread_rng();
            if rng.gen::<bool>() {
                new_pos_x = move_towards(diff_x, monster_position_x);
            } else {
                new_pos_y = move_towards(diff_y, monster_position_y);
            }
        }
    }
    monster_position.velocity_x = new_pos_x - monster_position_x;
    monster_position.velocity_y = new_pos_y - monster_position_y;
}

fn move_towards(diff: i32, position: i32) -> i32 {
    if diff.is_positive() {
        position - 1
    } else {
        position + 1
    }
}

fn is_any_player_nearby<'a>(
    player_positions: &'a PlayersPositions,
    monster_position: &Position,
) -> Option<(&'a Uuid, &'a Position)> {
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let monster_x_range = (monster_position_x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position_y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_y + MONSTER_DISTANCE_ACTIVATION);
    for (player_id, position) in player_positions {
        if monster_x_range.contains(&(i32_from(position.pos_x)))
            && monster_y_range.contains(&(i32_from(position.pos_y)))
            && monster_position.current_map == position.current_map
        {
            debug!("There is a player near a monster");
            return Some((player_id, position));
        }
    }
    None
}

fn is_specific_player_nearby(
    current_target_position: &Position,
    monster_position: &Position,
) -> bool {
    let monster_position_x = i32_from(monster_position.pos_x);
    let monster_position_y = i32_from(monster_position.pos_y);
    let monster_x_range = (monster_position_x - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_x + MONSTER_DISTANCE_ACTIVATION);
    let monster_y_range = (monster_position_y - MONSTER_DISTANCE_ACTIVATION)
        ..(monster_position_y + MONSTER_DISTANCE_ACTIVATION);

    if monster_x_range.contains(&(i32_from(current_target_position.pos_x)))
        && monster_y_range.contains(&(i32_from(current_target_position.pos_y)))
        && monster_position.current_map == current_target_position.current_map
    {
        return true;
    }
    false
}
