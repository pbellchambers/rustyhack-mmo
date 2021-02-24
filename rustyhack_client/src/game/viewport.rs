use console_engine::{pixel, ConsoleEngine};
use rustyhack_lib::background_map::tiles::{Tile, TilePosition};
use rustyhack_lib::background_map::BackgroundMap;
use rustyhack_lib::ecs::components::DisplayDetails;
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::message_handler::player_message::EntityUpdates;

pub(crate) struct Viewport {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) target_fps: u32,
    pub(crate) viewable_map_topleft: TilePosition,
}

impl Viewport {
    pub(crate) fn new(width: u32, height: u32, target_fps: u32) -> Viewport {
        Viewport {
            width,
            height,
            target_fps,
            viewable_map_topleft: TilePosition { x: 0, y: 0 },
        }
    }

    pub(crate) fn draw_viewport_contents(
        &mut self,
        console: &mut ConsoleEngine,
        player: &Player,
        background_map: &BackgroundMap,
        entity_updates: &EntityUpdates,
    ) {
        calculate_viewable_map_coords(self, &player);
        draw_viewable_map(console, &background_map, &self);
        draw_viewport_frame(console, &self);
        draw_player(console, &self, &player);
        draw_other_entities(console, &player, &entity_updates, &self);
        console.draw();
    }
}

fn draw_player(console: &mut ConsoleEngine, viewport: &Viewport, player: &Player) {
    debug!("Drawing player.");
    console.set_pxl(
        (viewport.width / 2) as i32,
        (viewport.height / 2) as i32,
        pixel::pxl_fg(player.display_details.icon, player.display_details.colour),
    )
}

fn draw_other_entities(
    console: &mut ConsoleEngine,
    player: &Player,
    entity_updates: &EntityUpdates,
    viewport: &Viewport,
) {
    debug!("Drawing other entities.");
    let default_display_details = DisplayDetails::default();
    let updates = entity_updates.position_updates.clone();
    for (name, position) in updates {
        if name != player.player_details.player_name && position.map == player.position.map {
            let relative_entity_position = TilePosition {
                x: position.x - viewport.viewable_map_topleft.x,
                y: position.y - viewport.viewable_map_topleft.y,
            };

            if relative_entity_position.x > 0
                && relative_entity_position.y > 0
                && relative_entity_position.x < (viewport.width - 1) as i32
                && relative_entity_position.y < (viewport.height - 1) as i32
            {
                let display_details = entity_updates.display_details.get(&name).unwrap_or_else(|| {
                    warn!("Entity update for {} doesn't have a corresponding display detail, using default.", &name);
                    &default_display_details});

                console.set_pxl(
                    relative_entity_position.x,
                    relative_entity_position.y,
                    pixel::pxl_fg(display_details.icon, display_details.colour),
                )
            }
        }
    }
}

fn calculate_viewable_map_coords(viewport: &mut Viewport, player: &Player) {
    debug!("Calculating viewable map coordinates.");
    let x_view_distance = viewport.width / 2;
    let y_view_distance = viewport.height / 2;
    viewport.viewable_map_topleft = TilePosition {
        x: player.position.x - x_view_distance as i32,
        y: player.position.y - y_view_distance as i32,
    }
}

fn draw_viewable_map(console: &mut ConsoleEngine, world_map: &BackgroundMap, viewport: &Viewport) {
    debug!("Drawing viewable map.");
    let mut viewport_print_y_loc: i32 = 0;
    while viewport_print_y_loc < viewport.height as i32 {
        let mut viewport_print_x_loc: i32 = 0;
        while viewport_print_x_loc < viewport.width as i32 {
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
                        .len() as i32)
                && (current_map_print_loc.y < world_map.data().len() as i32)
            {
                console.print(
                    viewport_print_x_loc,
                    viewport_print_y_loc,
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
                console.print(viewport_print_x_loc, viewport_print_y_loc, " ");
            }
            viewport_print_x_loc += 1;
        }
        viewport_print_y_loc += 1;
    }
}

fn draw_viewport_frame(console: &mut ConsoleEngine, viewport: &Viewport) {
    debug!("Drawing viewport frame.");
    console.rect(
        0,
        0,
        (viewport.width - 1) as i32,
        (viewport.height - 1) as i32,
        pixel::pxl('.'),
    );
}
