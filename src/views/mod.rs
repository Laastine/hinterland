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
  pub x: f64,
  pub y: f64,
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

////////////////

#[macro_use]
use game::gfx_macros;
use gfx;
use gfx_app;
use gfx::{Resources};
use gfx_app::{Application, WindowTargets};
use cgmath::{Transform, Point3, Vector3};
use cgmath::{SquareMatrix, Matrix4, AffineMatrix3};

use winit;
use winit::VirtualKeyCode as Key;
use winit::Event::KeyboardInput;
use winit::ElementState::Pressed;
use std::process;

use game::gfx_macros::{pipe, VertexData, TileMapData};
use game::constants::MAP_FILE_PATH;
use game::graphics::{TileMapPlane};
use data::{load_map_file, get_map_tile};

#[derive(Clone)]
struct InputState {
  distance: f32,
  x_pos: f32,
  y_pos: f32,
  move_amt: f32,
}

pub struct TileMap<R> where R: gfx::Resources {
  pub tiles: Vec<TileMapData>,
  pso: gfx::PipelineState<R, pipe::Meta>,
  pub tilemap_plane: TileMapPlane<R>,
  tile_size: f32,
  pub tilemap_size: [usize; 2],
  pub charmap_size: [usize; 2],
  pub limit_coords: [usize; 2],
  pub focus_coords: [usize; 2],
  pub focus_dirty: bool,
  input: InputState,
}

fn populate_tilemap<R>(tilemap: &mut TileMap<R>, tilemap_size: [usize; 2]) where R: gfx::Resources {
  for ypos in 0..tilemap_size[1] {
    for xpos in 0..tilemap_size[0] {
      tilemap.set_tile(xpos, ypos, [1.0, 4.0, 0.0, 0.0]);
    }
  }

  let tiledata = [9.0, 9.0, 0.0, 0.0];
  let map = load_map_file(MAP_FILE_PATH);
}

impl<R: Resources> Application<R> for TileMap<R> {
  fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend,
                             window_targets: gfx_app::WindowTargets<R>) -> Self {
    use gfx::traits::FactoryExt;

    let vs = gfx_app::shade::Source {
      glsl_150: include_bytes!("../shader/vertex_shader.glsl"),
      ..gfx_app::shade::Source::empty()
    };
    let ps = gfx_app::shade::Source {
      glsl_150: include_bytes!("../shader/fragment_shader.glsl"),
      ..gfx_app::shade::Source::empty()
    };

    // set up charmap plane and configure its tiles
    let tilemap_size = [32, 32];
    let tilemap_dimensions = [TILES_PCS_W, TILES_PCS_H];
    let tile_size = 42;

    let mut tiles = Vec::new();
    for _ in 0..tilemap_size[0] * tilemap_size[1] {
      tiles.push(TileMapData::new_empty());
    }

    let mut tilemap = TileMap {
      tiles: tiles,
      pso: factory.create_pipeline_simple(
        vs.select(backend).unwrap(),
        ps.select(backend).unwrap(),
        pipe::new()
      ).unwrap(),
      tilemap_plane: TileMapPlane::new(factory,
                                       tilemap_dimensions[0], tilemap_dimensions[1], tile_size,
                                       window_targets),
      tile_size: tile_size as f32,
      tilemap_size: tilemap_size,
      charmap_size: tilemap_dimensions,
      limit_coords: [tilemap_size[0] - tilemap_dimensions[0], tilemap_size[1] - tilemap_dimensions[1]],
      focus_coords: [0, 0],
      focus_dirty: false,
      input: InputState {
        distance: 800.0,
        x_pos: 0.0,
        y_pos: 0.0,
        move_amt: 10.0,
      },
    };

    populate_tilemap(&mut tilemap, tilemap_size);
    tilemap.set_focus([0, 0]);
    tilemap
  }

  fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
    let view: AffineMatrix3<f32> = Transform::look_at(
      Point3::new(self.input.x_pos, -self.input.y_pos, self.input.distance),
      Point3::new(self.input.x_pos, -self.input.y_pos, 0.0),
      Vector3::unit_y(),
    );

    self.tilemap_plane.update_view(&view);
    self.tilemap_plane.prepare_buffers(encoder, self.focus_dirty);
    self.focus_dirty = false;

    self.tilemap_plane.clear(encoder);

    encoder.draw(&self.tilemap_plane.slice, &self.pso, &self.tilemap_plane.params);
  }

  fn on(&mut self, event: winit::Event) {
    let i = self.input.clone();
    match event {
      KeyboardInput(Pressed, _, Some(Key::Equals)) => {
        self.input.distance -= i.move_amt;
      }
      KeyboardInput(Pressed, _, Some(Key::Minus)) => {
        self.input.distance += i.move_amt;
      }
      KeyboardInput(Pressed, _, Some(Key::Up)) => {
        self.input.y_pos -= i.move_amt;
      }
      KeyboardInput(Pressed, _, Some(Key::Down)) => {
        self.input.y_pos += i.move_amt;
      }
      KeyboardInput(Pressed, _, Some(Key::Left)) => {
        self.input.x_pos -= i.move_amt;
      }
      KeyboardInput(Pressed, _, Some(Key::Right)) => {
        self.input.x_pos += i.move_amt;
      }
      KeyboardInput(Pressed, _, Some(Key::Escape)) => {
        process::exit(0);
      }
      _ => ()
    }
  }

  fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
    self.tilemap_plane.resize(window_targets);
  }
}
