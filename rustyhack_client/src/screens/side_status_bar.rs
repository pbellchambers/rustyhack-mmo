use crate::client_consts;
use console_engine::screen::Screen;
use rustyhack_lib::ecs::player::Player;

pub(crate) fn draw(player: &Player) -> Screen {
    let mut screen = Screen::new(
        client_consts::CONSOLE_WIDTH - client_consts::VIEWPORT_WIDTH,
        client_consts::VIEWPORT_HEIGHT,
    );

    let lvl_string = "Lvl: ".to_owned() + &player.player_details.level.to_string();
    let exp_string = "Exp: ".to_owned() + &player.player_details.exp.to_string();
    let hp_string = "HP: ".to_owned()
        + &player.stats.current_hp.to_string()
        + "/"
        + &player.stats.max_hp.to_string();
    let armour_string = "Armour: ".to_owned() + &player.stats.armour.to_string() + "%";
    let str_string = "Str: ".to_owned() + &player.stats.str.to_string();
    let dex_string = "Dex: ".to_owned() + &player.stats.dex.to_string();
    let con_string = "Con: ".to_owned() + &player.stats.con.to_string();
    let gold_string = "Gold: ".to_owned() + &player.player_details.gold.to_string();

    screen.print(1, 0, &player.player_details.player_name);
    screen.print(1, 1, &lvl_string);
    screen.print(1, 2, &exp_string);
    screen.print(1, 4, &hp_string);
    screen.print(1, 5, &armour_string);
    screen.print(1, 6, &str_string);
    screen.print(1, 7, &dex_string);
    screen.print(1, 8, &con_string);
    screen.print(1, 10, &gold_string);
    screen
}
