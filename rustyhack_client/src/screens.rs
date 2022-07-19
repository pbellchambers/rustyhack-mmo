use crate::client_consts;
use console_engine::ConsoleEngine;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::math_utils::i32_from;
use rustyhack_lib::message_handler::messages::EntityUpdates;
use std::process;

mod bottom_text_window;
mod side_status_bar;
mod top_status_bar;
pub(crate) mod viewport;

pub(crate) fn draw_screens(
    console: &mut ConsoleEngine,
    all_maps: &AllMaps,
    player: &Player,
    other_entities: &EntityUpdates,
    system_messages: &[String],
) {
    //update the player viewport contents
    let viewport = viewport::draw_viewport_contents(
        player,
        all_maps
            .get(&player.position.current_map)
            .unwrap_or_else(|| {
                error!(
                    "There is no map for current player position: {}",
                    &player.position.current_map
                );
                process::exit(1);
            }),
        other_entities,
    );

    let top_status_bar = top_status_bar::draw(player);
    let side_status_bar = side_status_bar::draw(player);
    let bottom_text_window = bottom_text_window::draw(system_messages);

    //final draw step
    console.print_screen(0, 0, &top_status_bar);
    console.print_screen(i32_from(client_consts::VIEWPORT_WIDTH), 1, &side_status_bar);
    console.print_screen(
        0,
        i32_from(client_consts::VIEWPORT_HEIGHT),
        &bottom_text_window,
    );
    console.print_screen(0, 1, &viewport);
    console.draw();
}
