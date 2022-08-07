pub mod character_map;
pub mod tiles;

use crate::background_map::tiles::Tile;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundMap {
    pub data: Array2<Tile>,
}

impl BackgroundMap {
    #[must_use]
    pub fn data(&self) -> &Array2<Tile> {
        &self.data
    }

    #[must_use]
    pub fn get_tile_at(&self, y: u32, x: u32) -> &Tile {
        self.data
            .get((y as usize, x as usize))
            .unwrap_or(&Tile::EmptySpace)
    }
}

pub type AllMaps = HashMap<String, BackgroundMap>;

pub type AllMapsChunk = (usize, Vec<u8>);
