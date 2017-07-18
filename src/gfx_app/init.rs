use gfx_app::{Window, GameStatus};
use gfx_app::renderer::{DeviceRenderer, EncoderQueue};
use gfx_app::system::DrawSystem;
use gfx;
use std::time;
use std::sync::mpsc;
use terrain;
use specs;
use physics::{Dimensions, Planner};
use gfx_app;

pub fn run<W, D, F>(window: &mut W) -> GameStatus
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send {
  let (mut device_renderer, enc_queue) = DeviceRenderer::new(window.create_buffers(2));

  let mut w = specs::World::new();
  setup_world(&mut w, window.get_viewport_size());
  let mut plan = specs::Planner::new(w);
  let mut receiver = setup_planner(window, &mut plan, enc_queue);

  dispatch_loop(window, &mut device_renderer, plan, &mut receiver)
}

fn setup_world(world: &mut specs::World, viewport_size: (u32, u32)) {
  world.register::<terrain::Drawable>();
  world.register::<terrain::controls::InputState>();

  let dimensions = Dimensions::new(viewport_size.0, viewport_size.1);
  world.add_resource(terrain::terrain::generate());
  world.add_resource(dimensions);
  world.add_resource(terrain::controls::InputState::new());
  world.create()
    .with(terrain::Drawable::new())
    .with(terrain::controls::InputState::new()).build();
}

fn setup_planner<W, D, F>(window: &mut W, planner: &mut Planner, encoder_queue: EncoderQueue<D>)
                          -> mpsc::Receiver<GameStatus>
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send {
  let draw = {
    let rtv = window.get_render_target_view();
    let dsv = window.get_depth_stencil_view();
    DrawSystem::new(window.get_factory(), rtv, dsv, encoder_queue)
  };

  let (_, rx) = mpsc::channel();

  planner.add_system(draw, "drawing", 10);
  planner.add_system(terrain::PreDrawSystem::new(), "draw-prep-terrain", 15);

  let map_control = create_controls(planner);
  window.set_controls(map_control);

  rx
}

fn create_controls(planner: &mut Planner) -> gfx_app::controls::TilemapControls {
  let (terrain_system, terrain_control) = terrain::controls::TerrainControlSystem::new();
  let controls = gfx_app::controls::TilemapControls::new(terrain_control);
  planner.add_system(terrain_system, "terrain-system", 20);
  controls
}

fn dispatch_loop<W, D, F>(window: &mut W,
                          device_renderer: &mut DeviceRenderer<D>,
                          mut planner: specs::Planner<f32>,
                          game_state: &mut mpsc::Receiver<GameStatus>)
                          -> GameStatus
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send
{
  let mut last_time = time::Instant::now();
  loop {
    let elapsed = last_time.elapsed();
    let delta = elapsed.subsec_nanos() as f32 / 1e9 + elapsed.as_secs() as f32;
    last_time = time::Instant::now();

    planner.dispatch(delta);

    device_renderer.draw(window.get_device());
    window.swap_window();

    if let Some(quit_status) = window.poll_events() {
      return quit_status;
    }
    planner.wait();
    if let Ok(quit_status) = game_state.try_recv() {
      return quit_status;
    }
  }
}
