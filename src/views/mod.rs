use game::{Game, View, ViewAction};
use game::data::Rectangle;
use game::gfx::{CopySprite, Sprite};
use views::tilemap::{TerrainTile, TerrainSpriteSheet, TILES_PCS_W, TILES_PCS_H, TILESHEET_PCS_W, TILESHEET_PCS_H, get_tiles, viewport_move};
use views::background::{Background};
use data::{load_character};
use sdl2::mixer::{Chunk};
use std::path::Path;

mod tilemap;
mod background;

const PLAYER_SPEED: f64 = 170.0;
const ZOOM_SPEED: f32 = 0.01;
const CHARACTER_W: f64 = 56.0;
const CHARACTER_H: f64 = 43.0;

pub const SCREEN_WIDTH: f64 = 1280.0;
pub const SCREEN_HEIGHT: f64 = 720.0;

const FIRE_SPRITE_START_INDEX: usize = 211;

#[derive(Clone, Copy)]
enum CharacterFrame {
  Right = 0,
  UpRight = 1,
  Up = 2,
  UpLeft = 3,
  Left = 4,
  DownLeft = 5,
  Down = 6,
  DownRight = 7,
}

struct Character {
  rect: Rectangle,
  sprites: Vec<Sprite>,
  current: CharacterFrame,
  heading: CharacterFrame,
  move_anim_index: u32,
  fire_anim_index: u32
}

pub struct GameView {
  player: Character,
  tiles: Vec<TerrainTile>,
  sprite_sheet: Vec<Sprite>,
  background: Background,
  pistol: Chunk,
}

impl GameView {
  pub fn new(game: &mut Game) -> GameView {
    let spritesheet = Sprite::load(&mut game.renderer, "assets/character.png").unwrap();
    let pistol_audio = Chunk::from_file(Path::new("assets/audio/pistol.ogg")).unwrap();
    let character_datapoints = load_character();
    let mut sprites = Vec::with_capacity(512);

    for x in 0..(FIRE_SPRITE_START_INDEX - 1) {
      sprites.push(spritesheet.region(character_datapoints[x]).unwrap());
    }

    for x in FIRE_SPRITE_START_INDEX..255 {
      sprites.push(spritesheet.region(character_datapoints[x]).unwrap());
    }

    GameView {
      player: Character {
        rect: Rectangle {
          x: SCREEN_WIDTH * 0.5,
          y: SCREEN_HEIGHT * 0.4,
          w: CHARACTER_W,
          h: CHARACTER_H,
        },
        sprites: sprites.clone(),
        current: CharacterFrame::Down,
        heading: CharacterFrame::Down,
        move_anim_index: 0,
        fire_anim_index: 0
      },

      tiles: get_tiles(),

      sprite_sheet: TerrainSpriteSheet::new(&game),

      pistol: pistol_audio,

      background: Background {
        pos: 0.0,
        sprite: Sprite::load(&mut game.renderer, "assets/background.png").unwrap(),
      }
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

    self.player.rect.x += dx * 1.5;
    self.player.rect.y += dy * 0.75;

    let movable_region = Rectangle {
      x: 0.0,
      y: 0.0,
      w: game.output_size().0 * 2.0,
      h: game.output_size().1 * 2.0,
    };

    let curr_rect = game.renderer.viewport();
    let rect = viewport_move(&game, curr_rect);
    game.renderer.set_viewport(rect.to_sdl());

    self.background.render(&mut game.renderer);

    self.player.rect = self.player.rect.move_inside(movable_region).unwrap();
    self.player.current =
    if dx == 0.0 && dy < 0.0       { CharacterFrame::Up }
    else if dx > 0.0 && dy < 0.0   { CharacterFrame::UpRight }
    else if dx < 0.0 && dy < 0.0   { CharacterFrame::UpLeft }
    else if dx == 0.0 && dy == 0.0 { self.player.heading }
    else if dx > 0.0 && dy == 0.0  { CharacterFrame::Right }
    else if dx < 0.0 && dy == 0.0  { CharacterFrame::Left }
    else if dx == 0.0 && dy > 0.0  { CharacterFrame::Down }
    else if dx > 0.0 && dy > 0.0   { CharacterFrame::DownRight }
    else if dx < 0.0 && dy > 0.0   { CharacterFrame::DownLeft }
    else { unreachable!() };

    self.player.heading = self.player.current;
    for x in 0..TILES_PCS_W {
      for y in 0..TILES_PCS_H {
        let index = x * TILES_PCS_H + y;
        game.renderer.copy_sprite(&self.sprite_sheet[(self.tiles[index].current-1) as usize], self.tiles[index].rect);
      }
    }

    match game.events.mouse_click {
      Some(_) => {
        let index = 211 + self.player.current as usize * 5 + self.player.fire_anim_index as usize;
        game.renderer.copy_sprite(&self.player.sprites[index], self.player.rect);
        self.player.fire_anim_index =
          if dx == 0.0 && dy == 0.0 { 0u32 } else if self.player.fire_anim_index < 4u32 { self.player.fire_anim_index + 1u32 } else { 0u32 };
        if self.player.fire_anim_index == 0 {
          game.play_sound(&self.pistol);
        }
      },
      None => {
        let index = self.player.current as usize * 28 + self.player.move_anim_index as usize;
        game.renderer.copy_sprite(&self.player.sprites[index], self.player.rect);
        self.player.move_anim_index =
          if dx == 0.0 && dy == 0.0 { 0u32 } else if self.player.move_anim_index < 13u32 { self.player.move_anim_index + 1u32 } else { 0u32 };
      },
    };

    let scale = game.renderer.scale();
    if game.events.zoom_in == true && scale.0 <= 2.0 && scale.1 <= 2.0 {
      let _ = game.renderer.set_scale(scale.0 + ZOOM_SPEED, scale.1 + ZOOM_SPEED);
    } else if game.events.zoom_out == true && scale.0 > 0.85 && scale.1 > 0.85 {
      let _ = game.renderer.set_scale(scale.0 - ZOOM_SPEED, scale.1 - ZOOM_SPEED);
    }

    ViewAction::None
  }
}
