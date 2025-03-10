use crate::client_consts::DEFAULT_BG_COLOUR;
use console_engine::ConsoleEngine;
use console_engine::screen::Screen;
use crossterm::style::Color;
use rustyhack_lib::ecs::item::get_item_name;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::utils::math::i32_from;

pub(super) fn draw(player: &Player, console: &ConsoleEngine, viewport_width: u32) -> Screen {
    let mut screen = Screen::new(console.get_width() - viewport_width, console.get_height());

    let lvl_string = "Lvl: ".to_owned() + &player.stats.level.to_string();
    let exp_string = "Exp: ".to_owned() + &player.stats.exp.to_string();
    let exp_next_string = "Next: ".to_owned() + &player.stats.exp_next.to_string();

    let hp_string = "HP: ".to_owned()
        + &player.stats.current_hp.to_string()
        + "/"
        + &player.stats.max_hp.to_string();
    let str_string = "Str: ".to_owned() + &player.stats.str.to_string();
    let dex_string = "Dex: ".to_owned() + &player.stats.dex.to_string();
    let con_string = "Con: ".to_owned() + &player.stats.con.to_string();
    let stat_points_string = "Stat (u)p available!";

    let gold_string = "Gold: ".to_owned() + &player.inventory.gold.to_string();

    let equipped_title_string = "Equipped:";
    let weapon_string = player.inventory.equipped.weapon.name.to_string()
        + " ("
        + &player
            .inventory
            .equipped
            .weapon
            .damage_range
            .start
            .to_string()
        + "-"
        + &player
            .inventory
            .equipped
            .weapon
            .damage_range
            .end
            .to_string()
        + " dmg)";
    let armour_string = player.inventory.equipped.armour.name.to_string()
        + " ("
        + &player
            .inventory
            .equipped
            .armour
            .damage_reduction_percentage
            .to_string()
        + "% armour)";

    let inventory_title_string = "Inventory:";

    let mut y = 0;
    let max_y = i32_from(screen.get_height()) - 1;
    screen.print(1, y, &player.player_details.player_name);
    screen.print(1, y + 1, &lvl_string);
    screen.print(1, y + 2, &exp_string);
    screen.print(1, y + 3, &exp_next_string);
    screen.print(1, y + 5, &hp_string);
    screen.print(1, y + 6, &str_string);
    screen.print(1, y + 7, &dex_string);
    screen.print(1, y + 8, &con_string);
    if player.stats.stat_points > 0 {
        screen.print_fbg(1, y + 9, stat_points_string, Color::Cyan, DEFAULT_BG_COLOUR);
        y += 1;
    }
    screen.print(1, y + 10, &gold_string);
    screen.print(1, y + 12, equipped_title_string);
    screen.print(1, y + 13, &weapon_string);
    screen.print(1, y + 14, &armour_string);
    screen.print(1, y + 16, inventory_title_string);

    let mut line_count = y + 17;
    for item in &player.inventory.carried {
        if line_count > max_y {
            break;
        }
        let item_text = get_item_name(item);
        screen.print(1, line_count, &item_text);
        line_count += 1;
    }

    screen
}
