use uuid::Uuid;

pub(crate) fn resolve_player_combat(
    entity: String,
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
) {
    //todo
    info!(
        "Some pvp combat should occur with {} at: {} {} {} {}",
        entity, position_x, position_y, velocity_x, velocity_y
    );
}

pub(crate) fn resolve_monster_combat(
    entity: Uuid,
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
) {
    //todo
    info!(
        "Some pve combat should occur with {} at: {} {} {} {}",
        entity, position_x, position_y, velocity_x, velocity_y
    );
}
