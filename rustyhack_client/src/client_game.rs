use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use rustyhack_lib::message_handler::messages::EntityUpdates;

use crate::client_consts::{CONSOLE_HEIGHT, CONSOLE_WIDTH, GAME_TITLE, TARGET_FPS};
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
    let (entity_update_sender, entity_update_receiver) = crossbeam_channel::unbounded();
    debug!("Spawned thread channels.");
    client_message_handler::spawn_message_handler_thread(
        receiver,
        player_update_sender,
        entity_update_sender,
    );

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
    let mut console = ConsoleEngine::init(
        CONSOLE_WIDTH as u32,
        CONSOLE_HEIGHT as u32,
        TARGET_FPS as u32,
    )
    .unwrap();
    console.set_title(GAME_TITLE);
    info!("Initialised console engine.");

    let mut other_entities = EntityUpdates {
        position_updates: HashMap::new(),
        display_details: HashMap::new(),
        monster_type_map: HashMap::new(),
    };

    let mut status_messages: Vec<String> = vec![];

    info!("Starting client_game loop");
    loop {
        //wait for target fps, and clear screen between frames
        console.wait_frame();
        console.clear_screen();

        debug!("About to send player velocity update.");
        client_updates_handler::send_player_updates(sender, &console, &mut player, server_addr);

        debug!("About to wait for entity updates from server.");
        player = client_updates_handler::check_for_received_player_updates(
            &player_update_receiver,
            player,
        );
        other_entities = client_updates_handler::check_for_received_entity_updates(
            &entity_update_receiver,
            other_entities,
        );

        client_input_handler::handle_other_input(
            &mut console,
            &mut status_messages,
            &player,
            &all_maps,
            &other_entities,
        );

        //update and redraw the screens
        draw_screens(
            &mut console,
            &all_maps,
            &player,
            &other_entities,
            &status_messages,
        );

        //check if we should quit
        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            client_logout::send_logout_notification(sender, player, server_addr);
            //sleep for a second just to make sure the logout notification is sent
            thread::sleep(Duration::from_millis(1000));
            break;
        }
    }
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}
