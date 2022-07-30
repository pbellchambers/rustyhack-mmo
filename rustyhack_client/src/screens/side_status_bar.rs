use console_engine::screen::Screen;
use console_engine::ConsoleEngine;
use rustyhack_lib::ecs::item::get_item_name;
use rustyhack_lib::ecs::player::Player;

pub(crate) fn draw(player: &Player, console: &ConsoleEngine, viewport_width: u32) -> Screen {
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

    let gold_string = "Gold: ".to_owned() + &player.inventory.gold.to_string();

    let equipped_title_string = "Equipped:".to_owned();
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

    let inventory_title_string = "Inventory:".to_owned();

    screen.print(1, 0, &player.player_details.player_name);
    screen.print(1, 1, &lvl_string);
    screen.print(1, 2, &exp_string);
    screen.print(1, 3, &exp_next_string);
    screen.print(1, 5, &hp_string);
    screen.print(1, 6, &str_string);
    screen.print(1, 7, &dex_string);
    screen.print(1, 8, &con_string);
    screen.print(1, 10, &gold_string);
    screen.print(1, 12, &equipped_title_string);
    screen.print(1, 13, &weapon_string);
    screen.print(1, 14, &armour_string);
    screen.print(1, 16, &inventory_title_string);

    let mut line_count = 17;
    for item in player.inventory.carried.clone() {
        let item_text = get_item_name(&item);
        screen.print(1, line_count, &item_text);
        line_count += 1;
    }

    screen
}
