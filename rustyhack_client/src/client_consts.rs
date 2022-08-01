use crossterm::style::Color;
use std::time::Duration;

pub(crate) const INITIAL_CONSOLE_WIDTH: u32 = 98;
pub(crate) const INITIAL_CONSOLE_HEIGHT: u32 = 42;
pub(crate) const VIEWPORT_WIDTH_PERCENTAGE: u32 = 70;
pub(crate) const VIEWPORT_HEIGHT_PERCENTAGE: u32 = 60;
pub(crate) const TARGET_FPS: u32 = 10;
pub(crate) const LOG_NAME: &str = "rustyhack_client.log";
pub(crate) const GAME_TITLE: &str = "Rustyhack MMO";
pub(crate) const VALID_NAME_REGEX: &str = "^[[:alpha:]]+$";
pub(crate) const NON_COLLIDABLE_OBJECTS: [char; 5] = [',', ' ', '/', '^', 'v'];
pub(crate) const CLIENT_CLEANUP_TICK: Duration = Duration::from_secs(10);
pub(crate) const DEFAULT_BG_COLOUR: Color = Color::Reset;
pub(crate) const DEFAULT_FG_COLOUR: Color = Color::Reset;
