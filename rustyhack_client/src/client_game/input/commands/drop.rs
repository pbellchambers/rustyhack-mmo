use crate::client_game::input;
use crate::client_game::screens;
use crate::client_game::screens::SidebarState;
use bincode::serialize;
use console_engine::ConsoleEngine;
use crossbeam_channel::Sender;
use crossterm::event::KeyCode;
use laminar::Packet;
use rustyhack_lib::ecs::item::Item;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::{PlayerRequest, PositionMessage};

pub(crate) fn drop_item_choice(
    console: &mut ConsoleEngine,
    player: &Player,
    sender: &Sender<Packet>,
    server_addr: &str,
    mut sidebar_state: SidebarState,
    item_page_index: u16,
) -> SidebarState {
    if input::check_for_escape(console) {
        info!("Returning to default sidebar window.");
        sidebar_state = SidebarState::StatusBar;
    } else if check_for_back(console, item_page_index) {
        sidebar_state = SidebarState::DropItemChoice(item_page_index - 1);
    } else if check_for_next(console, item_page_index, player.inventory.carried.len()) {
        sidebar_state = SidebarState::DropItemChoice(item_page_index + 1);
    } else if let Some(item_index) = check_for_drop_item_number(console, &player.inventory.carried)
    {
        send_drop_item_request(sender, player, server_addr, item_index, item_page_index);
        sidebar_state = SidebarState::StatusBar;
    }
    sidebar_state
}

fn send_drop_item_request(
    sender: &Sender<Packet>,
    player: &Player,
    server_addr: &str,
    item_index: u8,
    item_page_index: u16,
) {
    let item_index = (item_page_index * 10) + u16::from(item_index);
    let packet = Packet::reliable_ordered(
        server_addr
            .parse()
            .expect("Server address format is invalid."),
        serialize(&PlayerRequest::DropItem((
            item_index,
            PositionMessage {
                player_name: player.player_details.player_name.clone(),
                position: player.position.clone(),
            },
        )))
        .unwrap(),
        Some(13),
    );
    rustyhack_lib::network::send_packet(packet, sender);
    info!(
        "Sent drop item request packet to server for item {}.",
        item_index
    );
}

fn check_for_back(console: &ConsoleEngine, item_page_index: u16) -> bool {
    console.is_key_pressed(KeyCode::Char('b')) && item_page_index > 0
}

fn check_for_next(console: &ConsoleEngine, item_page_index: u16, inventory_size: usize) -> bool {
    console.is_key_pressed(KeyCode::Char('n'))
        && screens::drop_item_choice::can_go_next_page(inventory_size, item_page_index)
}

fn check_for_drop_item_number(
    console: &ConsoleEngine,
    player_carried_inventory: &[Item],
) -> Option<u8> {
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
