use crate::client_consts::DEFAULT_BG_COLOUR;
use console_engine::ConsoleEngine;
use console_engine::screen::Screen;
use crossterm::style::Color;
use rustyhack_lib::utils::math::i32_from_usize;

pub(super) fn draw(
    system_messages: &[(String, Color)],
    console: &ConsoleEngine,
    viewport_width: u32,
    viewport_height: u32,
) -> Screen {
    let mut screen = Screen::new(viewport_width, console.get_height() - viewport_height);
    if !system_messages.is_empty() {
        system_messages
            .iter()
            .rev()
            .take((screen.get_height() - 1) as usize)
            .enumerate()
            .for_each(|(count, message)| {
                screen.print_fbg(
                    0,
                    i32_from_usize((screen.get_height() as usize) - 1 - count),
                    &message.0,
                    message.1,
                    DEFAULT_BG_COLOUR,
                );
            });
    }
    screen
}
