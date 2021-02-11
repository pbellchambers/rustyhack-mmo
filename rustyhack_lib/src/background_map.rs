pub mod character_map;
pub mod tiles;

use crate::background_map::tiles::Tile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundMap {
    pub data: Vec<Vec<Tile>>,
}

impl BackgroundMap {
    pub fn data(&self) -> &Vec<Vec<Tile>> {
        &self.data
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Tile {
        self.data[y][x]
    }
}

pub type AllMaps = HashMap<String, BackgroundMap>;
