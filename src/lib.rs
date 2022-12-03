//! Hexboard
//!
//! Hexboard is a library for coordinating hexagonal tile tracking and display. 
mod generation;

use crate::generation::{map_ti, circular_ring};
use hex2d::Coordinate;
use image::GenericImageView;
use nannou::prelude::*;
use std::path;
use std::collections::BTreeMap;

/// Hexagonal tiles must implement `Rawr` which interacts with nannou for drawing and `terrain`
/// which ...

pub trait Hextile {
    fn rawr(&self, c: Coordinate, d: &Draw, bounds: Rect);
    fn from_pixel(scale: f32, pixel: image::Rgba<u8>) -> Self;
    fn resize(&self, scale: f32) -> Self;
    fn default() -> Self;
}

/// Maps hexagonal tiles by their axial coordinate.
#[derive(Default)]
pub struct Board<T> {
    pub tiles: BTreeMap<Coordinate, T>
}

impl<T: Hextile> Board<T> {
    /// Generates a ring of hexagons for testing.
    pub fn new(hexagon_scaling: f32, radius: i32) -> Self {
        Board {
            tiles: circular_ring(hexagon_scaling, radius),
        }
    }

    /// Generates a map from an image file where each pixel represents a tile. 
    /// To Do: add pixel mapping to the API. 
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

    /// Draws the board using nannou.
    pub fn make(&self, offset: (i32, i32), draw: &Draw, bounds: Rect) {
        for (loc, tile) in self.tiles.iter() {
            tile.rawr(*loc + Coordinate::new(offset.0, offset.1), draw, bounds);
        }
    }

    /// Changes the zoom of the map. 
    pub fn update_scale(&mut self, new_scale: f32) -> Self {
        let mut update_game_tiles = BTreeMap::new();
        for (loc, tile) in self.tiles.iter() {
            update_game_tiles.insert(*loc, tile.resize(new_scale));
        }
        Board { tiles: update_game_tiles}
    }
}
