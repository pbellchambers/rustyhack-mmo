use crate::game::combat::{Attacker, CombatAttackerStats, CombatParties, Defender};
use crate::game::map_state::{AllMapStates, MapState};
use crate::game::{combat, map_state};
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{MonsterDetails, PlayerDetails, Position, Stats};
use rustyhack_lib::math_utils::{i32_from, u32_from};

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

        let attacker = get_attacker(player_details_option, monster_details_option);

        if entity_collision_status.0 {
            debug!(
                "Combat detected, attacker is: {:?}, defender is: {:?}",
                &attacker, &entity_collision_status.1
            );
            combat_parties.insert(entity_collision_status.1, attacker.clone());
            combat_attacker_stats.insert(attacker.id, *stats);
            position.velocity_x = 0;
            position.velocity_y = 0;
        }
    }
}

fn get_attacker(
    player_details_option: Option<&PlayerDetails>,
    monster_details_option: Option<&MonsterDetails>,
) -> Attacker {
    if let Some(player_details) = player_details_option {
        Attacker {
            id: player_details.id,
            name: player_details.player_name.clone(),
            client_addr: player_details.client_addr.clone(),
            currently_online: player_details.currently_online,
        }
    } else if let Some(monster_details) = monster_details_option {
        Attacker {
            id: monster_details.id,
            name: monster_details.monster_type.clone(),
            client_addr: "".to_string(),
            currently_online: true,
        }
    } else {
        panic!("Error: attacker was somehow not a player or monster.");
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
    #[resource] sender: &Sender<Packet>,
) {
    for (stats, monster_details_option, player_details_option) in query.iter_mut(world) {
        if stats.current_hp <= 0.0 {
            // Skip combat if defender is already dead.
            // This is possible if multiple player updates are processed
            // before the server tick for monsters.
            continue;
        }
        if let Some(player_details) = player_details_option {
            let defender = Defender {
                id: player_details.id,
                name: player_details.player_name.clone(),
                client_addr: player_details.client_addr.clone(),
                currently_online: player_details.currently_online,
            };
            if combat_parties.contains_key(&defender) {
                debug!("Identified player defender in combat resolution loop.");
                let attacker = combat_parties.get(&defender).unwrap();
                let attacker_stats = combat_attacker_stats.get(&attacker.id).unwrap();
                if attacker_stats.current_hp <= 0.0 {
                    // Skip combat if attacker is already dead.
                    // This is possible if multiple player updates are processed
                    // before the server tick for monsters.
                    continue;
                }
                let damage = combat::resolve_combat(attacker_stats, stats);
                let rounded_damage = damage.round();
                stats.current_hp -= rounded_damage;
                combat::send_combat_system_messages_to_players(
                    &defender,
                    attacker,
                    rounded_damage,
                    stats.current_hp,
                    sender,
                );
                stats.update_available = true;
            }
        } else if let Some(monster_details) = monster_details_option {
            let defender = Defender {
                id: monster_details.id,
                name: monster_details.monster_type.clone(),
                client_addr: "".to_string(),
                currently_online: true,
            };
            if combat_parties.contains_key(&defender) {
                debug!("Identified monster defender in combat resolution loop.");
                let attacker = combat_parties.get(&defender).unwrap();
                let attacker_stats = combat_attacker_stats.get(&attacker.id).unwrap();
                if attacker_stats.current_hp <= 0.0 {
                    // Skip combat if attacker is already dead.
                    // This is possible if multiple player updates are processed
                    // before the server tick for monsters.
                    continue;
                }
                let damage = combat::resolve_combat(attacker_stats, stats);
                let rounded_damage = damage.round();
                stats.current_hp -= rounded_damage;
                combat::send_combat_system_messages_to_players(
                    &defender,
                    attacker,
                    rounded_damage,
                    stats.current_hp,
                    sender,
                );
            }
        }
    }
    combat_parties.clear();
    combat_attacker_stats.clear();
}
