use hexgametile::{hexagon::HexagonalTile, terrain::Terrain};
use hex2d::{Coordinate, Spin, XY};
use std::collections::BTreeMap;

pub fn map_ti(
    cx: Vec<(Coordinate, image::Rgba<u8>)>,
    scale: f32,
) -> BTreeMap<Coordinate, HexagonalTile> {

    let mut game_tiles = BTreeMap::new();

    for pixel in cx.iter() {
        game_tiles.insert(pixel.0, HexagonalTile::new(scale, Terrain::from(pixel.1)));
    }

    game_tiles
}

pub fn place_ti(scale: f32) -> BTreeMap<Coordinate, HexagonalTile> {
    let mut game_tiles = BTreeMap::new();
    let center = Coordinate::new(0, 0);
    game_tiles.insert(center, HexagonalTile::new(scale, Terrain::Void));


    for &c in &center.neighbors() {
        game_tiles.insert(c, HexagonalTile::new(scale, Terrain::Wood));
    }

    for ring_c in center.ring_iter(2, Spin::CCW(XY)) {
        game_tiles.insert(ring_c, HexagonalTile::new(scale, Terrain::Veg));
    }

    game_tiles
}
