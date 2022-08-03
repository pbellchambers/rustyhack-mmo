use console_engine::screen::Screen;
use console_engine::ConsoleEngine;
use rustyhack_lib::ecs::item::get_item_name;
use rustyhack_lib::ecs::player::Player;

pub(crate) fn draw(player: &Player, console: &ConsoleEngine, viewport_width: u32) -> Screen {
    let mut screen = Screen::new(console.get_width() - viewport_width, console.get_height());

    let esc_string = "(Esc to cancel)";
    let drop_item_string = "Drop which item?";

    screen.print(1, 0, esc_string);
    screen.print(1, 2, drop_item_string);

    let mut line_count = 4;
    for (index, item) in player.inventory.carried.iter().enumerate() {
        let item_text = "(".to_string() + &index.to_string() + ") " + &get_item_name(item);
        screen.print(1, line_count, &item_text);
        line_count += 1;
    }

    screen
}
