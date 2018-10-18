use data::{get_map_tile, load_map_file};
use game::constants::{MAP_A_FILE_PATH, MAP_B_FILE_PATH, TILE_MAP_BUF_LENGTH, TILES_PCS_H, TILES_PCS_W};
use shaders::TileMapData;
use tiled::Map;

fn calc_index(x_pos: usize, y_pos: usize) -> usize {
  (y_pos * TILES_PCS_W) + x_pos
}

fn populate_tile_map<'a>(tiles: &'a mut Vec<TileMapData>, map: &Map) -> &'a mut Vec<TileMapData> {
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
  pub tile_sets: [Map; 2],
  pub curr_tile_set_idx: usize,
}

impl Terrain {
  pub fn new() -> Terrain {
    let mut map_data = Vec::with_capacity(TILE_MAP_BUF_LENGTH);

    for _ in 0..TILE_MAP_BUF_LENGTH {
      map_data.push(TileMapData::new_empty());
    }

    let map_a = load_map_file(MAP_A_FILE_PATH);
    let map_b = load_map_file(MAP_B_FILE_PATH);

    Terrain {
      tiles: populate_tile_map(&mut map_data, &map_a).to_vec(),
      tile_sets: [map_a, map_b],
      curr_tile_set_idx: 0,
    }
  }

  pub fn change_map(&mut self, idx: usize) {
    self.tiles = populate_tile_map(&mut self.tiles, &self.tile_sets[idx]).to_vec();
    self.curr_tile_set_idx = idx;
  }
}
