mod map_handler;
mod new_player;
mod updates_handler;
pub(crate) mod viewport;

use crate::consts::{GAME_TITLE, TARGET_FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use crate::game::viewport::Viewport;
use crate::networking::message_handler;
use console_engine::{ConsoleEngine, KeyCode, KeyModifiers};
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use rustyhack_lib::message_handler::player_message::EntityUpdates;
use std::collections::HashMap;
use std::process;

pub(crate) fn run(
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    server_addr: &str,
    client_addr: &str,
    player_name: &str,
) {
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

    let all_maps =
        map_handler::request_all_maps_data(&local_sender, &server_addr, &player_update_receiver);

    let mut player = new_player::send_new_player_request(
        &local_sender,
        player_name,
        &server_addr,
        &client_addr,
        &player_update_receiver,
    );

    let mut viewport = Viewport::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, TARGET_FPS);

    let mut console =
        console_engine::ConsoleEngine::init(viewport.width, viewport.height, viewport.target_fps);
    console.set_title(GAME_TITLE);
    info!("Initialised console engine.");

    let mut other_entities = EntityUpdates {
        position_updates: HashMap::new(),
        display_details: HashMap::new(),
    };

    info!("Starting game loop");
    loop {
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

        viewport.draw_viewport_contents(
            &mut console,
            &player,
            all_maps.get(&player.position.map).unwrap_or_else(|| {
                error!(
                    "There is no map for current player position: {}",
                    &player.position.map
                );
                process::exit(1);
            }),
            &other_entities,
        );

        if should_quit(&console) {
            info!("Ctrl-q detected - quitting app.");
            break;
        }
    }
}

fn should_quit(console: &ConsoleEngine) -> bool {
    console.is_key_pressed_with_modifier(KeyCode::Char('q'), KeyModifiers::CONTROL)
}
