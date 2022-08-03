use crate::client_consts::DEFAULT_FG_COLOUR;
use crate::client_game::commands;
use crate::screens::SidebarState;
use chrono::{DateTime, Local};
use console_engine::{ConsoleEngine, KeyCode};
use crossbeam_channel::Sender;
use crossterm::style::Color;
use laminar::Packet;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::EntityPositionBroadcast;

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
            sidebar_state = sidebar_change_check(console, system_messages, player, sidebar_state);
        }
        SidebarState::DropItemChoice => {
            sidebar_state = drop_item_choice(console, player, sender, server_addr, sidebar_state);
        }
        SidebarState::LevelUpChoice => {
            sidebar_state = escape_check(console, sidebar_state);
        }
    }

    sidebar_state
}

fn drop_item_choice(
    console: &mut ConsoleEngine,
    player: &Player,
    sender: &Sender<Packet>,
    server_addr: &str,
    mut sidebar_state: SidebarState,
) -> SidebarState {
    if console.is_key_pressed(KeyCode::Esc) {
        info!("Returning to default sidebar window.");
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('0')) && player.inventory.carried.get(0) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 0);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('1')) && player.inventory.carried.get(1) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 1);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('2')) && player.inventory.carried.get(2) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 2);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('3')) && player.inventory.carried.get(3) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 3);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('4')) && player.inventory.carried.get(4) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 4);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('5')) && player.inventory.carried.get(5) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 5);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('6')) && player.inventory.carried.get(6) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 6);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('7')) && player.inventory.carried.get(7) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 7);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('8')) && player.inventory.carried.get(8) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 8);
        sidebar_state = SidebarState::StatusBar;
    } else if console.is_key_pressed(KeyCode::Char('9')) && player.inventory.carried.get(9) != None
    {
        commands::drop_command::send_drop_item_request(sender, player, server_addr, 9);
        sidebar_state = SidebarState::StatusBar;
    }
    sidebar_state
}

fn escape_check(console: &mut ConsoleEngine, mut sidebar_state: SidebarState) -> SidebarState {
    if console.is_key_pressed(KeyCode::Esc) {
        info!("Returning to default sidebar window.");
        sidebar_state = SidebarState::StatusBar;
    }
    sidebar_state
}

fn sidebar_change_check(
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
            sidebar_state = SidebarState::DropItemChoice;
        }
    } else if console.is_key_pressed(KeyCode::Char('u')) {
        sidebar_state = SidebarState::LevelUpChoice;
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
        commands::look_command::get_what_player_sees(
            system_messages,
            player,
            all_maps,
            entity_position_map,
        );
    } else if console.is_key_pressed(KeyCode::Char('p')) {
        info!("Pickup command pressed.");
        commands::pickup_command::send_pickup_request(
            entity_position_map,
            system_messages,
            sender,
            player,
            server_addr,
        );
    }
}
