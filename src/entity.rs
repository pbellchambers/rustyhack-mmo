use crate::entity::door::Door;
use crate::entity::player::Player;
use crate::entity::wall::Wall;
use crate::world_map::WorldMap;
use console_engine::ConsoleEngine;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) mod door;
pub(crate) mod player;
pub(crate) mod wall;

#[derive(Clone, Copy)]
pub enum Entity {
    Player(Player),
    Wall(Wall),
    Door(Door),
    EmptySpace,
    Boundary,
    NewLine,
    EndOfFile,
}

impl Entity {
    pub fn character(&self) -> char {
        match self {
            Entity::Player(player) => player.character_icon,
            Entity::Wall(wall) => wall.character_icon,
            Entity::Door(door) => door.character_icon,
            Entity::EmptySpace => ' ',
            Entity::Boundary => '#',
            Entity::NewLine => ' ',
            Entity::EndOfFile => ' ',
        }
    }

    pub fn update_entities(
        console: &ConsoleEngine,
        entity_map: &mut HashMap<Uuid, Entity>,
        world_map: &WorldMap,
    ) {
        for entity in entity_map.values_mut() {
            match entity {
                Entity::Player(ref mut player) => {
                    Player::update_player_location(player, &console, &world_map)
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub enum Collidable {
    True,
    False,
}

#[derive(Clone, Copy)]
pub enum OpenState {
    Open,
    Closed,
}
