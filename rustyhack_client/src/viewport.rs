use crate::player::Player;
use console_engine::{pixel, ConsoleEngine};
use rustyhack_lib::background_map::tiles::{Tile, TilePosition};
use rustyhack_lib::background_map::BackgroundMap;

pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32, target_fps: u32) -> Viewport {
        Viewport {
            width,
            height,
            target_fps,
        }
    }

    pub fn draw_viewport_contents(
        &self,
        console: &mut ConsoleEngine,
        player: &Player,
        background_map: &BackgroundMap,
    ) {
        let viewable_map_coords: TilePosition = calculate_viewable_map_coords(&self, &player);
        draw_viewable_map(console, &background_map, &viewable_map_coords, &self);
        draw_viewport_frame(console, &self);
        draw_viewable_entities(console, &self, &player);
        console.draw();
    }
}

fn draw_viewable_entities(console: &mut ConsoleEngine, viewport: &Viewport, player: &Player) {
    console.set_pxl(
        (viewport.width / 2) as i32,
        (viewport.height / 2) as i32,
        pixel::pxl_fg(player.character.icon, player.entity_colour.colour),
    )
}

fn calculate_viewable_map_coords(viewport: &Viewport, player: &Player) -> TilePosition {
    let x_view_distance = viewport.width / 2;
    let y_view_distance = viewport.height / 2;
    TilePosition {
        x: player.position.x - x_view_distance as i32,
        y: player.position.y - y_view_distance as i32,
    }
}

fn draw_viewable_map(
    console: &mut ConsoleEngine,
    world_map: &BackgroundMap,
    viewable_map_topleft: &TilePosition,
    viewport: &Viewport,
) {
    let mut viewport_print_y_loc: i32 = 0;
    while viewport_print_y_loc < viewport.height as i32 {
        let mut viewport_print_x_loc: i32 = 0;
        while viewport_print_x_loc < viewport.width as i32 {
            let current_map_print_loc = TilePosition {
                x: viewable_map_topleft.x + viewport_print_x_loc,
                y: viewable_map_topleft.y + viewport_print_y_loc,
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
    console.rect(
        0,
        0,
        (viewport.width - 1) as i32,
        (viewport.height - 1) as i32,
        pixel::pxl('.'),
    );
}
