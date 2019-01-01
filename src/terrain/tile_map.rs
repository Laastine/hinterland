use crate::data::{get_map_tile, load_map_file};
use crate::game::constants::{MAP_FILE_PATH, TILE_MAP_BUF_LENGTH, TILES_PCS_H, TILES_PCS_W};
use crate::shaders::TileMapData;
use tiled::Map;

fn calc_index(x_pos: usize, y_pos: usize) -> usize {
  (y_pos * TILES_PCS_W) + x_pos
}

fn populate_tile_map<'a>(tiles: &'a mut Vec<TileMapData>, map: &Map) -> &'a mut Vec<TileMapData> {
  for y_pos in 0..TILES_PCS_H {
    for x_pos in 0..TILES_PCS_W {
      let map_val = get_map_tile(map, 0, x_pos, y_pos) - 1;
      let idx = calc_index(x_pos, y_pos);

      if idx < TILE_MAP_BUF_LENGTH {
        tiles[idx] =
          TileMapData::new([map_val as f32, 0.0, 0.0, 0.0]);
      } else if idx < TILE_MAP_BUF_LENGTH * 2 {
        tiles[idx - TILE_MAP_BUF_LENGTH] =
          TileMapData::new([tiles[idx - TILE_MAP_BUF_LENGTH].data[0], map_val as f32, 0.0, 0.0]);
      } else if idx < TILE_MAP_BUF_LENGTH * 3 {
        tiles[idx - TILE_MAP_BUF_LENGTH * 2] =
          TileMapData::new([tiles[idx - TILE_MAP_BUF_LENGTH * 2].data[0], tiles[idx - TILE_MAP_BUF_LENGTH * 2].data[1], map_val as f32, 0.0]);
      } else {
        tiles[idx - TILE_MAP_BUF_LENGTH * 3] =
          TileMapData::new([tiles[idx - TILE_MAP_BUF_LENGTH * 3].data[0], tiles[idx - TILE_MAP_BUF_LENGTH * 3].data[1], tiles[idx - TILE_MAP_BUF_LENGTH * 3].data[2], map_val as f32]);
      }

    }
  }
  tiles
}

pub struct Terrain {
  pub tiles: Vec<TileMapData>,
  pub tile_sets: [Map; 1],
  pub curr_tile_set_idx: usize,
}

impl Terrain {
  pub fn new() -> Terrain {
    let mut map_data = Vec::with_capacity(TILE_MAP_BUF_LENGTH);

    for _ in 0..TILE_MAP_BUF_LENGTH {
      map_data.push(TileMapData::new_empty());
    }

    let map_a = load_map_file(MAP_FILE_PATH);

    Terrain {
      tiles: populate_tile_map(&mut map_data, &map_a).to_vec(),
      tile_sets: [map_a],
      curr_tile_set_idx: 0,
    }
  }
}
