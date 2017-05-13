use gfx;
use gfx_app;
use gfx::{Resources};
use gfx_app::{Application};
use cgmath::{Transform, Point3, Vector3};
use cgmath::{AffineMatrix3};

use winit;
use winit::VirtualKeyCode as Key;
use winit::Event::KeyboardInput;
use winit::ElementState;
use std::process;
use std::fmt::{Display, Formatter, Result};

use game::gfx_macros::{pipe, TileMapData};
use game::graphics::{TileMapPlane};
use views::data::{InputState, MapControls};

mod data;

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
  events: MapControls,
}

impl<R: Resources> Application<R> for TileMap<R> {
  fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend,
                             window_targets: gfx_app::WindowTargets<R>) -> Self {
    use gfx::traits::FactoryExt;

    let vs = gfx_app::shade::Source {
      glsl_400: include_bytes!("../shader/vertex_shader.glsl"),
      ..gfx_app::shade::Source::empty()
    };
    let fs = gfx_app::shade::Source {
      glsl_400: include_bytes!("../shader/fragment_shader.glsl"),
      ..gfx_app::shade::Source::empty()
    };

    // set up charmap plane and configure its tiles
    let tilemap_size = [32, 32];
    let tilemap_dimensions = [32, 32];
    let tile_size = 64;

    let mut tiles = Vec::new();
    for _ in 0..tilemap_size[0] * tilemap_size[1] {
      tiles.push(TileMapData::new_empty());
    }

    let mut tilemap = TileMap {
      tiles: tiles,
      pso: factory.create_pipeline_simple(
        vs.select(backend).unwrap(),
        fs.select(backend).unwrap(),
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
        distance: 2000.0,
        x_pos: 0.0,
        y_pos: 0.0,
        move_amt: 20.0,
      },
      events: MapControls::new(),
  };

    tilemap.populate_tilemap(tilemap_size);
    tilemap.load_player();
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

    fn handle_event(state: ElementState, is_pressed: &mut bool) {
      match state {
        ElementState::Pressed => *is_pressed = true,
        ElementState::Released => *is_pressed = false,
      }
    }

    match event {
      KeyboardInput(_, _, Some(Key::Equals)) => {
        self.input.distance -= 20.0;
      }
      KeyboardInput(_, _, Some(Key::Minus)) => {
        self.input.distance += 20.0;
      }
      KeyboardInput(state, _, Some(Key::Up)) => {
        handle_event(state, &mut self.events.map_up);
      }
      KeyboardInput(state, _, Some(Key::Down)) => {
        handle_event(state, &mut self.events.map_down);
      }
      KeyboardInput(state, _, Some(Key::Left)) => {
        handle_event(state, &mut self.events.map_left);
      }
      KeyboardInput(state, _, Some(Key::Right)) => {
        handle_event(state, &mut self.events.map_right);
      }
      KeyboardInput(_, _, Some(Key::Escape)) => {
        process::exit(0);
      }
      _ => ()
    }

    let diagonal = (self.events.map_up ^ self.events.map_down) && (self.events.map_left ^ self.events.map_right);

    let moved = if diagonal { 1.0 / 2.0f32.sqrt() } else { 1.0 } * 10.0;
    let dx = match (self.events.map_left, self.events.map_right) {
      (true, true) | (false, false) => 0.0,
      (true, false) => -moved * 1.5,
      (false, true) => moved * 1.5,
    };

    let dy = match (self.events.map_up, self.events.map_down) {
      (true, true) | (false, false) => 0.0,
      (true, false) => -moved * 0.75,
      (false, true) => moved * 0.75,
    };

    self.input.x_pos += dx;
    self.input.y_pos += dy;
  }

  fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
    self.tilemap_plane.resize(window_targets);
  }
}
