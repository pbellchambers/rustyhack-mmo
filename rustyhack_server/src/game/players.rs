use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::{PlayerDetails, Position};
use std::collections::HashMap;

pub(crate) fn get_current_player_positions(world: &mut World) -> HashMap<String, Position> {
    let mut players_positions: HashMap<String, Position> = HashMap::new();
    let mut query = <(&Position, &PlayerDetails)>::query();
    for (position, player) in query.iter(world) {
        //only add online players
        if player.currently_online {
            players_positions.insert(player.player_name.clone(), position.clone());
        }
    }
    players_positions
}
