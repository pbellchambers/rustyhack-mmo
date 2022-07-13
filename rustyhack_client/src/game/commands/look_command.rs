use chrono::{DateTime, Local};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::EntityUpdates;

pub(crate) fn get_what_player_sees(
    status_messages: &mut Vec<String>,
    player: &Player,
    all_maps: &AllMaps,
    other_entities: &EntityUpdates,
) {
    let date_time: DateTime<Local> = Local::now();
    let time = date_time.format("[%H:%M:%S] ").to_string();
    let current_map = all_maps.get(&player.position.map).unwrap();

    let underneath =
        current_map.data[player.position.y as usize][player.position.x as usize].to_string();
    let mut north =
        current_map.data[(player.position.y - 1) as usize][player.position.x as usize].to_string();
    let mut south =
        current_map.data[(player.position.y + 1) as usize][player.position.x as usize].to_string();
    let mut east =
        current_map.data[player.position.y as usize][(player.position.x + 1) as usize].to_string();
    let mut west =
        current_map.data[player.position.y as usize][(player.position.x - 1) as usize].to_string();

    north = return_visible_entity_at(
        north,
        other_entities,
        player,
        player.position.x,
        player.position.y - 1,
    );
    south = return_visible_entity_at(
        south,
        other_entities,
        player,
        player.position.x,
        player.position.y + 1,
    );
    east = return_visible_entity_at(
        east,
        other_entities,
        player,
        player.position.x + 1,
        player.position.y,
    );
    west = return_visible_entity_at(
        west,
        other_entities,
        player,
        player.position.x - 1,
        player.position.y,
    );

    status_messages.push(time.to_owned() + "You see...");
    status_messages.push(time.to_owned() + "Underneath: " + &*underneath);
    status_messages.push(time.to_owned() + "North: " + &*north);
    status_messages.push(time.to_owned() + "South: " + &*south);
    status_messages.push(time.to_owned() + "East: " + &*east);
    status_messages.push(time + "West: " + &*west);
}

fn return_visible_entity_at(
    mut text: String,
    other_entities: &EntityUpdates,
    player: &Player,
    x: i32,
    y: i32,
) -> String {
    for (name, position) in other_entities.position_updates.clone() {
        if name != player.player_details.player_name
            && position.map == player.position.map
            && position.x == x
            && position.y == y
        {
            text = name;
        }
    }
    text
}
