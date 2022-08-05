use crate::consts::{BASE_HP_TABLE, CUMULATIVE_EXP_TABLE, EXP_LOSS_ON_DEATH_PERCENTAGE};
use crate::game::players::PlayersPositions;
use crate::network_messages::send_message_to_player;
use crossbeam_channel::Sender;
use crossterm::style::Color;
use laminar::Packet;
use legion::world::SubWorld;
use legion::{system, Query};
use rustyhack_lib::ecs::components::{PlayerDetails, Position, Stats};

#[system]
pub(super) fn resolve_player_deaths(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &mut Position, &mut Stats)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (player_details, position, stats) in query.iter_mut(world) {
        if stats.current_hp <= 0.0 {
            let mut exp_loss = 0;
            if stats.exp > 100 {
                exp_loss = (stats.exp * EXP_LOSS_ON_DEATH_PERCENTAGE) / 100;
                stats.exp -= exp_loss;
            }
            stats.current_hp = stats.max_hp;
            stats.in_combat = false;
            stats.update_available = true;
            *position = Position::default();
            position.update_available = true;
            if exp_loss > 0 {
                send_message_to_player(
                    &player_details.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    &("You lost ".to_string() + &exp_loss.to_string() + " exp."),
                    Some(Color::DarkYellow),
                    sender,
                );
            }
            send_message_to_player(
                &player_details.player_name,
                &player_details.client_addr,
                player_details.currently_online,
                "Now respawning at respawn point...",
                None,
                sender,
            );
        }
    }
}

#[system]
pub(super) fn update_player_positions_resource(
    world: &mut SubWorld,
    query: &mut Query<(&PlayerDetails, &Position)>,
    #[resource] players_positions: &mut PlayersPositions,
) {
    players_positions.clear();
    for (player_details, position) in query.iter(world) {
        if player_details.currently_online {
            players_positions.insert(player_details.id, position.clone());
        }
    }
}

#[system]
pub(super) fn level_up(
    world: &mut SubWorld,
    query: &mut Query<(&mut Stats, &PlayerDetails)>,
    #[resource] sender: &Sender<Packet>,
) {
    for (mut stats, player_details) in query.iter_mut(world) {
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
                "You levelled up, 2 new stat points available to spend!",
                Some(Color::Cyan),
                sender,
            );
        }
    }
}

fn calculate_new_stats(stats: &mut Stats) -> &mut Stats {
    // 2 new stat points are given on each level up
    // HP increases by 25
    // HP is increased by Constitution %
    // need to recalculate HP whenever player increases con
    stats.stat_points += 2;
    stats.max_hp =
        (BASE_HP_TABLE[(stats.level - 1) as usize] * (1.0 + (stats.con / 100.0))).round();
    stats
}
