use sdl2::mixer::{Chunk};
use std::path::Path;
use std::mem::replace;
use std::fmt::{Display, Formatter, Result};
use game::{Game, View, ViewAction};
use game::data::{Rectangle, MaybeAlive};
use game::gfx::{CopySprite, Sprite};
use game::constants::{BACKGROUND_PATH, PISTOL_AUDIO_PATH, TILES_PCS_W, TILES_PCS_H, PLAYER_SPEED, ZOOM_SPEED};
use views::tilemap::{TerrainTile, TerrainSpriteSheet, get_tiles, viewport_move, mapGlobalCoordinatesToGame};
use views::background::{Background};
use views::character::{Character, Stance};
use views::zombie::{Zombie};
use views::bullet::{Projectile};

mod bullet;
mod character;
mod zombie;
mod tilemap;
mod background;

#[derive(Clone, Debug)]
pub struct Point {
  x: f64,
  y: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
  Right = 0,
  UpRight = 1,
  Up = 2,
  UpLeft = 3,
  Left = 4,
  DownLeft = 5,
  Down = 6,
  DownRight = 7,
}

impl Display for Orientation {
  fn fmt(&self, f: &mut Formatter) -> Result {
    use views::Orientation::*;
    match *self {
      Right => write!(f, "0"),
      UpRight => write!(f, "1"),
      Up => write!(f, "2"),
      UpLeft => write!(f, "3"),
      Left => write!(f, "4"),
      DownLeft => write!(f, "5"),
      Down => write!(f, "6"),
      DownRight => write!(f, "7"),
    }
  }
}

pub struct GameView {
  character: Character,
  bullets: Vec<Box<Projectile>>,
  tiles: Vec<TerrainTile>,
  sprite_sheet: Vec<Sprite>,
  background: Background,
  zombies: Vec<Zombie>,
  pistol: Chunk,
  index: usize,
}

impl GameView {
  pub fn new(game: &mut Game) -> GameView {
    let pistol_audio = match Chunk::from_file(Path::new(PISTOL_AUDIO_PATH)) {
      Ok(f) => f,
      Err(e) => panic!("File {} not found: {}", PISTOL_AUDIO_PATH, e),
    };

    GameView {
      character: Character::new(&mut game.renderer),
      tiles: get_tiles(),
      sprite_sheet: TerrainSpriteSheet::new(&game),
      pistol: pistol_audio,
      zombies: vec![Zombie::new(&mut game.renderer)],
      background: Background {
        pos: 0.0,
        sprite: Sprite::load(&mut game.renderer, BACKGROUND_PATH).unwrap(),
      },
      index: 0,
      bullets: vec![],
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
      (true, false) => -moved * 1.5,
      (false, true) => moved * 1.5,
    };

    let dy = match (game.events.key_up, game.events.key_down) {
      (true, true) | (false, false) => 0.0,
      (true, false) => -moved * 0.75,
      (false, true) => moved * 0.75,
    };

    self.character.rect.x += dx;
    self.character.rect.y += dy;

    let movable_region = Rectangle {
      x: 0.0,
      y: 0.0,
      w: game.output_size().0,
      h: game.output_size().1,
    };

    let curr_rect = game.renderer.viewport();
    let rect = viewport_move(&game, curr_rect, dx, dy);
    game.renderer.set_viewport(rect.to_sdl());

    self.background.render(&mut game.renderer);

    self.character.rect = self.character.rect.move_inside(movable_region).unwrap();

    for x in 0..TILES_PCS_W {
      for y in 0..TILES_PCS_H {
        let index = x * TILES_PCS_H + y;
        game.renderer.copy_sprite(&self.sprite_sheet[(self.tiles[index].current - 1) as usize], self.tiles[index].rect);
      }
    }

    self.bullets =
      replace(&mut self.bullets, vec![])
        .into_iter()
        .filter_map(|b| b.update(game, elapsed))
        .collect();

    let mut transition_bullets: Vec<_> =
      replace(&mut self.bullets, vec![])
        .into_iter()
        .map(|b| MaybeAlive { alive: true, value: b })
        .collect();

    self.zombies =
      replace(&mut self.zombies, vec![])
        .into_iter()
        .filter_map(|z| {
          let mut zombie_alive = true;

          for bullet in &mut transition_bullets {
            if z.rect().overlaps(bullet.value.rect()) {
              zombie_alive = false;
            }
          }

          if zombie_alive {
            Some(z)
          } else {
            None
          }
        })
        .collect();

    self.bullets = transition_bullets.into_iter()
      .filter_map(MaybeAlive::as_option)
      .collect();

    self.zombies = replace(&mut self.zombies, vec![])
      .into_iter()
      .filter_map(|z| z.update(elapsed))
      .collect();

    match game.events.mouse_click {
      Some(_) => {
        if self.index == 0 {
          game.play_sound(&self.pistol);
        }
        self.index = if self.index < 4 { self.index + 1 } else { 0 };
        self.character.update(elapsed, dx, dy, Stance::Firing);
        self.bullets.append(&mut self.character.fire_bullets());
      },
      None => {
        self.character.update(elapsed, dx, dy, Stance::Running);
      },
    };

    for zombie in &mut self.zombies {
      zombie.render(&mut game.renderer);
    }

    self.character.render(&mut game.renderer);

    let point = mapGlobalCoordinatesToGame(self.character.rect.x, self.character.rect.y);
    println!("point {:?}", point);


    for bullet in &self.bullets {
      bullet.render(game);
    }

    let scale = game.renderer.scale();
    if game.events.zoom_in == true && scale.0 <= 2.0 && scale.1 <= 2.0 {
      let _ = game.renderer.set_scale(scale.0 + ZOOM_SPEED, scale.1 + ZOOM_SPEED);
    } else if game.events.zoom_out == true && scale.0 > 0.85 && scale.1 > 0.85 {
      let _ = game.renderer.set_scale(scale.0 - ZOOM_SPEED, scale.1 - ZOOM_SPEED);
    }

    ViewAction::None
  }
}
