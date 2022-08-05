use crate::consts::{GOLD_LOSS_ON_PVP_DEATH_PERCENTAGE, MONSTER_EXP_MULTIPLICATION_FACTOR};
use crate::game::combat::{Attacker, CombatAttackerStats, CombatParties, Defender};
use crate::game::map_state::{AllMapStates, MapState};
use crate::game::{combat, map_state};
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::consts::DEFAULT_MAP;
use rustyhack_lib::ecs::components::{
    DisplayDetails, EntityType, Inventory, MonsterDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::monster::Monster;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::utils::math::{i32_from, u32_from};
use uuid::Uuid;

#[system]
#[allow(clippy::type_complexity)]
pub(crate) fn check_for_combat(
    world: &mut SubWorld,
    query: &mut Query<(
        &mut Position,
        Option<&MonsterDetails>,
        Option<&PlayerDetails>,
        &DisplayDetails,
        &mut Stats,
        &Inventory,
    )>,
    #[resource] all_map_states: &mut AllMapStates,
    #[resource] combat_parties: &mut CombatParties,
    #[resource] combat_attacker_stats: &mut CombatAttackerStats,
) {
    for (
        position,
        monster_details_option,
        player_details_option,
        display_details,
        stats,
        inventory,
    ) in query.iter_mut(world)
    {
        debug!("Checking for possible combat after velocity updates.");
        if (position.velocity_x == 0 && position.velocity_y == 0) || stats.current_hp <= 0.0 {
            //either not moving, or already dead, no combat
            stats.in_combat = false;
            continue;
        }
        let current_map_state = get_current_map_states(all_map_states, &position.current_map);
        let potential_pos_x = u32_from(i32_from(position.pos_x) + position.velocity_x);
        let potential_pos_y = u32_from(i32_from(position.pos_y) + position.velocity_y);

        let entity_collision_status = map_state::is_colliding_with_entity(
            potential_pos_x,
            potential_pos_y,
            current_map_state,
        );

        let attacker = get_attacker(player_details_option, monster_details_option);

        if entity_collision_status.0 {
            //combat detected
            if !attacker.is_player && !entity_collision_status.1.is_player {
                debug!("Preventing possible monster combat.");
                //monsters shouldn't attack other monsters, stop combat and movement
                position.velocity_x = 0;
                position.velocity_y = 0;
                stats.in_combat = false;
            } else {
                debug!(
                    "Combat detected, attacker is: {:?}, defender is: {:?}",
                    &attacker, &entity_collision_status.1
                );
                combat_parties.insert(attacker.clone(), entity_collision_status.1);
                combat_attacker_stats.insert(attacker.id, (*stats, inventory.clone()));
                position.velocity_x = 0;
                position.velocity_y = 0;
                stats.in_combat = true;
            }
        } else if attacker.is_player {
            //no combat detected - update map state for movement
            let player = Player {
                player_details: player_details_option.unwrap().clone(),
                display_details: *display_details,
                position: position.clone(),
                stats: *stats,
                inventory: inventory.clone(),
            };
            map_state::insert_entity_at(
                all_map_states.get_mut(&position.current_map).unwrap(),
                EntityType::Player(player.clone()),
                potential_pos_x,
                potential_pos_y,
            );
            map_state::remove_entity_at(
                all_map_states.get_mut(&position.current_map).unwrap(),
                &EntityType::Player(player),
                position.pos_x,
                position.pos_y,
            );
            //player has moved, therefore no longer attacking
            if combat_parties.contains_key(&attacker) {
                combat_parties.remove(&attacker);
            }
            stats.in_combat = false;
        } else if !attacker.is_player {
            //no combat detected - update map state for movement
            let monster = Monster {
                monster_details: monster_details_option.unwrap().clone(),
                display_details: *display_details,
                position: position.clone(),
                stats: *stats,
                inventory: inventory.clone(),
            };
            map_state::insert_entity_at(
                all_map_states.get_mut(&position.current_map).unwrap(),
                EntityType::Monster(monster.clone()),
                potential_pos_x,
                potential_pos_y,
            );
            map_state::remove_entity_at(
                all_map_states.get_mut(&position.current_map).unwrap(),
                &EntityType::Monster(monster),
                position.pos_x,
                position.pos_y,
            );
            //monster has moved, therefore no longer attacking
            if combat_parties.contains_key(&attacker) {
                combat_parties.remove(&attacker);
            }
            stats.in_combat = false;
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
            is_player: true,
        }
    } else if let Some(monster_details) = monster_details_option {
        Attacker {
            id: monster_details.id,
            name: monster_details.monster_type.clone(),
            client_addr: "".to_string(),
            currently_online: true,
            is_player: false,
        }
    } else {
        panic!("Error: attacker was somehow not a player or monster.");
    }
}

fn get_current_map_states<'a>(
    all_map_states: &'a mut AllMapStates,
    map: &String,
) -> &'a mut MapState {
    return if all_map_states.contains_key(map) {
        all_map_states.get_mut(map).unwrap()
    } else {
        warn!("Tried to get map state for map that doesn't exist.");
        warn!("Will return default map, but things might break.");
        all_map_states.get_mut(DEFAULT_MAP).unwrap()
    };
}

