use crate::game::player_updates::send_message_to_player;
use crate::game::players::PlayersPositions;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::ecs::components::{PlayerDetails, Position, Stats};

#[system]
pub(crate) fn resolve_player_deaths(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &mut Position, &mut Stats)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (player_details, position, stats) in query.iter_mut(world) {
        if stats.current_hp <= 0.0 {
            stats.current_hp = stats.max_hp;
            stats.update_available = true;
            *position = Position::default();
            position.update_available = true;
            send_message_to_player(
                player_details,
                "You died. Now respawning at respawn point...",
                sender,
            );
        }
    }
}

#[system]
pub(crate) fn update_player_positions_resource(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &Position)>,
    #[resource] players_positions: &mut PlayersPositions,
) {
    players_positions.clear();
    for (player_details, position) in query.iter(world) {
        players_positions.insert(player_details.id, position.clone());
    }
}
