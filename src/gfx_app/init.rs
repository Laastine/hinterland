use std::time;

use specs::{Builder, DispatcherBuilder, World};

use crate::character;
use crate::character::controls::CharacterControlSystem;
use crate::critter::CharacterSprite;
use crate::gfx_app::{system::DrawSystem, WindowContext, WindowStatus};
use crate::gfx_app::controls::TilemapControls;
use crate::graphics::{camera, DeltaTime, GameTime};
use crate::graphics::camera::CameraControlSystem;
use crate::graphics::dimensions::Dimensions;
use crate::terrain;
//use crate::gfx_app::renderer::DeviceRenderer;

pub fn run(window_context: &mut WindowContext) {
  let mut w = World::new();
  setup_world(&mut w, window_context.get_viewport_size(), window_context.get_hidpi_factor());
  dispatch_loop(window_context, &mut w);
}

fn setup_world(world: &mut World, viewport_size: (f32, f32), hidpi_factor: f32) {
  world.register::<terrain::TerrainDrawable>();
  world.register::<character::CharacterDrawable>();
  world.register::<CharacterSprite>();
  world.register::<camera::CameraInputState>();
  world.register::<character::controls::CharacterInputState>();

  world.add_resource(Dimensions::new(viewport_size.0, viewport_size.1, hidpi_factor));
  world.add_resource(character::controls::CharacterInputState::new());
  world.add_resource(DeltaTime(0.0));
  world.add_resource(GameTime(0));

  world.create_entity()
    .with(terrain::TerrainDrawable::new())
    .with(character::CharacterDrawable::new())
    .with(CharacterSprite::new())
    .with(camera::CameraInputState::new())
    .with(character::controls::CharacterInputState::new())
    .build();
}

fn dispatch_loop(window_context: &mut WindowContext,
                 w: &mut World) {
//  let (mut device_renderer, encoder_queue) = DeviceRenderer::new(window_context.get_device());
  let draw_system = DrawSystem::new(window_context);

  let (camera_system, camera_control) = CameraControlSystem::new();
  let (character_system, character_control) = CharacterControlSystem::new();
  let controls = TilemapControls::new(camera_control, character_control);

  let mut dispatcher = DispatcherBuilder::new()
    .with(draw_system, "drawing", &[])
    .with(terrain::PreDrawSystem::new(), "draw-prep-terrain", &["drawing"])
    .with(character::PreDrawSystem::new(), "draw-prep-character", &["drawing"])
    .with(camera_system, "terrain-system", &[])
    .build();

  window_context.set_controls(controls);

  let start_time = time::Instant::now();
  let mut last_time = time::Instant::now();

  loop {
    let elapsed = last_time.elapsed();
    let delta = f64::from(elapsed.subsec_nanos()) / 1e9 + elapsed.as_secs() as f64;

    // Throttle update speed
    if delta >= 0.016 {
      last_time = time::Instant::now();
      dispatcher.dispatch(&w.res);
      w.maintain();

      *w.write_resource::<DeltaTime>() = DeltaTime(delta);
      *w.write_resource::<GameTime>() = GameTime(start_time.elapsed().as_secs());
//      device_renderer.draw(window_context.get_device());
    }

    if let WindowStatus::Close = window_context.poll_events() {
      break;
    }
  }
}
