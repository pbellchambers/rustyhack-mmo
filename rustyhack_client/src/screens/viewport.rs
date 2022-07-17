use crate::client_consts;
use console_engine::pixel;
use console_engine::screen::Screen;
use rustyhack_lib::background_map::tiles::{Tile, TilePosition};
use rustyhack_lib::background_map::BackgroundMap;
use rustyhack_lib::ecs::components::DisplayDetails;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::messages::EntityUpdates;

struct Viewport {
    width: usize,
    height: usize,
    viewable_map_topleft: TilePosition,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            width: client_consts::VIEWPORT_WIDTH,
            height: client_consts::VIEWPORT_HEIGHT,
            viewable_map_topleft: TilePosition { x: 0, y: 0 },
        }
    }
}

pub(crate) fn draw_viewport_contents(
    player: &Player,
    background_map: &BackgroundMap,
    entity_updates: &EntityUpdates,
) -> Screen {
    let mut viewport = Viewport::default();
    let mut screen = Screen::new(viewport.width as u32, viewport.height as u32);
    calculate_viewable_map_coords(&mut viewport, player);
    draw_viewable_map(&mut screen, background_map, &viewport);
    draw_viewport_frame(&mut screen, &viewport);
    draw_player(&mut screen, &viewport, player);
    draw_other_entities(&mut screen, player, entity_updates, &viewport);
    screen
}

fn draw_player(screen: &mut Screen, viewport: &Viewport, player: &Player) {
    debug!("Drawing player.");
    screen.set_pxl(
        (viewport.width / 2) as i32,
        (viewport.height / 2) as i32,
        pixel::pxl_fg(player.display_details.icon, player.display_details.colour),
    )
}

fn draw_other_entities(
    screen: &mut Screen,
    player: &Player,
    entity_updates: &EntityUpdates,
    viewport: &Viewport,
) {
    debug!("Drawing other entities.");
    let default_display_details = DisplayDetails::default();
    let updates = entity_updates.position_updates.clone();
    for (name, position) in updates {
        if name != player.player_details.player_name
            && position.current_map == player.position.current_map
        {
            let relative_entity_position = TilePosition {
                x: position.pos_x as isize - viewport.viewable_map_topleft.x,
                y: position.pos_y as isize - viewport.viewable_map_topleft.y,
            };

            if relative_entity_position.x > 0
                && relative_entity_position.y > 0
                && relative_entity_position.x < (viewport.width - 1) as isize
                && relative_entity_position.y < (viewport.height - 1) as isize
            {
                let display_details = entity_updates.display_details.get(&name).unwrap_or_else(|| {
                    warn!("Entity update for {} doesn't have a corresponding display detail, using default.", &name);
                    &default_display_details});

                screen.set_pxl(
                    relative_entity_position.x as i32,
                    relative_entity_position.y as i32,
                    pixel::pxl_fg(display_details.icon, display_details.colour),
                )
            }
        }
    }
}

fn calculate_viewable_map_coords(viewport: &mut Viewport, player: &Player) {
    debug!("Calculating viewable map coordinates.");
    let x_view_distance: usize = viewport.width / 2;
    let y_view_distance: usize = viewport.height / 2;
    viewport.viewable_map_topleft = TilePosition {
        x: player.position.pos_x as isize - x_view_distance as isize,
        y: player.position.pos_y as isize - y_view_distance as isize,
    }
}

fn draw_viewable_map(screen: &mut Screen, world_map: &BackgroundMap, viewport: &Viewport) {
    debug!("Drawing viewable map.");
    let mut viewport_print_y_loc: isize = 0;
    while viewport_print_y_loc < viewport.height as isize {
        let mut viewport_print_x_loc: isize = 0;
        while viewport_print_x_loc < viewport.width as isize {
            let current_map_print_loc = TilePosition {
                x: viewport.viewable_map_topleft.x + viewport_print_x_loc,
                y: viewport.viewable_map_topleft.y + viewport_print_y_loc,
            };
            if (current_map_print_loc.x >= 0)
                && (current_map_print_loc.y >= 0)
                && (current_map_print_loc.x
                    < world_map
                        .data()
                        .get(current_map_print_loc.y as usize)
                        .unwrap_or(&vec![Tile::EmptySpace])
                        .len() as isize)
                && (current_map_print_loc.y < world_map.data().len() as isize)
            {
                screen.print(
                    viewport_print_x_loc as i32,
                    viewport_print_y_loc as i32,
                    &world_map
                        .data()
                        .get(current_map_print_loc.y as usize)
                        .unwrap_or(&vec![Tile::EmptySpace])
                        .get(current_map_print_loc.x as usize)
                        .unwrap_or(&Tile::EmptySpace)
                        .character()
                        .to_string(),
                );
            } else {
                screen.print(
                    viewport_print_x_loc as i32,
                    viewport_print_y_loc as i32,
                    " ",
                );
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
        (viewport.width - 1) as i32,
        (viewport.height - 1) as i32,
        pixel::pxl('.'),
    );
}
