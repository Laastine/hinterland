use std::time;

use specs::{Builder, DispatcherBuilder, World};

use crate::gfx_app::{system::DrawSystem, WindowContext, WindowStatus};
use crate::graphics::{DeltaTime, GameTime, camera};
use crate::terrain;
use crate::graphics::dimensions::Dimensions;
use crate::graphics::camera::CameraControlSystem;
use crate::gfx_app::controls::TilemapControls;

pub fn run(window: &mut WindowContext) {
  let mut w = World::new();
  setup_world(&mut w, window.get_viewport_size(), window.get_hidpi_factor());
  dispatch_loop(window, &mut w);
}

fn setup_world(world: &mut World, viewport_size: (f32, f32), hidpi_factor: f32) {
  world.register::<terrain::TerrainDrawable>();
  world.register::<camera::CameraInputState>();

  world.add_resource(Dimensions::new(viewport_size.0, viewport_size.1, hidpi_factor));
  world.add_resource(DeltaTime(0.0));
  world.add_resource(GameTime(0));

  world.create_entity()
    .with(terrain::TerrainDrawable::new())
    .with(camera::CameraInputState::new())
    .build();
}

fn dispatch_loop(window: &mut WindowContext,
                 w: &mut World) {
  let draw = DrawSystem::new(window.get_window());

  let (terrain_system, terrain_control) = CameraControlSystem::new();
  let controls = TilemapControls::new(terrain_control);

  let mut dispatcher = DispatcherBuilder::new()
    .with(draw, "drawing", &[])
    .with(terrain::PreDrawSystem::new(), "draw-prep-terrain", &["drawing"])
    .with(terrain_system, "terrain-system", &[])
    .build();

  window.set_controls(controls);

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
    }

    if let WindowStatus::Close = window.poll_events() {
      break;
    }
  }
}
