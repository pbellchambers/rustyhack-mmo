mod combat_systems;
mod common_entity_systems;
mod map_state_systems;
mod monster_systems;
mod player_systems;

use legion::Schedule;

pub(crate) fn build_map_state_update_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(map_state_systems::reset_map_state_system())
        .add_system(map_state_systems::add_entities_to_map_state_system())
        .build();
    info!("Built map state update system schedule.");
    schedule
}

pub(crate) fn build_player_update_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(combat_systems::check_for_combat_system())
        .add_system(combat_systems::resolve_combat_system())
        .add_system(common_entity_systems::update_entities_position_system())
        .add_system(player_systems::resolve_player_deaths_system())
        .add_system(player_systems::update_player_positions_resource_system())
        .build();
    info!("Built player update system schedule.");
    schedule
}

pub(crate) fn build_monster_update_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(monster_systems::resolve_monster_deaths_system())
        .add_system(monster_systems::update_monster_velocities_system())
        .add_system(common_entity_systems::update_entities_position_system())
        .build();
    info!("Built monster update system schedule.");
    schedule
}
