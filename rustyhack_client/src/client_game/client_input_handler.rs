use crate::client_consts::DEFAULT_FG_COLOUR;
use crate::client_game::commands;
use crate::screens;
use crate::screens::SidebarState;
use chrono::{DateTime, Local};
use console_engine::{ConsoleEngine, KeyCode};
use crossbeam_channel::Sender;
use crossterm::style::Color;
use laminar::Packet;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::item::Item;
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
        SidebarState::DropItemChoice(item_page_index) => {
            sidebar_state = drop_item_choice(
                console,
                player,
                sender,
                server_addr,
                sidebar_state,
                item_page_index,
            );
        }
        SidebarState::LevelUpChoice => {
            if check_for_escape(console) {
                info!("Returning to default sidebar window.");
                sidebar_state = SidebarState::StatusBar;
            }
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
    item_page_index: u16,
) -> SidebarState {
    if check_for_escape(console) {
        info!("Returning to default sidebar window.");
        sidebar_state = SidebarState::StatusBar;
    } else if check_for_back(console, item_page_index) {
        sidebar_state = SidebarState::DropItemChoice(item_page_index - 1);
    } else if check_for_next(console, item_page_index, player.inventory.carried.len()) {
        sidebar_state = SidebarState::DropItemChoice(item_page_index + 1);
    } else if let Some(item_index) = check_for_number(console, &player.inventory.carried) {
        commands::drop_command::send_drop_item_request(
            sender,
            player,
            server_addr,
            item_index,
            item_page_index,
        );
        sidebar_state = SidebarState::StatusBar;
    }
    sidebar_state
}

fn check_for_escape(console: &ConsoleEngine) -> bool {
    console.is_key_pressed(KeyCode::Esc)
}

fn check_for_back(console: &ConsoleEngine, item_page_index: u16) -> bool {
    console.is_key_pressed(KeyCode::Char('b')) && item_page_index > 0
}

fn check_for_next(console: &ConsoleEngine, item_page_index: u16, inventory_size: usize) -> bool {
    console.is_key_pressed(KeyCode::Char('n'))
        && screens::drop_item_choice::can_go_next_page(inventory_size, item_page_index)
}

fn check_for_number(console: &ConsoleEngine, player_carried_inventory: &[Item]) -> Option<u8> {
    let key_pressed: Option<u8>;

    if console.is_key_pressed(KeyCode::Char('0')) {
        key_pressed = Some(0);
    } else if console.is_key_pressed(KeyCode::Char('1')) {
        key_pressed = Some(1);
    } else if console.is_key_pressed(KeyCode::Char('2')) {
        key_pressed = Some(2);
    } else if console.is_key_pressed(KeyCode::Char('3')) {
        key_pressed = Some(3);
    } else if console.is_key_pressed(KeyCode::Char('4')) {
        key_pressed = Some(4);
    } else if console.is_key_pressed(KeyCode::Char('5')) {
        key_pressed = Some(5);
    } else if console.is_key_pressed(KeyCode::Char('6')) {
        key_pressed = Some(6);
    } else if console.is_key_pressed(KeyCode::Char('7')) {
        key_pressed = Some(7);
    } else if console.is_key_pressed(KeyCode::Char('8')) {
        key_pressed = Some(8);
    } else if console.is_key_pressed(KeyCode::Char('9')) {
        key_pressed = Some(9);
    } else {
        key_pressed = None;
    }

    if key_pressed != None && player_carried_inventory.get(key_pressed.unwrap() as usize) != None {
        key_pressed
    } else {
        None
    }
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
            sidebar_state = SidebarState::DropItemChoice(0);
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
