use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::client_consts::{GAME_TITLE, INITIAL_CONSOLE_HEIGHT, INITIAL_CONSOLE_WIDTH, TARGET_FPS};
use rustyhack_lib::message_handler::messages::EntityUpdates;

use crate::networking::client_message_handler;
use crate::screens::draw_screens;

mod client_input_handler;
mod client_logout;
mod client_map_handler;
mod client_updates_handler;
mod commands;
mod new_player;

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
    client_message_handler::spawn_message_handler_thread(receiver, player_update_sender);

    //get basic data from server needed to start client_game
    let all_maps =
        client_map_handler::request_all_maps_data(sender, server_addr, &player_update_receiver);

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

    let mut entity_updates = EntityUpdates {
        position_updates: HashMap::new(),
        display_details: HashMap::new(),
        monster_type_map: HashMap::new(),
    };

    let mut system_messages: Vec<String> = vec![];

    info!("Starting client_game loop");
    loop {
        //wait for target fps tick time to continue
        console.wait_frame();

        debug!("About to send player velocity update.");
        client_updates_handler::send_player_updates(sender, &console, &mut player, server_addr);

        debug!("About to wait for entity updates from server.");
        entity_updates = client_updates_handler::check_for_received_server_messages(
            &player_update_receiver,
            &mut player,
            entity_updates,
            &mut system_messages,
        );

        client_input_handler::handle_other_input(
            &mut console,
            &mut system_messages,
            &player,
            &all_maps,
            &entity_updates,
        );

        //clear, update and redraw the screens
        draw_screens(
            &mut console,
            &all_maps,
            &player,
            &entity_updates,
            &system_messages,
        );

        //check if we should quit
        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            client_logout::send_logout_notification(sender, player, server_addr);
            //sleep for a second just to make sure the logout notification is sent
            thread::sleep(Duration::from_millis(250));
            break;
        }
    }
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}
