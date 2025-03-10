use crate::game::map::state::EntityPositionMap;
use crate::game::map::{state, tiles};
use legion::systems::CommandBuffer;
use legion::{Entity, maybe_changed, system};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::consts::DEAD_MAP;
use rustyhack_lib::ecs::components::{
    Dead, DisplayDetails, ItemDetails, MonsterDetails, PlayerDetails, Position,
};
use rustyhack_lib::ecs::item::{Item, get_item_name};
use rustyhack_lib::utils::math::{i32_from, u32_from};

#[system(par_for_each)]
#[filter(maybe_changed::<Position>())]
pub(super) fn check_for_tile_collision(position: &mut Position, #[resource] all_maps: &AllMaps) {
    //no velocity, no updates
    if position.velocity_x != 0 || position.velocity_y != 0 {
        let current_map = state::get_current_map(all_maps, &position.current_map);
        let potential_pos_x = u32_from(i32_from(position.pos_x) + position.velocity_x);
        let potential_pos_y = u32_from(i32_from(position.pos_y) + position.velocity_y);

        if tiles::entity_is_colliding_with_tile(
            current_map.get_tile_at(potential_pos_y, potential_pos_x),
        ) {
            debug!("Entity colliding with tile, setting velocity to 0.");
            position.velocity_x = 0;
            position.velocity_y = 0;
        } else {
            debug!("Entity not colliding with tile, continuing to combat check.");
        }
    }
}

#[system(par_for_each)]
#[filter(maybe_changed::<Position>())]
pub(super) fn update_entities_position(position: &mut Position) {
    //no velocity, no updates
    if position.velocity_x != 0 || position.velocity_y != 0 {
        position.pos_x = u32_from(i32_from(position.pos_x) + position.velocity_x);
        position.pos_y = u32_from(i32_from(position.pos_y) + position.velocity_y);
        position.velocity_x = 0;
        position.velocity_y = 0;
        position.update_available = true;
    }
}

#[system(for_each)]
pub(super) fn collate_all_player_positions(
    player_details: &PlayerDetails,
    position: &Position,
    display_details: &DisplayDetails,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    if player_details.currently_online {
        entity_position_map.insert(
            player_details.id,
            (
                position.clone(),
                *display_details,
                player_details.player_name.clone(),
            ),
        );
    }
}

#[system(for_each)]
pub(super) fn collate_all_monster_positions(
    monster_details: &MonsterDetails,
    position: &Position,
    display_details: &DisplayDetails,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    debug!("Getting all monster positions");
    entity_position_map.insert(
        monster_details.id,
        (
            position.clone(),
            *display_details,
            monster_details.monster_type.clone(),
        ),
    );
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[system(for_each)]
pub(super) fn collate_all_item_positions(
    entity: &Entity,
    item_details: &ItemDetails,
    item: &Item,
    position: &Position,
    display_details: &DisplayDetails,
    commands: &mut CommandBuffer,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    debug!("Getting all item positions");
    if item_details.has_been_picked_up {
        let dead_map = position.current_map.clone() + DEAD_MAP;
        let dead_position = Position {
            current_map: dead_map,
            ..Dead::dead()
        };
        debug!(
            "Removing item id {} to dead map: {:?}",
            item_details.id, dead_position
        );
        entity_position_map.insert(
            item_details.id,
            (
                dead_position,
                DisplayDetails::dead(),
                "picked_up_item".to_string(),
            ),
        );
        debug!("Removing item id {} from world.", item_details.id);
        commands.remove(*entity);
    } else {
        let item_name: String = get_item_name(item);
        entity_position_map.insert(
            item_details.id,
            (position.clone(), *display_details, item_name),
        );
    }
}
