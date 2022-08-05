use crate::consts::{BASE_HP_TABLE, CUMULATIVE_EXP_TABLE, EXP_LOSS_ON_DEATH_PERCENTAGE};
use crate::game::players::PlayersPositions;
use crate::network_messages::send_message_to_player;
use crossbeam_channel::Sender;
use crossterm::style::Color;
use laminar::Packet;
use legion::{maybe_changed, system};
use rustyhack_lib::ecs::components::{PlayerDetails, Position, Stats};

#[system(par_for_each)]
#[filter(maybe_changed::<Stats>())]
pub(super) fn resolve_player_deaths(
    player_details: &PlayerDetails,
    position: &mut Position,
    stats: &mut Stats,
    #[resource] sender: &Sender<Packet>,
) {
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

#[system]
pub(super) fn clear_player_positions_resource(
    #[resource] players_positions: &mut PlayersPositions,
) {
    players_positions.clear();
}

#[system(for_each)]
pub(super) fn update_player_positions_resource(
    player_details: &PlayerDetails,
    position: &Position,
    #[resource] players_positions: &mut PlayersPositions,
) {
    if player_details.currently_online {
        players_positions.insert(player_details.id, position.clone());
    }
}

#[system(par_for_each)]
#[filter(maybe_changed::<Stats>())]
pub(super) fn level_up(
    stats: &mut Stats,
    player_details: &PlayerDetails,
    #[resource] sender: &Sender<Packet>,
) {
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
        calculate_new_stats(stats);
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

fn calculate_new_stats(stats: &mut Stats) {
    // 2 new stat points are given on each level up
    // HP increases by 25
    // HP is increased by Constitution %
    // need to recalculate HP whenever player increases con
    stats.stat_points += 2;
    stats.max_hp =
        (BASE_HP_TABLE[(stats.level - 1) as usize] * (1.0 + (stats.con / 100.0))).round();
}
