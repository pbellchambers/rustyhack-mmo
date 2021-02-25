use crate::consts;
use console_engine::ConsoleEngine;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::player_message::EntityUpdates;
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
) {
    //update the player viewport contents
    let viewport = viewport::draw_viewport_contents(
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

    let top_status_bar = top_status_bar::draw(player);
    let side_status_bar = side_status_bar::draw(player);

    //final draw step
    console.print_screen(0, 0, &top_status_bar);
    console.print_screen(0, 1, &viewport);
    console.print_screen(consts::VIEWPORT_WIDTH as i32, 1, &side_status_bar);
    console.draw();
}
