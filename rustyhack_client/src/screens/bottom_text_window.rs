use crate::client_consts;
use console_engine::screen::Screen;

pub(crate) fn draw(status_messages: &[String]) -> Screen {
    let mut screen = Screen::new(
        client_consts::CONSOLE_WIDTH as u32,
        (client_consts::CONSOLE_HEIGHT - client_consts::VIEWPORT_HEIGHT) as u32,
    );
    if !status_messages.is_empty() {
        for (count, message) in status_messages.iter().rev().enumerate() {
            if (count as u32) < screen.get_height() {
                screen.print(0, (screen.get_height() - 1 - count as u32) as i32, message);
            } else {
                break;
            }
        }
    }
    screen
}
