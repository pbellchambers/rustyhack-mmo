use crate::client_consts;
use console_engine::screen::Screen;
use console_engine::ConsoleEngine;
use rustyhack_lib::math_utils::i32_from_usize;

pub(crate) fn draw(system_messages: &[String], console: &ConsoleEngine) -> Screen {
    let mut screen = Screen::new(
        console.get_width(),
        console.get_height() - client_consts::VIEWPORT_HEIGHT,
    );
    if !system_messages.is_empty() {
        for (count, message) in system_messages.iter().rev().enumerate() {
            if (count) < screen.get_height() as usize {
                screen.print(
                    0,
                    i32_from_usize((screen.get_height() as usize) - 1 - count),
                    message,
                );
            } else {
                break;
            }
        }
    }
    screen
}
