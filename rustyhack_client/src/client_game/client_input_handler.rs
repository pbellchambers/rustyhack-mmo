use crate::client_game::commands;
use console_engine::{ConsoleEngine, KeyCode};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::EntityUpdates;

pub(crate) fn handle_other_input(
    console: &mut ConsoleEngine,
    status_messages: &mut Vec<String>,
    player: &Player,
    all_maps: &AllMaps,
    other_entities: &EntityUpdates,
) {
    if console.is_key_pressed(KeyCode::Char(' ')) {
        commands::look_command::get_what_player_sees(
            status_messages,
            player,
            all_maps,
            other_entities,
        );
    }
}
