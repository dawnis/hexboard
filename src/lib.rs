mod generation;

use hexgametile::Rawr;
use crate::generation::{map_ti, place_ti};
use hexgametile::hexagon::HexagonalTile;
use hex2d::Coordinate;
use image::GenericImageView;
use nannou::prelude::*;
use std::path;
use std::collections::BTreeMap;

/// Gameboard which implements a coordinate: tile map paradigm.
#[derive(Default)]
pub struct Board {
    pub tiles: BTreeMap<Coordinate, HexagonalTile>
}

impl Board {
    /// Generates a ring of hexagons for testing.
    pub fn new(hexagon_scaling: f32) -> Self {
        Board {
            tiles: place_ti(hexagon_scaling),
        }
    }

    pub fn from_img(image_path: &path::Path, hexagon_scaling: f32) -> Self {
        let (width, height) = image::image_dimensions(image_path).unwrap();

        let mut cx: Vec<(Coordinate, image::Rgba<u8>)> = Vec::new();

        let img = image::open(image_path).expect("file not found");

        for pixel in img.pixels() {
            let (x, y, c) = pixel;
            let x_c = x as i32 - width as i32 / 2;
            let y_c = y as i32 - height as i32 / 4 - x as i32 / 2;
           
            let hxc = Coordinate::new(x_c, y_c);
            cx.push((hxc, c));
        }

        Board {
            tiles: map_ti(cx, hexagon_scaling),
        }
    }

    pub fn make(&self, offset: (i32, i32), draw: &Draw, bounds: Rect) {
        for (loc, tile) in self.tiles.iter() {
            tile.rawr(*loc + Coordinate::new(offset.0, offset.1), draw, bounds);
        }
    }

    pub fn update_scale(&mut self, new_scale: f32) -> Self {
        let mut update_game_tiles = BTreeMap::new();
        for (loc, tile) in self.tiles.iter() {
            update_game_tiles.insert(*loc, HexagonalTile::new(new_scale, tile.terrain));
        }
        Board { tiles: update_game_tiles}
    }
}
