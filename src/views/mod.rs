use game::{Game, View, ViewAction};
use game::data::Rectangle;
use game::gfx::{CopySprite, Sprite};
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use chrono::*;

const PLAYER_SPEED: f64 = 150.0;
const CHARACTER_W: f64 = 56.0;
const CHARACTER_H: f64 = 43.0;

const TERRAIN_W: f64 = 102.0;
const TERRAIN_H: f64 = 67.0;

const TILES_W: usize = 16;
const TILES_H: usize = 14;

#[derive(Clone)]
struct Background {
  pos: f64,
  sprite: Sprite,
}

impl Background {
  fn render(&mut self, renderer: &mut Renderer) {
    let size = self.sprite.size();

    let (window_w, window_h) = renderer.output_size().unwrap();
    let scale = window_h as f64 / size.1;
    renderer.copy_sprite(&self.sprite, Rectangle {
      x: 0.0,
      y: 0.0,
      w: size.0,
      h: window_h as f64,
    })
  }
}

#[derive(Clone, Copy)]
enum CharacterFrame {
  Down = 0,
  DownLeft = 1,
  Left = 2,
  UpLeft = 3,
  Up = 4,
  UpRight = 5,
  Right = 6,
  DownRight = 7
}

#[derive(Clone, Copy)]
enum TerrainFrame {
  Sand = 0,
  Grass = 1,
  Water = 2,
  Wood = 3,
}

struct Character {
  rect: Rectangle,
  sprites: Vec<Sprite>,
  current: CharacterFrame,
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
    let spritesheet = Sprite::load(&mut game.renderer, "assets/warrior.png").unwrap();
    let terrain_spritesheet = Sprite::load(&mut game.renderer, "assets/terrain.png").unwrap();
    let mut sprites = Vec::with_capacity(9);
    let mut terrain_sprites = Vec::with_capacity(TILES_W);
    let mut tiles = Vec::with_capacity(TILES_W * TILES_H);

    for x in 0..3 {
      terrain_sprites.push(terrain_spritesheet.region(Rectangle {
        w: TERRAIN_W,
        h: TERRAIN_H,
        x: TERRAIN_W * x as f64,
        y: 0.0 as f64,
      }).unwrap());
    }

    for x in 0..8 {
      sprites.push(spritesheet.region(Rectangle {
        w: CHARACTER_W,
        h: CHARACTER_H,
        x: CHARACTER_W * x as f64,
        y: 0.0 as f64,
      }).unwrap());
    }

    for x in 0..TILES_W {
      for y in 0..TILES_H {
        tiles.push(TerrainTile {
          rect: Rectangle {
            x: 102.0 * x as f64,
            y: 64.0 * y as f64,
            w: TERRAIN_W,
            h: TERRAIN_H,
          },
          terrain_sprites: terrain_sprites.clone(),
          current: TerrainFrame::Grass,
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
    self.player.rect.y += dy;

    let movable_region = Rectangle {
      x: 0.0,
      y: 0.0,
      w: game.output_size().0 * 0.70,
      h: game.output_size().1,
    };

    self.player.rect = self.player.rect.move_inside(movable_region).unwrap();
    self.player.current =
    if dx == 0.0 && dy < 0.0       { CharacterFrame::Up }
    else if dx > 0.0 && dy < 0.0   { CharacterFrame::UpRight }
    else if dx < 0.0 && dy < 0.0   { CharacterFrame::UpLeft }
    else if dx == 0.0 && dy == 0.0 { CharacterFrame::Down }
    else if dx > 0.0 && dy == 0.0  { CharacterFrame::Right }
    else if dx < 0.0 && dy == 0.0  { CharacterFrame::Left }
    else if dx == 0.0 && dy > 0.0  { CharacterFrame::Down }
    else if dx > 0.0 && dy > 0.0   { CharacterFrame::DownRight }
    else if dx < 0.0 && dy > 0.0   { CharacterFrame::DownLeft }
    else { unreachable!() };

    game.renderer.set_draw_color(Color::RGB(0, 0, 0));
    game.renderer.clear();

    // background
    self.background.render(&mut game.renderer);

    // player
    game.renderer.set_draw_color(Color::RGB(119,119,119));
    game.renderer.fill_rect(self.player.rect.to_sdl().unwrap());
    game.renderer.copy_sprite(&self.player.sprites[self.player.current as usize], self.player.rect);

    // tile
    game.renderer.set_draw_color(Color::RGBA(120, 120, 120, 1));
    for x in 0..TILES_W {
      for y in 0..TILES_H {
        let index = y * TILES_H + x;
        let local: DateTime<Local> = Local::now();
        println!("{} index {:?}", local, index);
        game.renderer.fill_rect(self.tiles[index].rect.to_sdl().unwrap());
        game.renderer.copy_sprite(&self.tiles[index].terrain_sprites[self.tiles[index].current as usize], self.tiles[index].rect);
      }
    }
    ViewAction::None
  }
}
