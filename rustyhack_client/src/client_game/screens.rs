use crate::client_consts::{VIEWPORT_HEIGHT_PERCENTAGE, VIEWPORT_WIDTH_PERCENTAGE};
use console_engine::ConsoleEngine;
use crossterm::style::Color;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::EntityPositionBroadcast;
use rustyhack_lib::utils::math::i32_from;
use std::process;

mod bottom_text_window;
pub(crate) mod drop_item_choice;
mod side_status_bar;
mod stat_up_choice;
mod top_status_bar;
pub(crate) mod viewport;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SidebarState {
    StatusBar,
    DropItemChoice(u16),
    StatUpChoice,
}

pub(crate) fn draw_screens(
    console: &mut ConsoleEngine,
    all_maps: &AllMaps,
    player: &Player,
    entity_position_broadcast: &EntityPositionBroadcast,
    system_messages: &[(String, Color)],
    sidebar_state: SidebarState,
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
    let side_bar = match sidebar_state {
        SidebarState::StatusBar => side_status_bar::draw(player, console, viewport_width),
        SidebarState::DropItemChoice(item_page_index) => {
            drop_item_choice::draw(player, console, viewport_width, item_page_index)
        }
        SidebarState::StatUpChoice => stat_up_choice::draw(player, console, viewport_width),
    };
    let bottom_text_window =
        bottom_text_window::draw(system_messages, console, viewport_width, viewport_height);

    //final draw step
    console.print_screen(0, 0, &top_status_bar);
    console.print_screen(i32_from(viewport_width), 1, &side_bar);
    console.print_screen(0, i32_from(viewport_height), &bottom_text_window);
    console.print_screen(0, 1, &viewport);
    console.draw();
}
