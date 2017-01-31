use game::gfx::{CopySprite, Sprite};
use game::{Game};
use game::data::Rectangle;
use views::{SCREEN_WIDTH, TerrainFrame};
use data::{load_map_file, get_tile};
use sdl2::render::Renderer;
use conv::prelude::*;
use sdl2::rect::Rect as SdlRect;

pub const TERRAIN_W: f64 = 500.0;
pub const TERRAIN_H: f64 = 250.0;

pub const TILES_PCS_W: usize = 16;
pub const TILES_PCS_H: usize = 16;

const TILES_W: f64 = 32.0;
const TILES_H: f64 = 32.0;

#[derive(Clone)]
pub struct TerrainTile {
  pub rect: Rectangle,
  pub terrain_sprites: Vec<Sprite>,
  pub current: u32,
}

#[derive(Clone)]
pub struct Tilemap {
  pub pos: f64,
  pub sprite: Sprite,
}

#[derive(Clone)]
pub struct Point {
  x: f64,
  y: f64,
}

pub fn cartesian_to_isometric(point_x: f64, point_y: f64) -> Point {
  Point {
    x: (point_x - point_y),
    y: (point_x + point_y) / 2.0
  }
}

pub fn get_tiles(game: &Game) -> Vec<TerrainTile> {
  let map = load_map_file("assets/tilemap.tmx");
  let terrain_spritesheet = Sprite::load(&game.renderer, "assets/terrain.png").unwrap();
  let mut terrain_sprites = Vec::with_capacity(32 * 32);
  let mut tiles = Vec::with_capacity(map.width as usize * map.height as usize * 2);

  for x in 0..(map.tile_width-1) {
    for y in 0..(map.tile_height-1) {
      terrain_sprites.push(terrain_spritesheet.region(Rectangle {
        w: TILES_W,
        h: TILES_H,
        x: TILES_W * y as f64,
        y: TILES_H * x as f64,
      }).unwrap());
    }
  }

  for x in 0..TILES_PCS_W {
    for y in 0..TILES_PCS_H {
      let point = cartesian_to_isometric(TILES_W * x as f64, TILES_H * y as f64);
      let tile_index = get_tile(&map, 0, x, y);
      tiles.push(TerrainTile {
        rect: Rectangle {
          x: point.x + (SCREEN_WIDTH / 2.0),
          y: point.y,
          w: TILES_W,
          h: TILES_H,
        },
        terrain_sprites: terrain_sprites.clone(),
        current: get_tile(&map, 0, x, y),
      });
    }
  }
  tiles
}

pub fn viewport_move(game: &Game, curr_rect: SdlRect) -> Rectangle {
  if game.events.move_right == true {
    Rectangle {
      x: f64::value_from(curr_rect.x() - 3).unwrap(),
      y: f64::value_from(curr_rect.y()).unwrap(),
      w: game.output_size().0 * 3.0,
      h: game.output_size().1 * 3.0,
    }
  } else if game.events.move_left == true && curr_rect.x() < 0  {
    Rectangle {
      x: f64::value_from(curr_rect.x() + 3).unwrap(),
      y: f64::value_from(curr_rect.y()).unwrap(),
      w: game.output_size().0 * 3.0,
      h: game.output_size().1 * 3.0,
    }
  } else if game.events.move_up == true && curr_rect.y() < 0 {
    Rectangle {
      x: f64::value_from(curr_rect.x()).unwrap(),
      y: f64::value_from(curr_rect.y() + 3).unwrap(),
      w: game.output_size().0 * 3.0,
      h: game.output_size().1 * 3.0,
    }
  } else if game.events.move_down == true {
    Rectangle {
      x: f64::value_from(curr_rect.x()).unwrap(),
      y: f64::value_from(curr_rect.y() - 3).unwrap(),
      w: game.output_size().0 * 3.0,
      h: game.output_size().1 * 3.0,
    }
  } else {
    Rectangle {
      x: f64::value_from(curr_rect.x()).unwrap(),
      y: f64::value_from(curr_rect.y()).unwrap(),
      w: game.output_size().0 * 3.0,
      h: game.output_size().1 * 3.0,
    }
  }
}
