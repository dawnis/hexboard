//! Hexboard
//!
//! Hexboard is a library for coordinating hexagonal tile tracking and display. 
mod generation;

use crate::generation::{map_ti, circular_ring};
use hex2d::Spacing;
use hex2d::Coordinate;
use image::GenericImageView;
use nannou::prelude::*;
use std::path;
use std::collections::BTreeMap;

/// Trait which must be implemented by tiles using this libary.
pub trait Hextile {
    fn get_scale(&self) -> f32;
    fn draw(&self, c: Coordinate, d: &Draw);
    fn from_pixel(scale: f32, pixel: image::Rgba<u8>) -> Self;
    fn resize(&self, scale: f32) -> Self;
    fn default() -> Self;
}

#[derive(Default, Clone, Copy)]
struct ViewBoundary {
    left: f32,
    right: f32, 
    top: f32, 
    bottom: f32
}

/// Maps hexagonal tiles by their axial coordinate.
#[derive(Default)]
pub struct Board<T> {
    pub tiles: BTreeMap<Coordinate, T>,
    vb: ViewBoundary
}

impl<T: Hextile> Board<T> {

    /// Determines if a coordinate is in the viewing window
    fn is_viewable(&self, cd: Coordinate, scale: f32) -> bool {
        let hpc = cd.to_pixel(Spacing::FlatTop(scale));
        self.vb.left < hpc.0 && self.vb.right > hpc.0 
           && self.vb.bottom < hpc.1  && self.vb.top >  hpc.1 
    }

    /// Generates a ring of hexagons for testing.
    pub fn new(hexagon_scaling: f32, radius: i32, window: (f32, f32, f32, f32)) -> Self {
        Board {
            tiles: circular_ring(hexagon_scaling, radius),
            vb: ViewBoundary{left: window.0, right: window.1, 
                top: window.2, bottom: window.3}
        }
    }

    /// Generates a map from an image file where each pixel represents a tile. 
    pub fn from_img(image_path: &path::Path, hexagon_scaling: f32, window: (f32, f32, f32, f32)) -> Self {
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
            vb: ViewBoundary{left: window.0, 
                                              right: window.1,
                                              top: window.2,
                                              bottom: window.3}
        }
    }

    /// Draws the board using nannou.
    pub fn display(&self, offset: (i32, i32), draw: &Draw) {
        for (loc, tile) in self.tiles.iter() {
            let oc = *loc + Coordinate::new(offset.0, offset.1);
            if self.is_viewable(oc, tile.get_scale()) {
                    tile.draw(oc, draw);
               }
        }
    }

    /// Changes the zoom of the map. 
    pub fn update_scale(&mut self, new_scale: f32) -> Self {
        let mut update_game_tiles = BTreeMap::new();
        for (loc, tile) in self.tiles.iter() {
            update_game_tiles.insert(*loc, tile.resize(new_scale));
        }
        Board { tiles: update_game_tiles, vb: self.vb}
    }
}
