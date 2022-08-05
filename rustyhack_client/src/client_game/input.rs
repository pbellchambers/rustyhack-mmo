pub mod commands;

use crate::client_consts::DEFAULT_FG_COLOUR;
use crate::client_game::screens::SidebarState;
use chrono::{DateTime, Local};
use console_engine::{ConsoleEngine, KeyCode};
use crossbeam_channel::Sender;
use crossterm::style::Color;
use laminar::Packet;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::EntityPositionBroadcast;

#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_other_input(
    sender: &Sender<Packet>,
    console: &mut ConsoleEngine,
    system_messages: &mut Vec<(String, Color)>,
    player: &Player,
    all_maps: &AllMaps,
    entity_position_map: &EntityPositionBroadcast,
    server_addr: &str,
    mut sidebar_state: SidebarState,
) -> SidebarState {
    default_input_check(
        console,
        system_messages,
        player,
        all_maps,
        entity_position_map,
        sender,
        server_addr,
    );

    match sidebar_state {
        SidebarState::StatusBar => {
            sidebar_state = sidebar_state_check(console, system_messages, player, sidebar_state);
        }
        SidebarState::DropItemChoice(item_page_index) => {
            sidebar_state = commands::drop::drop_item_choice(
                console,
                player,
                sender,
                server_addr,
                sidebar_state,
                item_page_index,
            );
        }
        SidebarState::StatUpChoice => {
            sidebar_state = commands::stat_up::stat_up_choice(
                console,
                player,
                sender,
                server_addr,
                sidebar_state,
            );
        }
    }

    sidebar_state
}

pub(crate) fn check_for_escape(console: &ConsoleEngine) -> bool {
    console.is_key_pressed(KeyCode::Esc)
}

fn sidebar_state_check(
    console: &mut ConsoleEngine,
    system_messages: &mut Vec<(String, Color)>,
    player: &Player,
    mut sidebar_state: SidebarState,
) -> SidebarState {
    if console.is_key_pressed(KeyCode::Char('d')) {
        info!("Drop item command pressed.");
        if player.inventory.carried.is_empty() {
            let date_time: DateTime<Local> = Local::now();
            let time = date_time.format("[%H:%M:%S] ").to_string();
            info!("No item available to drop.");
            system_messages.push(((time + "No item available to drop."), DEFAULT_FG_COLOUR));
        } else {
            sidebar_state = SidebarState::DropItemChoice(0);
        }
    } else if console.is_key_pressed(KeyCode::Char('u')) && player.stats.stat_points > 0 {
        info!("Stat up command pressed.");
        sidebar_state = SidebarState::StatUpChoice;
    }
    sidebar_state
}

fn default_input_check(
    console: &mut ConsoleEngine,
    system_messages: &mut Vec<(String, Color)>,
    player: &Player,
    all_maps: &AllMaps,
    entity_position_map: &EntityPositionBroadcast,
    sender: &Sender<Packet>,
    server_addr: &str,
) {
    if console.is_key_pressed(KeyCode::Char('l')) {
        commands::look::get_what_player_sees(
            system_messages,
            player,
            all_maps,
            entity_position_map,
        );
    } else if console.is_key_pressed(KeyCode::Char('p')) {
        info!("Pickup command pressed.");
        commands::pickup::send_pickup_request(
            entity_position_map,
            system_messages,
            sender,
            player,
            server_addr,
        );
    }
}
