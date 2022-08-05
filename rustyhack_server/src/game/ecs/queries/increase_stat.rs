use crate::consts::BASE_HP_TABLE;
use crate::network_messages::send_message_to_player;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::{PlayerDetails, Stats};

pub(crate) fn increase_stat(
    world: &mut World,
    stat: &str,
    player_name: &str,
    sender: &Sender<Packet>,
) {
    let mut query = <(&PlayerDetails, &mut Stats)>::query();
    query.par_for_each_mut(world, |(player_details, stats)| {
        if player_details.player_name == player_name && stats.stat_points > 0 {
            let mut updated_stat = false;
            match stat {
                "Str" => {
                    if stats.str < 100.0 {
                        stats.str += 1.0;
                        stats.stat_points -= 1;
                        updated_stat = true;
                    }
                }
                "Dex" => {
                    if stats.dex < 100.0 {
                        stats.dex += 1.0;
                        stats.stat_points -= 1;
                        updated_stat = true;
                    }
                }
                "Con" => {
                    if stats.con < 100.0 {
                        stats.con += 1.0;
                        stats.stat_points -= 1;
                        stats.max_hp = (BASE_HP_TABLE[(stats.level - 1) as usize]
                            * (1.0 + (stats.con / 100.0)))
                            .round();
                        updated_stat = true;
                    }
                }
                _ => {}
            };

            if updated_stat {
                stats.update_available = true;
                send_message_to_player(
                    &player_details.player_name,
                    &player_details.client_addr,
                    player_details.currently_online,
                    &("Increased ".to_string() + stat + "."),
                    None,
                    sender,
                );
            }
        }
    });
}
