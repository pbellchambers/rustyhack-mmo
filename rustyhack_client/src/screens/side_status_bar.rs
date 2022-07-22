use crate::client_consts;
use console_engine::screen::Screen;
use console_engine::ConsoleEngine;
use rustyhack_lib::ecs::player::Player;

pub(crate) fn draw(player: &Player, console: &ConsoleEngine) -> Screen {
    let mut screen = Screen::new(
        console.get_width() - client_consts::VIEWPORT_WIDTH,
        client_consts::VIEWPORT_HEIGHT,
    );

    let lvl_string = "Lvl: ".to_owned() + &player.stats.level.to_string();
    let exp_string = "Exp: ".to_owned() + &player.stats.exp.to_string();
    let exp_next_string = "Next: ".to_owned() + &player.stats.exp_next.to_string();

    let hp_string = "HP: ".to_owned()
        + &player.stats.current_hp.to_string()
        + "/"
        + &player.stats.max_hp.to_string();
    let weapon_damage_string = "Damage: ".to_owned()
        + &player
            .inventory
            .equipped
            .weapon
            .damage_range
            .start
            .to_string()
        + " - "
        + &player
            .inventory
            .equipped
            .weapon
            .damage_range
            .end
            .to_string();
    let armour_value_string = "Armour: ".to_owned()
        + &player
            .inventory
            .equipped
            .armour
            .damage_reduction_percentage
            .to_string()
        + "%";
    let str_string = "Str: ".to_owned() + &player.stats.str.to_string();
    let dex_string = "Dex: ".to_owned() + &player.stats.dex.to_string();
    let con_string = "Con: ".to_owned() + &player.stats.con.to_string();

    let gold_string = "Gold: ".to_owned() + &player.inventory.gold.to_string();

    let equipment_title_string = "Equipped:".to_owned();
    let weapon_string = player.inventory.equipped.weapon.name.to_string();
    let armour_string = player.inventory.equipped.armour.name.to_string();

    let inventory_title_string = "Inventory:".to_owned();

    //todo remove this serde dependency once it's done properly
    let inventory_string = serde_json::to_string(&player.inventory.carried).unwrap();

    screen.print(1, 0, &player.player_details.player_name);
    screen.print(1, 1, &lvl_string);
    screen.print(1, 2, &exp_string);
    screen.print(1, 3, &exp_next_string);
    screen.print(1, 5, &hp_string);
    screen.print(1, 6, &weapon_damage_string);
    screen.print(1, 7, &armour_value_string);
    screen.print(1, 8, &str_string);
    screen.print(1, 9, &dex_string);
    screen.print(1, 10, &con_string);
    screen.print(1, 12, &gold_string);
    screen.print(1, 14, &equipment_title_string);
    screen.print(1, 16, &weapon_string);
    screen.print(1, 17, &armour_string);
    screen.print(1, 19, &inventory_title_string);
    screen.print(1, 20, &inventory_string);
    screen
}
