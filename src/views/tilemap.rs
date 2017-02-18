use game::gfx::{Sprite};
use game::{Game};
use game::data::Rectangle;
use game::constants::{SCREEN_WIDTH, TILESHEET_PCS_W, TILESHEET_PCS_H, TILES_PCS_W, TILES_PCS_H, MAP_FILE_PATH};
use data::{load_map_file, get_tile};
use views::Point;
use conv::prelude::*;
use sdl2::rect::Rect as SdlRect;
const TRANSLATE_X_CONST: f64 = SCREEN_WIDTH * 0.8;
const TRANSLATE_Y_CONST: f64 = 0.0;

const TILES_W: f64 = 32.0;
const TILES_H: f64 = 32.0;

#[derive(Clone)]
pub struct TerrainTile {
  pub rect: Rectangle,
  pub current: u32,
}

#[derive(Clone)]
pub struct Tilemap {
  pub pos: f64,
  pub sprite: Sprite,
}

pub fn cartesian_to_isometric(point_x: f64, point_y: f64) -> Point {
  Point {
    x: (point_x - point_y),
    y: (point_x + point_y) / 2.0
  }
}

pub struct TerrainSpriteSheet {
  pub terrain_spritesheet: Vec<Sprite>
}

impl TerrainSpriteSheet {
  pub fn new(game: &Game) -> Vec<Sprite> {
    let map = load_map_file(MAP_FILE_PATH);
    let terrain_spritesheet = Sprite::load(&game.renderer, "assets/maps/terrain.png").unwrap();
    let mut terrain_sprites = Vec::with_capacity(TILESHEET_PCS_W * TILESHEET_PCS_H * 2);

    for y in 0..(map.tile_width) {
      for x in 0..(map.tile_height) {
        terrain_sprites.push(terrain_spritesheet.region(Rectangle {
          w: TILES_W,
          h: TILES_H,
          x: TILES_W * x as f64,
          y: TILES_H * y as f64,
        }).unwrap());
      }
    }
    terrain_sprites
  }
}

pub fn get_tiles() -> Vec<TerrainTile> {
  let map = load_map_file(MAP_FILE_PATH);
  let mut tiles = Vec::with_capacity(map.width as usize * map.height as usize * 2);

  for x in 0..TILES_PCS_W {
    for y in 0..TILES_PCS_H {
      let point = cartesian_to_isometric(TILES_W * x as f64, TILES_H * y as f64);
      tiles.push(TerrainTile {
        rect: Rectangle {
          x: point.x + TRANSLATE_X_CONST,
          y: point.y + TRANSLATE_Y_CONST,
          w: TILES_W,
          h: TILES_H,
        },
        current: get_tile(&map, 0, x, y),
      });
    }
  }
  tiles
}

pub fn viewport_move(game: &Game, curr_rect: SdlRect, dx: f64, dy: f64) -> Rectangle {
  Rectangle {
    x: f64::value_from(curr_rect.x()).unwrap() - dx,
    y: f64::value_from(curr_rect.y()).unwrap() - dy,
    w: game.output_size().0 * 3.0,
    h: game.output_size().1 * 3.0,
  }
}
