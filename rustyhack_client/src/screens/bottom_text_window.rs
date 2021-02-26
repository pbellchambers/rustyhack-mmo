use crate::consts;
use console_engine::screen::Screen;

pub(crate) fn draw(status_messages: &[String]) -> Screen {
    let mut screen = Screen::new(
        consts::CONSOLE_WIDTH,
        consts::CONSOLE_HEIGHT - consts::VIEWPORT_HEIGHT,
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
