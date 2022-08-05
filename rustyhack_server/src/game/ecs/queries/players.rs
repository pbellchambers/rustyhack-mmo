use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::{PlayerDetails, Position};
use rustyhack_lib::network::packets::PositionMessage;
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

pub fn set_player_velocity(world: &mut World, position_message: &PositionMessage) {
    let mut query = <(&mut PlayerDetails, &mut Position)>::query();
    for (player_details, position) in query.iter_mut(world) {
        if player_details.player_name == position_message.player_name {
            position.velocity_x = position_message.position.velocity_x;
            position.velocity_y = position_message.position.velocity_y;
        }
    }
}
