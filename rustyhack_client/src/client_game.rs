use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, Sender};
use crossterm::style::Color;
use laminar::{Packet, SocketEvent};
use rustyhack_lib::network::packets::EntityPositionBroadcast;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use crate::client_consts::{
    CLIENT_CLEANUP_TICK, GAME_TITLE, INITIAL_CONSOLE_HEIGHT, INITIAL_CONSOLE_WIDTH, TARGET_FPS,
};
use crate::client_game::screens::{draw_screens, SidebarState};
use input::commands::movement;

use crate::client_network_messages::{
    client_network_packet_receiver, map_downloader, new_player, player_logout,
};

mod client_updates_handler;
mod input;
mod screens;

pub(crate) fn run(
    sender: &Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    server_addr: &str,
    client_addr: &str,
    player_name: &str,
) {
    //setup message handling threads
    let (player_update_sender, player_update_receiver) = crossbeam_channel::unbounded();
    debug!("Spawned thread channels.");
    client_network_packet_receiver::spawn_network_packet_receiver_thread(
        receiver,
        player_update_sender,
    );

    //get basic data from server needed to start client_game
    let all_maps =
        map_downloader::request_all_maps_data(sender, server_addr, &player_update_receiver);

    //create player
    let mut player = new_player::send_new_player_request(
        sender,
        player_name,
        server_addr,
        client_addr,
        &player_update_receiver,
    );

    //initialise console engine
    let mut console =
        ConsoleEngine::init(INITIAL_CONSOLE_WIDTH, INITIAL_CONSOLE_HEIGHT, TARGET_FPS).unwrap();
    console.set_title(&(GAME_TITLE.to_string() + " - v" + env!("CARGO_PKG_VERSION")));
    info!("Initialised console engine.");

    let mut entity_position_map: EntityPositionBroadcast = HashMap::new();
    let mut system_messages: Vec<(String, Color)> = vec![];
    let mut sidebar_state = SidebarState::StatusBar;

    info!("Starting client_game loop");
    let mut client_cleanup_tick_time = Instant::now();
    loop {
        //wait for target fps tick time to continue
        console.wait_frame();

        debug!("About to send player velocity update.");
        movement::send_player_updates(sender, &console, &mut player, server_addr);

        debug!("About to wait for entity updates from server.");
        client_updates_handler::handle_received_server_messages(
            &player_update_receiver,
            &mut player,
            &mut entity_position_map,
            &mut system_messages,
        );

        if client_cleanup_tick_time.elapsed() > CLIENT_CLEANUP_TICK {
            //no need to do this often, dead entities are already not displayed
            client_updates_handler::cleanup_dead_entities(&player, &mut entity_position_map);
            client_cleanup_tick_time = Instant::now();
        }

        sidebar_state = input::handle_other_input(
            sender,
            &mut console,
            &mut system_messages,
            &player,
            &all_maps,
            &entity_position_map,
            server_addr,
            sidebar_state,
        );

        //clear, update and redraw the screens
        draw_screens(
            &mut console,
            &all_maps,
            &player,
            &entity_position_map,
            &system_messages,
            sidebar_state,
        );

        //check if we should quit
        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            player_logout::send_logout_notification(sender, player, server_addr);
            //sleep for a reasonable delay to make sure the logout notification is sent
            thread::sleep(Duration::from_millis(250));
            break;
        }
    }
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}
