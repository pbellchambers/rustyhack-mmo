use crate::client_consts::DEFAULT_FG_COLOUR;
use chrono::{DateTime, Local};
use crossterm::style::Color;
use rayon::prelude::*;
use rustyhack_lib::background_map::AllMaps;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::EntityPositionBroadcast;

pub(crate) fn get_what_player_sees(
    system_messages: &mut Vec<(String, Color)>,
    player: &Player,
    all_maps: &AllMaps,
    entity_position_map: &EntityPositionBroadcast,
) {
    let date_time: DateTime<Local> = Local::now();
    let time = date_time.format("[%H:%M:%S] ").to_string();
    let current_map = all_maps.get(&player.position.current_map).unwrap();

    let mut underneath = current_map
        .get_tile_at(player.position.pos_y, player.position.pos_x)
        .to_string();
    let mut north = current_map
        .get_tile_at(player.position.pos_y - 1, player.position.pos_x)
        .to_string();
    let mut south = current_map
        .get_tile_at(player.position.pos_y + 1, player.position.pos_x)
        .to_string();
    let mut east = current_map
        .get_tile_at(player.position.pos_y, player.position.pos_x + 1)
        .to_string();
    let mut west = current_map
        .get_tile_at(player.position.pos_y, player.position.pos_x - 1)
        .to_string();

    underneath = return_visible_entity_at(
        underneath,
        entity_position_map,
        player,
        player.position.pos_x,
        player.position.pos_y,
    );

    north = return_visible_entity_at(
        north,
        entity_position_map,
        player,
        player.position.pos_x,
        player.position.pos_y - 1,
    );
    south = return_visible_entity_at(
        south,
        entity_position_map,
        player,
        player.position.pos_x,
        player.position.pos_y + 1,
    );
    east = return_visible_entity_at(
        east,
        entity_position_map,
        player,
        player.position.pos_x + 1,
        player.position.pos_y,
    );
    west = return_visible_entity_at(
        west,
        entity_position_map,
        player,
        player.position.pos_x - 1,
        player.position.pos_y,
    );

    system_messages.push(((time.clone() + "You see..."), DEFAULT_FG_COLOUR));
    system_messages.push((
        (time.clone() + "Underneath: " + &*underneath),
        DEFAULT_FG_COLOUR,
    ));
    system_messages.push(((time.clone() + "North: " + &*north), DEFAULT_FG_COLOUR));
    system_messages.push(((time.clone() + "South: " + &*south), DEFAULT_FG_COLOUR));
    system_messages.push(((time.clone() + "East: " + &*east), DEFAULT_FG_COLOUR));
    system_messages.push(((time + "West: " + &*west), DEFAULT_FG_COLOUR));
}

pub(super) fn return_visible_entity_at(
    entity_name: String,
    entity_position_map: &EntityPositionBroadcast,
    player: &Player,
    x: u32,
    y: u32,
) -> String {
    let entity = entity_position_map.par_iter().find_any(
        |(
            _uuid,
            (
                entity_position_x,
                entity_position_y,
                entity_current_map,
                _entity_icon,
                _entity_icon_colour,
                entity_name_or_type,
            ),
        )| {
            *entity_name_or_type != player.player_details.player_name
                && *entity_current_map == player.position.current_map
                && *entity_position_x == x
                && *entity_position_y == y
        },
    );

    match entity {
        None => entity_name,
        Some(entity) => entity.1.5.clone(),
    }
}
