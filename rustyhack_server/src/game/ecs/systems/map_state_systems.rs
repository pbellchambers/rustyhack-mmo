use crate::game::map::state;
use crate::game::map::state::AllMapStates;
use legion::system;
use rustyhack_lib::ecs::components::{
    DisplayDetails, EntityType, Inventory, MonsterDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::monster::Monster;
use rustyhack_lib::ecs::player::Player;

#[system]
pub(crate) fn reset_map_state(#[resource] all_map_states: &mut AllMapStates) {
    debug!("Clearing map state.");
    state::clear_all_entities(all_map_states);
}

#[system(for_each)]
pub(crate) fn add_entities_to_map_state(
    position: &Position,
    display_details: &DisplayDetails,
    monster_details_option: Option<&MonsterDetails>,
    player_details_option: Option<&PlayerDetails>,
    stats: &Stats,
    inventory: &Inventory,

    #[resource] all_map_states: &mut AllMapStates,
) {
    debug!("Adding current entity positions to map state.");
    if let Some(monster_details) = monster_details_option {
        let monster = Monster {
            monster_details: monster_details.clone(),
            display_details: *display_details,
            position: position.clone(),
            stats: *stats,
            inventory: inventory.clone(),
        };
        state::insert_entity_at(
            all_map_states.get_mut(&position.current_map).unwrap(),
            EntityType::Monster(monster),
            position.pos_x,
            position.pos_y,
        );
    }
    if let Some(player_details) = player_details_option {
        let player = Player {
            player_details: player_details.clone(),
            display_details: *display_details,
            position: position.clone(),
            stats: *stats,
            inventory: inventory.clone(),
        };
        state::insert_entity_at(
            all_map_states.get_mut(&position.current_map).unwrap(),
            EntityType::Player(player),
            position.pos_x,
            position.pos_y,
        );
    }
}
