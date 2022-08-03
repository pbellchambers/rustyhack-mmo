use console_engine::screen::Screen;
use console_engine::ConsoleEngine;
use rustyhack_lib::ecs::player::Player;

pub(crate) fn draw(_player: &Player, console: &ConsoleEngine, viewport_width: u32) -> Screen {
    let mut screen = Screen::new(console.get_width() - viewport_width, console.get_height());

    let esc_string = "(Esc to cancel)";
    let level_up_string = "Level Up?";

    screen.print(1, 0, esc_string);
    screen.print(1, 2, level_up_string);

    screen
}
