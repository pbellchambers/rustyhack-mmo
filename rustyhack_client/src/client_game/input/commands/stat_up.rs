use crate::client_game::input;
use crate::client_game::screens::SidebarState;
use bincode::serialize;
use console_engine::ConsoleEngine;
use crossbeam_channel::Sender;
use crossterm::event::KeyCode;
use laminar::Packet;
use rustyhack_lib::ecs::components::Stats;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::PlayerRequest;

pub(crate) fn stat_up_choice(
    console: &mut ConsoleEngine,
    player: &Player,
    sender: &Sender<Packet>,
    server_addr: &str,
    mut sidebar_state: SidebarState,
) -> SidebarState {
    if input::check_for_escape(console) {
        info!("Returning to default sidebar window.");
        sidebar_state = SidebarState::StatusBar;
    } else if let Some(stat) = check_for_stat_up(console, player.stats) {
        send_stat_up_request(sender, player, server_addr, stat);
        sidebar_state = SidebarState::StatusBar;
    }
    sidebar_state
}

fn send_stat_up_request(sender: &Sender<Packet>, player: &Player, server_addr: &str, stat: &str) {
    let packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::StatUp((
            stat.to_string(),
            player.player_details.player_name.clone(),
        )))
        .unwrap(),
        Some(13),
    );
    rustyhack_lib::network::send_packet(packet, sender);
    info!("Sent stat up request packet to server for {}.", stat);
}

fn check_for_stat_up(console: &ConsoleEngine, stats: Stats) -> Option<&str> {
    let stat: Option<&str>;

    if console.is_key_pressed(KeyCode::Char('1')) && stats.str < 100.0 {
        stat = Some("Str");
    } else if console.is_key_pressed(KeyCode::Char('2')) && stats.dex < 100.0 {
        stat = Some("Dex");
    } else if console.is_key_pressed(KeyCode::Char('3')) && stats.con < 100.0 {
        stat = Some("Con");
    } else {
        stat = None;
    }

    if stat != None && stats.stat_points > 0 {
        stat
    } else {
        None
    }
}
