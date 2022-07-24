use crossterm::style::Color;

pub const DEFAULT_MAP: &str = "Home";
pub const DEFAULT_PLAYER_ICON: char = '@';
pub const DEFAULT_PLAYER_COLOUR: Color = Color::Magenta;
pub const DEFAULT_PLAYER_POSITION_X: u32 = 16;
pub const DEFAULT_PLAYER_POSITION_Y: u32 = 6;
pub const DEAD_MAP: &str = "Dead";
pub const DEAD_ICON: char = ' ';
pub const DEFAULT_MONSTER_TYPE: &str = "default_monster";
pub const DEFAULT_MONSTER_ICON: char = 'x';
pub const DEFAULT_ITEM_ICON: char = ',';
pub const DEFAULT_ITEM_COLOUR: Color = Color::DarkYellow;
pub const DEFAULT_MONSTER_COLOUR: Color = Color::Red;
pub const DEFAULT_MONSTER_POSITION_X: u32 = 20;
pub const DEFAULT_MONSTER_POSITION_Y: u32 = 20;
