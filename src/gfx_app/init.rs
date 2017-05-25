use gfx_app::{GlutinWindow, Window, GameStatus};
use gfx_app::renderer::{DeviceRenderer, EncoderQueue};
use gfx_app::system::{DrawSystem};
use gfx;
use std::time;
use std::sync::mpsc;
use terrain;
use specs;
use specs::{Planner};
use terrain::gfx_macros;

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

//  let dimensions = Dimensions::new(viewport_size.0, viewport_size.1);
//  world.add_resource(terrain::generate(&dimensions, 10));
//  world.add_resource(dimensions);
//  world.add_resource()
  world.add_resource(terrain::terrain::generate());
  world.create().with(terrain::Drawable::new()).build();
}

fn setup_planner<W, D, F>(window: &mut W,
                          planner: &mut Planner<f32>,
                          encoder_queue: EncoderQueue<D>)
                          -> mpsc::Receiver<GameStatus>
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send
{
  let draw = {
    let rtv = window.get_render_target_view();
    let dsv = window.get_depth_stencil_view();
    DrawSystem::new(window.get_factory(), rtv, dsv, encoder_queue)
  };

  let (tx, rx) = mpsc::channel();

  planner.add_system(draw, "drawing", 10);
  planner.add_system(terrain::PreDrawSystem::new(), "draw-prep-terrain", 15);

  rx
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
