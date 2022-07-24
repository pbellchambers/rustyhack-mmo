use crate::client_game::commands;
use console_engine::{ConsoleEngine, KeyCode};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::EntityPositionBroadcast;

pub(crate) fn handle_other_input(
    console: &mut ConsoleEngine,
    system_messages: &mut Vec<String>,
    player: &Player,
    all_maps: &AllMaps,
    entity_position_broadcast: &EntityPositionBroadcast,
) {
    if console.is_key_pressed(KeyCode::Char('l')) {
        commands::look_command::get_what_player_sees(
            system_messages,
            player,
            all_maps,
            entity_position_broadcast,
        );
    }
}
