use console_engine::screen::Screen;
use console_engine::ConsoleEngine;
use rustyhack_lib::ecs::item::get_item_name;
use rustyhack_lib::ecs::player::Player;

pub(super) fn draw(
    player: &Player,
    console: &ConsoleEngine,
    viewport_width: u32,
    item_page_index: u16,
) -> Screen {
    let mut screen = Screen::new(console.get_width() - viewport_width, console.get_height());

    let esc_string = "(Esc) to cancel";
    let drop_item_string = "Drop which item?";

    screen.print(1, 0, esc_string);
    screen.print(1, 2, drop_item_string);

    let mut line_count = 4;
    for (index, item) in player.inventory.carried.iter().enumerate() {
        let item_display_index = if index >= (item_page_index * 10) as usize {
            index - (item_page_index * 10) as usize
        } else {
            index
        };

        //don't display more than 10 items per page
        if line_count == 14 {
            break;
        }

        //don't display items from previous pages
        if index < (item_page_index * 10) as usize {
            continue;
        }

        let item_text = if item_page_index == 0 {
            "(".to_string() + &item_display_index.to_string() + ") " + &get_item_name(item)
        } else {
            item_page_index.to_string()
                + "("
                + &item_display_index.to_string()
                + ") "
                + &get_item_name(item)
        };
        screen.print(1, line_count, &item_text);
        line_count += 1;
    }

    //display back/next options only when valid
    let can_go_next_page = can_go_next_page(player.inventory.carried.len(), item_page_index);
    if item_page_index == 0 && can_go_next_page {
        screen.print(1, 15, "(n)ext ->");
    } else if item_page_index > 0 && can_go_next_page {
        screen.print(1, 15, "<-- (b)ack | (n)ext -->");
    } else if item_page_index > 0 && !can_go_next_page {
        screen.print(1, 15, "<-- (b)ack");
    }

    screen
}

pub(crate) fn can_go_next_page(inventory_size: usize, item_page_index: u16) -> bool {
    inventory_size > ((item_page_index + 1) * 10) as usize
}
