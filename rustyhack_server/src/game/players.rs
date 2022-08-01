use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::{PlayerDetails, Position};
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) type PlayersPositions = HashMap<Uuid, Position>;

pub(crate) fn logout_all_players(world: &mut World) {
    let mut query = <&mut PlayerDetails>::query();
    query.par_for_each_mut(world, |player_details| {
        player_details.currently_online = false;
        player_details.client_addr = "".to_string();
    });
    info!("Marked all players logged out.");
}
