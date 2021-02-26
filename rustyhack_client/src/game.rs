use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use std::collections::HashMap;

use rustyhack_lib::message_handler::player_message::EntityUpdates;

use crate::consts::{CONSOLE_HEIGHT, CONSOLE_WIDTH, GAME_TITLE, TARGET_FPS};
use crate::networking::message_handler;
use crate::screens::draw_screens;

mod input_handler;
mod map_handler;
mod new_player;
mod updates_handler;

pub(crate) fn run(
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    server_addr: &str,
    client_addr: &str,
    player_name: &str,
) {
    //setup message handling threads
    let (player_update_sender, player_update_receiver) = crossbeam_channel::unbounded();
    let (entity_update_sender, entity_update_receiver) = crossbeam_channel::unbounded();
    debug!("Spawned thread channels.");
    let local_sender = sender.clone();
    message_handler::spawn_message_handler_thread(
        sender,
        receiver,
        player_update_sender,
        entity_update_sender,
    );

    //get basic data from server needed to start game
    let all_maps =
        map_handler::request_all_maps_data(&local_sender, &server_addr, &player_update_receiver);

    //create player
    let mut player = new_player::send_new_player_request(
        &local_sender,
        player_name,
        &server_addr,
        &client_addr,
        &player_update_receiver,
    );

    //initialise console engine
    let mut console =
        console_engine::ConsoleEngine::init(CONSOLE_WIDTH, CONSOLE_HEIGHT, TARGET_FPS);
    console.set_title(GAME_TITLE);
    info!("Initialised console engine.");

    let mut other_entities = EntityUpdates {
        position_updates: HashMap::new(),
        display_details: HashMap::new(),
    };

    let mut status_messages: Vec<String> = vec![];

    info!("Starting game loop");
    loop {
        //wait for target fps, and clear screen between frames
        console.wait_frame();
        console.clear_screen();

        debug!("About to send player velocity update.");
        updates_handler::send_player_updates(&local_sender, &console, &player, &server_addr);

        debug!("About to wait for entity updates from server.");
        player =
            updates_handler::check_for_received_player_updates(&player_update_receiver, player);
        other_entities = updates_handler::check_for_received_entity_updates(
            &entity_update_receiver,
            other_entities,
        );

        input_handler::handle_other_input(&mut console, &mut status_messages, &player, &all_maps);

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
            break;
        }
    }
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}
