use game::gfx::{CopySprite, Sprite};
use game::{Game};
use game::data::Rectangle;
use sdl2::render::Renderer;
use conv::prelude::*;

pub const TERRAIN_W: f64 = 500.0;
pub const TERRAIN_H: f64 = 250.0;

pub const TILES_PCS_W: usize = 160;
pub const TILES_PCS_H: usize = 100;

const TILES_W: f64 = 100.0;
const TILES_H: f64 = 50.0;

#[derive(Clone, Copy)]
pub enum TerrainFrame {
  Sand = 0,
  Grass = 1,
}

#[derive(Clone)]
pub struct TerrainTile {
  pub rect: Rectangle,
  pub terrain_sprites: Vec<Sprite>,
  pub current: TerrainFrame,
}

pub fn get_tiles(game: &Game) -> Vec<TerrainTile> {
  let terrain_spritesheet = Sprite::load(&game.renderer, "assets/terrain.png").unwrap();
  let mut terrain_sprites = Vec::with_capacity(TILES_PCS_W );
  let mut tiles = Vec::with_capacity(TILES_PCS_W * TILES_PCS_H * 2);

  for x in 0..3 {
    terrain_sprites.push(terrain_spritesheet.region(Rectangle {
      w: TILES_W,
      h: TILES_H,
      x: TILES_W * x as f64,
      y: 0.0 as f64,
    }).unwrap());
  }

  for x in 0..TILES_PCS_W {
    for y in 0..TILES_PCS_H {
      let x2: f64 = TILES_W * 1.5 as f64;
      let y2: f64 = TILES_H * 1.5 as f64;
      tiles.push(TerrainTile {
        rect: Rectangle {
          x: TILES_W * x as f64,
          y: TILES_H * y as f64,
          w: TILES_W,
          h: TILES_H,
        },
        terrain_sprites: terrain_sprites.clone(),
        current: TerrainFrame::Grass,
      });

      tiles.push(TerrainTile {
        rect: Rectangle {
          x: TILES_W * f64::value_from((x + 1)).unwrap() - x2 as f64,
          y: TILES_H * f64::value_from((y + 1)).unwrap() - y2 as f64,
          w: TILES_W,
          h: TILES_H,
        },
        terrain_sprites: terrain_sprites.clone(),
        current: TerrainFrame::Sand,
      });
    }
  }
  tiles
}

#[derive(Clone)]
pub struct Tilemap {
  pub pos: f64,
  pub sprite: Sprite,
}
