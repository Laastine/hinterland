use specs::{Builder, World};

use crate::game::constants::{RESOLUTION_X, RESOLUTION_Y};
use crate::gfx_app::WindowContext;
use crate::terrain;

pub fn run(window: &mut WindowContext) {
  let mut w = World::new();
  setup_world(&mut w, window.get_viewport_size(), window.get_hidpi_factor());
  dispatch_loop();
}

fn setup_world(world: &mut World, viewport_size: (f32, f32), hidpi_factor: f32) {
  world.register::<terrain::TerrainDrawable>();
  world.create_entity()
    .with(terrain::TerrainDrawable::new())
    .build();
}

fn dispatch_loop() {
  unimplemented!()
}
