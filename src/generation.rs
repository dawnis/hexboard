use crate::Hextile;
use hex2d::{Coordinate, Spin, XY};
use std::collections::BTreeMap;

pub fn map_ti<T: Hextile>(
    cx: Vec<(Coordinate, image::Rgba<u8>)>,
    scale: f32,
) -> BTreeMap<Coordinate, T> {

    let mut game_tiles = BTreeMap::new();

    for pixel in cx.iter() {
        game_tiles.insert(pixel.0, T::from_pixel(scale, pixel.1));
    }

    game_tiles
}

pub fn circular_ring<T: Hextile>(_scale: f32, layers: i32) -> BTreeMap<Coordinate, T> {
    let mut game_tiles = BTreeMap::new();
    let center = Coordinate::new(0, 0);
    game_tiles.insert(center, <T as Hextile>::default());


    for layer in 1..layers {

        if layer == 1 {
            for &c in &center.neighbors() {
                game_tiles.insert(c, <T as Hextile>::default());
            }
        } else {
            for ring_c in center.ring_iter(layer, Spin::CCW(XY)) {
                game_tiles.insert(ring_c, <T as Hextile>::default());
            }
        }
    }

    game_tiles
}
