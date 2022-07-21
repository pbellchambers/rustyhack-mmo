use crate::consts::{BASE_HP_TABLE, CUMULATIVE_EXP_TABLE};
use crate::game::player_updates::send_message_to_player;
use crate::game::players::PlayersPositions;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Query};
use rand::Rng;
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
                &player_details.player_name,
                &player_details.client_addr,
                player_details.currently_online,
                "Now respawning at respawn point...",
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

#[system]
pub(crate) fn level_up(
    world: &mut SubWorld,
    query: &mut Query<(&mut Stats, Option<&PlayerDetails>)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (mut stats, player_details_option) in query.iter_mut(world) {
        if let Some(player_details) = player_details_option {
            if stats.exp >= stats.exp_next && stats.level < 100 {
                info!(
                    "Player {} levelled up from {} to {}!",
                    player_details.player_name,
                    stats.level,
                    stats.level + 1
                );
                stats.level += 1;
                if stats.level >= 100 {
                    stats.exp_next = 0;
                } else {
                    stats.exp_next = CUMULATIVE_EXP_TABLE[(stats.level - 1) as usize];
                }
                stats = calculate_new_stats(stats);
                stats.update_available = true;
                send_message_to_player(
                    &player_details.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    "You levelled up!",
                    sender,
                );
            }
        }
    }
}

fn calculate_new_stats(stats: &mut Stats) -> &mut Stats {
    // 2 new stat points are given on each level up
    // HP increases by 25
    // HP is increased by Constitution %
    //todo make stat selection a player choice rather than randomly assigning 2 stat upgrades here
    let mut stat_upgrades = 2;
    while stat_upgrades > 0 {
        let mut rng = rand::thread_rng();
        let random_choice = rng.gen_range(1..=3);
        if random_choice == 1 {
            stats.str += 1.0;
        } else if random_choice == 2 {
            stats.dex += 1.0;
        } else {
            stats.con += 1.0;
        }
        stat_upgrades -= 1;
    }
    stats.max_hp = BASE_HP_TABLE[(stats.level - 1) as usize] * (1.0 + (stats.con / 100.0));
    stats
}