#[system]
pub(crate) fn resolve_combat(
    world: &mut SubWorld,
    query: &mut Query<(
        &mut Stats,
        Option<&mut MonsterDetails>,
        Option<&PlayerDetails>,
        &mut Inventory,
    )>,
    #[resource] combat_parties: &mut CombatParties,
    #[resource] combat_attacker_stats: &mut CombatAttackerStats,
    #[resource] sender: &Sender<Packet>,
) {
    for (defender_stats, monster_details_option, player_details_option, defender_inventory) in
        query.iter_mut(world)
    {
        if defender_stats.current_hp <= 0.0 {
            // Skip combat if defender is already dead.
            // This is possible if multiple player updates are processed
            // before the server tick for monsters.
            continue;
        }
        let mut defender_is_monster = false;
        let mut monster_target = None;
        let mut defender: Defender = Defender::default();
        if let Some(player_details) = player_details_option {
            //player is the defender
            defender = Defender {
                id: player_details.id,
                name: player_details.player_name.clone(),
                client_addr: player_details.client_addr.clone(),
                currently_online: player_details.currently_online,
                is_player: true,
            };
        } else if let Some(monster_details) = &monster_details_option {
            //monster is the defender
            defender = Defender {
                id: monster_details.id,
                name: monster_details.monster_type.clone(),
                client_addr: "".to_string(),
                currently_online: true,
                is_player: false,
            };
            monster_target = monster_details.current_target;
            defender_is_monster = true;
        }
        for (combat_parties_attacker, combat_parties_defender) in combat_parties.iter() {
            if combat_parties_defender == &defender {
                //combat detected
                defender_stats.in_combat = true;
                let attacker = combat_parties_attacker;
                let (mut attacker_stats, mut attacker_inventory) =
                    combat_attacker_stats.get(&attacker.id).unwrap().clone();
                if attacker_stats.current_hp <= 0.0 {
                    // Skip combat if attacker is already dead.
                    // This is possible if combat is happening simultaneously.
                    continue;
                }
                let damage = combat::resolve_combat(
                    &attacker_stats,
                    &attacker_inventory,
                    defender_stats,
                    defender_inventory,
                )
                .round();
                apply_damage(defender_stats, damage);
                if combat_attacker_stats.contains_key(&defender.id) {
                    combat_attacker_stats
                        .get_mut(&defender.id)
                        .unwrap()
                        .0
                        .current_hp = defender_stats.current_hp;
                }
                let (exp_gain, gold_gain) = check_and_apply_gains(
                    defender_is_monster,
                    combat_attacker_stats,
                    &attacker.id,
                    &mut attacker_stats,
                    &mut attacker_inventory,
                    defender_stats,
                    defender_inventory,
                );
                if defender_is_monster {
                    //set current monster target as attacker
                    monster_target = Some(attacker.id);
                }
                combat::send_combat_updates_to_players(
                    &defender,
                    attacker,
                    damage,
                    defender_stats.current_hp,
                    exp_gain,
                    gold_gain,
                    sender,
                );

                if let Some(_player_details) = player_details_option {
                    //only set flag for players
                    defender_stats.update_available = true;
                }
            }
        }
        if defender_is_monster {
            //set current_target to most recent attacker
            monster_details_option.unwrap().current_target = monster_target;
        }
    }
    combat_parties.clear();
}

fn check_and_apply_gains(
    defender_is_monster: bool,
    combat_attacker_stats: &mut CombatAttackerStats,
    attacker_id: &Uuid,
    attacker_stats: &mut Stats,
    attacker_inventory: &mut Inventory,
    defender_stats: &Stats,
    defender_inventory: &mut Inventory,
) -> (u32, u32) {
    if defender_stats.current_hp <= 0.0 {
        //calculate xp to be gained
        let mut exp_gain = 0;
        if defender_is_monster {
            exp_gain = defender_stats.level * MONSTER_EXP_MULTIPLICATION_FACTOR;
            attacker_stats.exp += exp_gain;
        }

        //calculate gold to be gained
        let mut gold_gain = 0;
        if defender_is_monster {
            gold_gain = defender_inventory.gold;
        } else if defender_inventory.gold > 100 {
            gold_gain = (defender_inventory.gold * GOLD_LOSS_ON_PVP_DEATH_PERCENTAGE) / 100;
            defender_inventory.gold -= gold_gain;
            defender_inventory.update_available = true;
        }
        attacker_inventory.gold += gold_gain;
        if exp_gain > 0 || gold_gain > 0 {
            attacker_stats.update_available = true;
        }
        combat_attacker_stats.insert(*attacker_id, (*attacker_stats, attacker_inventory.clone()));

        return (exp_gain, gold_gain);
    }
    (0, 0)
}

#[system]
pub(crate) fn apply_combat_gains(
    world: &mut SubWorld,
    query: &mut Query<(&mut Stats, &mut Inventory, Option<&PlayerDetails>)>,
    #[resource] combat_attacker_stats: &mut CombatAttackerStats,
) {
    for (stats, inventory, player_details_option) in query.iter_mut(world) {
        if let Some(player_details) = player_details_option {
            if combat_attacker_stats.contains_key(&player_details.id)
                && combat_attacker_stats
                    .get(&player_details.id)
                    .unwrap()
                    .0
                    .update_available
            {
                stats.exp = combat_attacker_stats.get(&player_details.id).unwrap().0.exp;
                inventory.gold = combat_attacker_stats
                    .get(&player_details.id)
                    .unwrap()
                    .1
                    .gold;
                stats.update_available = true;
                inventory.update_available = true;
            }
        }
    }
    combat_attacker_stats.clear();
}

fn apply_damage(stats: &mut Stats, damage: f32) {
    stats.current_hp -= damage;
}
