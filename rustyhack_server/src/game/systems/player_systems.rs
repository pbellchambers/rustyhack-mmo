use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::ecs::components::{PlayerDetails, Position, Stats};
use std::collections::HashMap;

#[system(par_for_each)]
pub(crate) fn update_player_input(
    player_details: &PlayerDetails,
    position: &mut Position,
    #[resource] player_updates: &HashMap<String, Position>,
) {
    debug!("Adding player velocity updates to world.");
    for (update_entity_name, update_position) in player_updates {
        if update_entity_name == &player_details.player_name {
            position.velocity_x = update_position.velocity_x;
            position.velocity_y = update_position.velocity_y;
        }
    }
}

#[system]
pub(crate) fn resolve_player_deaths(
    world: &mut SubWorld,
    query: &mut Query<(&mut PlayerDetails, &mut Position, &mut Stats)>,
) {
    for (_player_details, position, stats) in query.iter_mut(world) {
        if stats.current_hp <= 0.0 {
            stats.current_hp = stats.max_hp;
            stats.update_available = true;
            *position = Position::default();
            position.update_available = true;
        }
    }
}
