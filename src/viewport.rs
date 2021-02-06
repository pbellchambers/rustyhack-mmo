use crate::entity::{Entity, Location};
use crate::world_map::WorldMap;
use console_engine::{pixel, ConsoleEngine};
use std::collections::HashMap;
use uuid::Uuid;

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
        console: &mut ConsoleEngine,
        entity_map: &HashMap<Uuid, Entity>,
        viewport: &Viewport,
        world_map: &WorldMap,
        current_player_uuid: &Uuid,
    ) {
        let viewable_map_coords: Location =
            calculate_viewable_map_coords(&viewport, entity_map.get(&current_player_uuid).unwrap());
        draw_viewable_map(console, &world_map, &viewable_map_coords, &viewport);
        draw_viewport_frame(console, &viewport);
        draw_viewable_entities(console, &entity_map, &viewport);
        console.draw();
    }
}

fn draw_viewable_map(
    console: &mut ConsoleEngine,
    world_map: &WorldMap,
    viewable_map_topleft: &Location,
    viewport: &Viewport,
) {
    let mut viewport_print_y_loc: i32 = 0;
    while viewport_print_y_loc < viewport.height as i32 {
        let mut viewport_print_x_loc: i32 = 0;
        while viewport_print_x_loc < viewport.width as i32 {
            let current_map_print_loc = Location {
                x: viewable_map_topleft.x + viewport_print_x_loc,
                y: viewable_map_topleft.y + viewport_print_y_loc,
            };
            //NOTE: This will panic if the map file doesn't have half a viewport size
            // of empty characters to print at the bottom of the map:
            // current_map.print_loc.y can be out of bounds of world_map.data()[]
            if (current_map_print_loc.x >= 0)
                && (current_map_print_loc.y >= 0)
                && (current_map_print_loc.x
                < world_map.data()[current_map_print_loc.y as usize].len() as i32)
                && (current_map_print_loc.y < world_map.data().len() as i32)
            {
                console.print(
                    viewport_print_x_loc,
                    viewport_print_y_loc,
                    &world_map.data()[current_map_print_loc.y as usize]
                        [current_map_print_loc.x as usize]
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

fn calculate_viewable_map_coords(viewport: &Viewport, player_entity: &Entity) -> Location {
    if let Entity::Player(player) = player_entity {
        let x_view_distance = viewport.width / 2;
        let y_view_distance = viewport.height / 2;
        Location {
            x: player.location.x - x_view_distance as i32,
            y: player.location.y - y_view_distance as i32,
        }
    } else {
        panic!("Current player uuid is not an Entity:Player. This should not be possible.");
    }
}

fn draw_viewable_entities(
    console: &mut ConsoleEngine,
    entity_map: &HashMap<Uuid, Entity>,
    viewport: &Viewport,
) {
    for entity in entity_map.values() {
        if let Entity::Player(player) = entity {
            console.set_pxl(
                (viewport.width / 2) as i32,
                (viewport.height / 2) as i32,
                pixel::pxl_fg(player.character_icon, player.colour),
            )
        }
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
