use game::gfx::{CopySprite, Sprite};
use game::{Game};
use game::data::Rectangle;
use sdl2::render::Renderer;
use conv::prelude::*;

pub const TERRAIN_W: f64 = 100.0;
pub const TERRAIN_H: f64 = 50.0;

pub const TILES_W: usize = 32;
pub const TILES_H: usize = 20;

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
  let mut terrain_sprites = Vec::with_capacity(TILES_W);
  let mut tiles = Vec::with_capacity(TILES_W * TILES_H * 2);

  for x in 0..3 {
    terrain_sprites.push(terrain_spritesheet.region(Rectangle {
      w: TERRAIN_W,
      h: TERRAIN_H,
      x: TERRAIN_W * x as f64,
      y: 0.0 as f64,
    }).unwrap());
  }

  for x in 0..TILES_W {
    for y in 0..TILES_H {
      let x2: f64 = 100.0 * 1.5 as f64;
      let y2: f64 = 50.0 * 1.5 as f64;
      tiles.push(TerrainTile {
        rect: Rectangle {
          x: 100.0 * x as f64,
          y: 50.0 * y as f64,
          w: TERRAIN_W,
          h: TERRAIN_H,
        },
        terrain_sprites: terrain_sprites.clone(),
        current: TerrainFrame::Grass,
      });

      tiles.push(TerrainTile {
        rect: Rectangle {
          x: 100.0 * f64::value_from((x + 1)).unwrap() - x2 as f64,
          y: 50.0 * f64::value_from((y + 1)).unwrap() - y2 as f64,
          w: TERRAIN_W,
          h: TERRAIN_H,
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

impl Tilemap {
  pub fn render(&mut self, renderer: &mut Renderer) {
    let size = self.sprite.size();

    let (_, window_h) = renderer.output_size().unwrap();
    renderer.copy_sprite(&self.sprite, Rectangle {
      x: 0.0,
      y: 0.0,
      w: size.0,
      h: window_h as f64,
    })
  }
}
