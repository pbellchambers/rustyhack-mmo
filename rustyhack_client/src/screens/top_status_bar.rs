use crate::client_consts;
use console_engine::pixel;
use console_engine::screen::Screen;
use rustyhack_lib::ecs::player::Player;

pub(crate) fn draw(player: &Player) -> Screen {
    let mut screen = Screen::new(client_consts::CONSOLE_WIDTH, 1);
    screen.line(
        0,
        0,
        (client_consts::CONSOLE_WIDTH - 1) as i32,
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
        (client_consts::CONSOLE_WIDTH / 4) as i32,
        0,
        &player_update_text,
    );
    screen
}
