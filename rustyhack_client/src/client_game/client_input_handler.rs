use crate::client_game::commands;
use console_engine::{ConsoleEngine, KeyCode};
use crossbeam_channel::Sender;
use laminar::Packet;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::EntityPositionBroadcast;

pub(crate) fn handle_other_input(
    sender: &Sender<Packet>,
    console: &mut ConsoleEngine,
    system_messages: &mut Vec<String>,
    player: &Player,
    all_maps: &AllMaps,
    entity_position_map: &EntityPositionBroadcast,
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
    } else if console.is_key_pressed(KeyCode::Char('d')) {
        info!("Drop item command pressed.");
        commands::drop_command::send_drop_item_request(
            system_messages,
            sender,
            player,
            server_addr,
        );
    }
}
