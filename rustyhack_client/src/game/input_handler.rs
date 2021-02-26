use chrono::{DateTime, Local};
use console_engine::{ConsoleEngine, KeyCode};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;

pub(crate) fn handle_other_input(
    console: &mut ConsoleEngine,
    status_messages: &mut Vec<String>,
    player: &Player,
    all_maps: &AllMaps,
) {
    let date_time: DateTime<Local> = Local::now();
    let time = date_time.format("[%H:%M:%S] ").to_string();
    if console.is_key_pressed(KeyCode::Char(' ')) {
        status_messages.push(time + &get_what_player_sees(player, all_maps));
    }
}

fn get_what_player_sees(player: &Player, all_maps: &AllMaps) -> String {
    let current_map = all_maps.get(&player.position.map).unwrap();
    let test = current_map.data[player.position.y as usize][player.position.x as usize];
    "You see ".to_string() + &*test.to_string()
}
