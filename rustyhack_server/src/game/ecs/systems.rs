mod combat_systems;
mod map_state_systems;
mod monster_systems;
mod network_messages_systems;
pub(crate) mod player_systems;
mod position_systems;
mod regen_systems;

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
        .add_system(position_systems::check_for_tile_collision_system())
        .add_system(combat_systems::check_for_combat_system())
        .add_system(player_systems::resolve_player_deaths_system())
        .add_system(position_systems::update_entities_position_system())
        .add_system(player_systems::update_player_positions_resource_system())
        .build();
    info!("Built normal player update system schedule.");
    schedule
}

pub(crate) fn build_server_tick_update_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(monster_systems::update_monster_velocities_system())
        .add_system(position_systems::check_for_tile_collision_system())
        .add_system(combat_systems::check_for_combat_system())
        .add_system(combat_systems::resolve_combat_system())
        .flush()
        .add_system(combat_systems::apply_combat_gains_system())
        .add_system(player_systems::level_up_system())
        .add_system(player_systems::resolve_player_deaths_system())
        .add_system(monster_systems::resolve_monster_deaths_system())
        .add_system(monster_systems::spawn_monsters_system())
        .add_system(position_systems::update_entities_position_system())
        .add_system(player_systems::update_player_positions_resource_system())
        .build();
    info!("Built server tick update system schedule.");
    schedule
}

pub(crate) fn build_health_regen_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(regen_systems::apply_health_regen_system())
        .build();
    info!("Built health regen schedule.");
    schedule
}

pub(crate) fn send_network_messages_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(network_messages_systems::send_player_position_updates_system())
        .add_system(network_messages_systems::send_player_stats_updates_system())
        .add_system(network_messages_systems::send_player_inventory_updates_system())
        .build();
    info!("Built network messages schedule.");
    schedule
}

pub(crate) fn network_broadcast_schedule() -> Schedule {
    let schedule = Schedule::builder()
        .add_system(position_systems::collate_all_player_positions_system())
        .add_system(position_systems::collate_all_monster_positions_system())
        .add_system(position_systems::collate_all_item_positions_system())
        .add_system(network_messages_systems::broadcast_entity_updates_system())
        .build();
    info!("Built network broadcast schedule.");
    schedule
}
