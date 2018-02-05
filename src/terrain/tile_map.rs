use data::{get_map_tile, load_map_file};
use game::constants::{MAP_FILE_PATH, TILE_MAP_BUF_LENGTH, TILES_PCS_H, TILES_PCS_W};
use shaders::TileMapData;
use tiled::{Map, Tileset};

fn calc_index(x_pos: usize, y_pos: usize) -> usize {
  (y_pos * TILES_PCS_W) + x_pos
}

fn populate_tile_map(mut tiles: Vec<TileMapData>, map: &Map) -> Vec<TileMapData> {
  for y_pos in 0..TILES_PCS_H {
    for x_pos in 0..TILES_PCS_W {
      let map_val = get_map_tile(map, 0, x_pos, y_pos);
      let tex_x = map_val % 32;
      let y = (map_val - tex_x) / 32;
      let x = if tex_x < 1 { 1 } else { tex_x - 1 };
      let idx = calc_index(x_pos, y_pos);
      tiles[idx] = TileMapData::new([x as f32, y as f32, 0.0, 0.0]);
    }
  }
  tiles
}

#[derive(Debug)]
pub struct Terrain {
  pub tiles: Vec<TileMapData>,
  pub tile_sets: Vec<Tileset>,
}

impl Terrain {
  pub fn new() -> Terrain {
    let mut map_data = Vec::with_capacity(TILE_MAP_BUF_LENGTH);

    for _ in 0..TILE_MAP_BUF_LENGTH {
      map_data.push(TileMapData::new_empty());
    }

    let map = load_map_file(MAP_FILE_PATH);

    Terrain {
      tiles: populate_tile_map(map_data, &map),
      tile_sets: map.tilesets,
    }
  }
}
