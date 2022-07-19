use chrono::{DateTime, Local};
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::EntityUpdates;

pub(crate) fn get_what_player_sees(
    system_messages: &mut Vec<String>,
    player: &Player,
    all_maps: &AllMaps,
    other_entities: &EntityUpdates,
) {
    let date_time: DateTime<Local> = Local::now();
    let time = date_time.format("[%H:%M:%S] ").to_string();
    let current_map = all_maps.get(&player.position.current_map).unwrap();

    let underneath = current_map.data[player.position.pos_y as usize]
        [player.position.pos_x as usize]
        .to_string();
    let mut north = current_map.data[(player.position.pos_y - 1) as usize]
        [player.position.pos_x as usize]
        .to_string();
    let mut south = current_map.data[(player.position.pos_y + 1) as usize]
        [player.position.pos_x as usize]
        .to_string();
    let mut east = current_map.data[player.position.pos_y as usize]
        [(player.position.pos_x + 1) as usize]
        .to_string();
    let mut west = current_map.data[player.position.pos_y as usize]
        [(player.position.pos_x - 1) as usize]
        .to_string();

    north = return_visible_entity_at(
        north,
        other_entities,
        player,
        player.position.pos_x,
        player.position.pos_y - 1,
    );
    south = return_visible_entity_at(
        south,
        other_entities,
        player,
        player.position.pos_x,
        player.position.pos_y + 1,
    );
    east = return_visible_entity_at(
        east,
        other_entities,
        player,
        player.position.pos_x + 1,
        player.position.pos_y,
    );
    west = return_visible_entity_at(
        west,
        other_entities,
        player,
        player.position.pos_x - 1,
        player.position.pos_y,
    );

    system_messages.push(time.clone() + "You see...");
    system_messages.push(time.clone() + "Underneath: " + &*underneath);
    system_messages.push(time.clone() + "North: " + &*north);
    system_messages.push(time.clone() + "South: " + &*south);
    system_messages.push(time.clone() + "East: " + &*east);
    system_messages.push(time + "West: " + &*west);
}

fn return_visible_entity_at(
    mut entity_name: String,
    other_entities: &EntityUpdates,
    player: &Player,
    x: u32,
    y: u32,
) -> String {
    for (entity_id_or_name, position) in other_entities.position_updates.clone() {
        if entity_id_or_name != player.player_details.player_name
            && position.current_map == player.position.current_map
            && position.pos_x == x
            && position.pos_y == y
        {
            if other_entities
                .monster_type_map
                .contains_key(&*entity_id_or_name)
            {
                entity_name = other_entities
                    .monster_type_map
                    .get(&*entity_id_or_name)
                    .unwrap()
                    .clone();
            } else {
                entity_name = entity_id_or_name;
            }
        }
    }
    entity_name
}
