use crate::game::combat::{CombatAttackerStats, CombatParties};
use crate::game::map_state::{AllMapStates, MapState};
use crate::game::{combat, map_state};
use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{MonsterDetails, PlayerDetails, Position, Stats};
use rustyhack_lib::math_utils::{i32_from, u32_from};
use uuid::Uuid;

#[system]
pub(crate) fn check_for_combat(
    world: &mut SubWorld,
    query: &mut Query<(
        &mut Position,
        Option<&MonsterDetails>,
        Option<&PlayerDetails>,
        &Stats,
    )>,
    #[resource] all_map_states: &AllMapStates,
    #[resource] combat_parties: &mut CombatParties,
    #[resource] combat_attacker_stats: &mut CombatAttackerStats,
) {
    for (position, monster_details_option, player_details_option, stats) in query.iter_mut(world) {
        debug!("Checking for possible combat after velocity updates.");
        if position.velocity_x == 0 && position.velocity_y == 0 {
            //no velocity, no updates
            continue;
        }
        let current_map_states = get_current_map_states(all_map_states, &position.current_map);
        let potential_pos_x = u32_from(i32_from(position.pos_x) + position.velocity_x);
        let potential_pos_y = u32_from(i32_from(position.pos_y) + position.velocity_y);

        let entity_collision_status = map_state::is_colliding_with_entity(
            potential_pos_x,
            potential_pos_y,
            current_map_states,
        );

        let attacker_id = get_attacker_id(player_details_option, monster_details_option);
        debug!("Combat detected, attacker is: {}", attacker_id);

        if entity_collision_status.0 {
            combat_parties.insert(entity_collision_status.1, attacker_id);
            combat_attacker_stats.insert(attacker_id, *stats);
            position.velocity_x = 0;
            position.velocity_y = 0;
        }
    }
}

fn get_attacker_id(
    player_details_option: Option<&PlayerDetails>,
    monster_details_option: Option<&MonsterDetails>,
) -> Uuid {
    if let Some(player_details) = player_details_option {
        player_details.id
    } else if let Some(monster_details) = monster_details_option {
        monster_details.id
    } else {
        error!("Attacker was somehow not a player or monster, returning new Uuid.");
        Uuid::new_v4()
    }
}

fn get_current_map_states<'a>(all_map_states: &'a AllMapStates, map: &String) -> &'a MapState {
    all_map_states.get(map).unwrap_or_else(|| {
        error!("Entity is located on a map that does not exist: {}", &map);
        warn!("Will return the default map, but this may cause problems.");
        all_map_states.get(DEFAULT_MAP).unwrap()
    })
}

#[system]
pub(crate) fn resolve_combat(
    world: &mut SubWorld,
    query: &mut Query<(&mut Stats, Option<&MonsterDetails>, Option<&PlayerDetails>)>,
    #[resource] combat_parties: &mut CombatParties,
    #[resource] combat_attacker_stats: &mut CombatAttackerStats,
) {
    for (stats, monster_details_option, player_details_option) in query.iter_mut(world) {
        debug!("Resolving combat.");
        if let Some(player_details) = player_details_option {
            if combat_parties.contains_key(&player_details.id) {
                let attacker_id = combat_parties.get(&player_details.id).unwrap();
                let damage =
                    combat::resolve_combat(combat_attacker_stats.get(attacker_id).unwrap(), stats);
                stats.current_hp -= damage.round();
                stats.update_available = true;
            }
        } else if let Some(monster_details) = monster_details_option {
            if combat_parties.contains_key(&monster_details.id) {
                let attacker_id = combat_parties.get(&monster_details.id).unwrap();
                let damage =
                    combat::resolve_combat(combat_attacker_stats.get(attacker_id).unwrap(), stats);
                stats.current_hp -= damage.round();
            }
        }
    }
    combat_parties.clear();
    combat_attacker_stats.clear();
}
