use game::{Game, View, ViewAction};
use game::data::Rectangle;
use game::gfx::{CopySprite, Sprite};
use views::background::Background;
use data::load_character;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::rect::Point;
use conv::prelude::*;

mod background;

const PLAYER_SPEED: f64 = 150.0;
const CHARACTER_W: f64 = 56.0;
const CHARACTER_H: f64 = 43.0;

const TERRAIN_W: f64 = 100.0;
const TERRAIN_H: f64 = 50.0;

const TILES_W: usize = 32;
const TILES_H: usize = 20;

#[derive(Clone, Copy)]
enum CharacterFrame {
  Still = 0,
  Right = 1,
  UpRight = 28,
  Up = 56,
  UpLeft = 84,
  Left = 108,
  DownLeft = 122,
  Down = 150,
  DownRight = 178
}

#[derive(Clone, Copy)]
enum TerrainFrame {
  Sand = 0,
  Grass = 1,
}

struct Character {
  rect: Rectangle,
  sprites: Vec<Sprite>,
  current: CharacterFrame,
  anim_index: u32
}

struct TerrainTile {
  rect: Rectangle,
  terrain_sprites: Vec<Sprite>,
  current: TerrainFrame,
}

pub struct GameView {
  player: Character,
  tiles: Vec<TerrainTile>,
  background: Background,
}

impl GameView {
  pub fn new(game: &mut Game) -> GameView {
    let spritesheet = Sprite::load(&mut game.renderer, "assets/character.png").unwrap();
    let character_datapoints = load_character();
    let terrain_spritesheet = Sprite::load(&mut game.renderer, "assets/terrain.png").unwrap();
    let mut sprites = Vec::with_capacity(256);
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

    for x in 0..194 {
      sprites.push(spritesheet.region(character_datapoints[x]).unwrap());
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
            x: 100.0 * f64::value_from((x+1)).unwrap() - x2 as f64,
            y: 50.0 * f64::value_from((y+1)).unwrap() - y2 as f64,
            w: TERRAIN_W,
            h: TERRAIN_H,
          },
          terrain_sprites: terrain_sprites.clone(),
          current: TerrainFrame::Sand,
        });
      }
    }

    GameView {
      player: Character {
        rect: Rectangle {
          x: 64.0,
          y: 64.0,
          w: CHARACTER_W,
          h: CHARACTER_H,
        },
        sprites: sprites.clone(),
        current: CharacterFrame::Down,
        anim_index: 0
      },

      tiles: tiles,

      background: Background {
        pos: 0.0,
        sprite: Sprite::load(&mut game.renderer, "assets/background.png").unwrap(),
      },
    }
  }
}

impl View for GameView {
  fn render(&mut self, game: &mut Game, elapsed: f64) -> ViewAction {
    if game.events.now.quit || game.events.now.key_escape == Some(true) {
      return ViewAction::Quit;
    }

    let diagonal = (game.events.key_up ^ game.events.key_down) && (game.events.key_left ^ game.events.key_right);
    let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 } * PLAYER_SPEED * elapsed;
    let dx = match (game.events.key_left, game.events.key_right) {
      (true, true) | (false, false) => 0.0,
      (true, false) => -moved,
      (false, true) => moved,
    };

    let dy = match (game.events.key_up, game.events.key_down) {
      (true, true) | (false, false) => 0.0,
      (true, false) => -moved,
      (false, true) => moved,
    };

    self.player.rect.x += dx;
    self.player.rect.y += dy / 2.0;

    let movable_region = Rectangle {
      x: 0.0,
      y: 0.0,
      w: game.output_size().0,
      h: game.output_size().1,
    };

    self.player.rect = self.player.rect.move_inside(movable_region).unwrap();
    self.player.current =
    if dx == 0.0 && dy < 0.0       { CharacterFrame::Up }
    else if dx > 0.0 && dy < 0.0   { CharacterFrame::UpRight }
    else if dx < 0.0 && dy < 0.0   { CharacterFrame::UpLeft }
    else if dx == 0.0 && dy == 0.0 { CharacterFrame::Still }
    else if dx > 0.0 && dy == 0.0  { CharacterFrame::Right }
    else if dx < 0.0 && dy == 0.0  { CharacterFrame::Left }
    else if dx == 0.0 && dy > 0.0  { CharacterFrame::Down }
    else if dx > 0.0 && dy > 0.0   { CharacterFrame::DownRight }
    else if dx < 0.0 && dy > 0.0   { CharacterFrame::DownLeft }
    else { unreachable!() };

    game.renderer.set_draw_color(Color::RGBA(120, 120, 120, 0));
    self.background.render(&mut game.renderer);
    game.renderer.clear();

    for x in 0..TILES_W {
      for y in 0..TILES_H {
        let index = x * TILES_H + y;
        game.renderer.copy_sprite(&self.tiles[index].terrain_sprites[self.tiles[index].current as usize], self.tiles[index].rect);
      }
    }

    match game.events.mouse_click {
      Some(m) => {
        game.renderer.set_draw_color(Color::RGBA(255, 0, 0, 0));
        let _ = game.renderer.draw_line(Point::new(m.0, m.1 ), Point::new(m.0 - 10, m.1));
      },
      None => {},
    }

    // player
    game.renderer.set_draw_color(Color::RGBA(119,119,119,0));

    game.renderer.copy_sprite(&self.player.sprites[self.player.current as usize + self.player.anim_index as usize], self.player.rect);
    self.player.anim_index =
      if dx == 0.0 && dy == 0.0 { 0u32 }
      else if self.player.anim_index < 13u32 { self.player.anim_index + 1u32 }
      else { 0u32 };

    ViewAction::None
  }
}
