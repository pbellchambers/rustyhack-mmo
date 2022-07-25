use crate::client_consts::NON_COLLIDABLE_OBJECTS;
use console_engine::pixel;
use console_engine::screen::Screen;
use rustyhack_lib::background_map::tiles::Tile;
use rustyhack_lib::background_map::BackgroundMap;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::math_utils::{i32_from, usize_from_i32};
use rustyhack_lib::message_handler::messages::EntityPositionBroadcast;

struct Viewport {
    width: u32,
    height: u32,
    viewable_map_top_left_position: RelativePosition,
}

#[derive(Clone, Copy, Debug)]
pub struct RelativePosition {
    //the only position that allows for negative coordinates, indicating something is positioned off-screen
    pub x: i32,
    pub y: i32,
}

pub(crate) fn draw_viewport_contents(
    player: &Player,
    background_map: &BackgroundMap,
    entity_position_map: &EntityPositionBroadcast,
    viewport_width: u32,
    viewport_height: u32,
) -> Screen {
    let mut viewport = Viewport {
        width: viewport_width,
        height: viewport_height,
        viewable_map_top_left_position: RelativePosition { x: 0, y: 0 },
    };
    let mut screen = Screen::new(viewport.width, viewport.height);
    calculate_viewable_map_coords(&mut viewport, player);
    draw_viewable_map(&mut screen, background_map, &viewport);
    draw_viewport_frame(&mut screen, &viewport);
    draw_other_entities(&mut screen, player, entity_position_map, &viewport);
    draw_player(&mut screen, &viewport, player);
    screen
}

fn draw_player(screen: &mut Screen, viewport: &Viewport, player: &Player) {
    debug!("Drawing player.");
    screen.set_pxl(
        i32_from(viewport.width / 2),
        i32_from(viewport.height / 2),
        pixel::pxl_fg(player.display_details.icon, player.display_details.colour),
    );
}

fn draw_other_entities(
    screen: &mut Screen,
    player: &Player,
    entity_position_map: &EntityPositionBroadcast,
    viewport: &Viewport,
) {
    debug!("Drawing other entities.");
    for (
        entity_position_x,
        entity_position_y,
        entity_current_map,
        entity_icon,
        entity_icon_colour,
        entity_name_or_type,
    ) in entity_position_map.values()
    {
        if *entity_name_or_type != player.player_details.player_name
            && *entity_current_map == player.position.current_map
        {
            let relative_entity_position = RelativePosition {
                x: i32_from(*entity_position_x) - viewport.viewable_map_top_left_position.x,
                y: i32_from(*entity_position_y) - viewport.viewable_map_top_left_position.y,
            };

            // don't draw anything outside of the viewable screen coordinates
            if relative_entity_position.x > 0
                && relative_entity_position.y > 0
                && relative_entity_position.x < i32_from(viewport.width - 1)
                && relative_entity_position.y < i32_from(viewport.height - 1)
            {
                //Only allow drawing entities on top of non-collidable objects
                if NON_COLLIDABLE_OBJECTS.contains(
                    &screen
                        .get_pxl(relative_entity_position.x, relative_entity_position.y)
                        .unwrap()
                        .chr,
                ) {
                    screen.set_pxl(
                        relative_entity_position.x,
                        relative_entity_position.y,
                        pixel::pxl_fg(*entity_icon, *entity_icon_colour),
                    );
                }
            }
        }
    }
}

fn calculate_viewable_map_coords(viewport: &mut Viewport, player: &Player) {
    debug!("Calculating viewable map coordinates.");
    let x_view_distance = i32_from(viewport.width / 2);
    let y_view_distance = i32_from(viewport.height / 2);
    viewport.viewable_map_top_left_position = RelativePosition {
        x: i32_from(player.position.pos_x) - x_view_distance,
        y: i32_from(player.position.pos_y) - y_view_distance,
    }
}

#[allow(clippy::similar_names)]
fn draw_viewable_map(screen: &mut Screen, world_map: &BackgroundMap, viewport: &Viewport) {
    debug!("Drawing viewable map.");
    let mut viewport_print_y_loc: i32 = 0;
    while viewport_print_y_loc < i32_from(viewport.height) {
        let mut viewport_print_x_loc: i32 = 0;
        while viewport_print_x_loc < i32_from(viewport.width) {
            let current_map_print_loc = RelativePosition {
                x: viewport.viewable_map_top_left_position.x + viewport_print_x_loc,
                y: viewport.viewable_map_top_left_position.y + viewport_print_y_loc,
            };
            if (current_map_print_loc.x >= 0)
                && (current_map_print_loc.y >= 0)
                && (usize_from_i32(current_map_print_loc.x))
                    < world_map
                        .data()
                        .get(usize_from_i32(current_map_print_loc.y))
                        .unwrap_or(&vec![Tile::EmptySpace])
                        .len()
                && (usize_from_i32(current_map_print_loc.y) < world_map.data().len())
            {
                screen.print(
                    viewport_print_x_loc,
                    viewport_print_y_loc,
                    &world_map
                        .data()
                        .get(usize_from_i32(current_map_print_loc.y))
                        .unwrap_or(&vec![Tile::EmptySpace])
                        .get(usize_from_i32(current_map_print_loc.x))
                        .unwrap_or(&Tile::EmptySpace)
                        .character()
                        .to_string(),
                );
            } else {
                screen.print(viewport_print_x_loc, viewport_print_y_loc, " ");
            }
            viewport_print_x_loc += 1;
        }
        viewport_print_y_loc += 1;
    }
}

fn draw_viewport_frame(screen: &mut Screen, viewport: &Viewport) {
    debug!("Drawing viewport frame.");
    screen.rect(
        0,
        0,
        i32_from(viewport.width - 1),
        i32_from(viewport.height - 1),
        pixel::pxl('.'),
    );
}
