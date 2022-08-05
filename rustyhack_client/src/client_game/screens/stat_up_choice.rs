use console_engine::screen::Screen;
use console_engine::ConsoleEngine;
use rustyhack_lib::ecs::player::Player;

pub(super) fn draw(player: &Player, console: &ConsoleEngine, viewport_width: u32) -> Screen {
    let mut screen = Screen::new(console.get_width() - viewport_width, console.get_height());

    let esc_string = "(Esc) to cancel";
    let level_up_string = "Increase which stat?";
    let str_string = "(1) Str to ".to_owned() + &(player.stats.str + 1.0).to_string();
    let dex_string = "(2) Dex to ".to_owned() + &(player.stats.dex + 1.0).to_string();
    let con_string = "(3) Con to ".to_owned() + &(player.stats.con + 1.0).to_string();
    let stat_points_string =
        "Stat points available: ".to_owned() + &player.stats.stat_points.to_string();

    screen.print(1, 0, esc_string);
    screen.print(1, 2, level_up_string);
    if player.stats.str < 100.0 {
        screen.print(1, 4, &str_string);
    }
    if player.stats.dex < 100.0 {
        screen.print(1, 5, &dex_string);
    }
    if player.stats.con < 100.0 {
        screen.print(1, 6, &con_string);
    }
    screen.print(1, 8, &stat_points_string);

    screen
}
