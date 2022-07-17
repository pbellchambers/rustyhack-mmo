use crate::client_consts;
use console_engine::pixel;
use console_engine::screen::Screen;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::math_utils::i32_from;

pub(crate) fn draw(player: &Player) -> Screen {
    let mut screen = Screen::new(client_consts::CONSOLE_WIDTH, 1);
    screen.line(
        0,
        0,
        i32_from(client_consts::CONSOLE_WIDTH - 1),
        0,
        pixel::pxl('='),
    );
    let player_update_text = player.position.current_map.clone()
        + " ("
        + &player.position.pos_x.to_string()
        + ","
        + &player.position.pos_y.to_string()
        + ")";
    screen.print(
        i32_from(client_consts::CONSOLE_WIDTH / 4),
        0,
        &player_update_text,
    );
    screen
}
