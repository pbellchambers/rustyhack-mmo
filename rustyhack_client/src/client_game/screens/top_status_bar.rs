use console_engine::screen::Screen;
use console_engine::{ConsoleEngine, pixel};
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::utils::math::i32_from;

pub(super) fn draw(player: &Player, console: &ConsoleEngine) -> Screen {
    let mut screen = Screen::new(console.get_width(), 1);
    screen.line(0, 0, i32_from(console.get_width() - 1), 0, pixel::pxl('='));
    let player_update_text = player.position.current_map.clone()
        + " ("
        + &player.position.pos_x.to_string()
        + ","
        + &player.position.pos_y.to_string()
        + ")";
    screen.print(i32_from(console.get_width() / 4), 0, &player_update_text);
    screen
}
