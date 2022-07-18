use legion::system;
use rustyhack_lib::ecs::components::{PlayerDetails, Position};
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
