use crate::client_consts::{VIEWPORT_HEIGHT_PERCENTAGE, VIEWPORT_WIDTH_PERCENTAGE};
use console_engine::ConsoleEngine;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::math_utils::i32_from;
use rustyhack_lib::message_handler::messages::EntityPositionBroadcast;
use std::process;

mod bottom_text_window;
mod side_status_bar;
mod top_status_bar;
pub(crate) mod viewport;

pub(crate) fn draw_screens(
    console: &mut ConsoleEngine,
    all_maps: &AllMaps,
    player: &Player,
    entity_position_broadcast: &EntityPositionBroadcast,
    system_messages: &[String],
) {
    //check and update if resized
    console.check_resize();

    //this feels hacky...
    let viewport_width = console.get_width() * 100 / (10000 / VIEWPORT_WIDTH_PERCENTAGE);
    let viewport_height = console.get_height() * 100 / (10000 / VIEWPORT_HEIGHT_PERCENTAGE);
    debug!(
        "Viewport width and height is: {}, {}",
        viewport_width, viewport_height
    );

    //clear screen
    console.clear_screen();

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
        entity_position_broadcast,
        viewport_width,
        viewport_height,
    );

    let top_status_bar = top_status_bar::draw(player, console);
    let side_status_bar = side_status_bar::draw(player, console, viewport_width);
    let bottom_text_window =
        bottom_text_window::draw(system_messages, console, viewport_width, viewport_height);

    //final draw step
    console.print_screen(0, 0, &top_status_bar);
    console.print_screen(i32_from(viewport_width), 1, &side_status_bar);
    console.print_screen(0, i32_from(viewport_height), &bottom_text_window);
    console.print_screen(0, 1, &viewport);
    console.draw();
}
