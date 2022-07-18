use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::ecs::components::{PlayerDetails, Position, Stats};

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
