use bullet;
use gfx_app::{Window, GameStatus};
use gfx_app::renderer::{DeviceRenderer, EncoderQueue};
use gfx_app::system::DrawSystem;
use gfx;
use std::time;
use std::sync::mpsc;
use terrain;
use specs;
use specs::{World};
use specs::DispatcherBuilder;
use graphics::{Dimensions, DeltaTime};
use gfx_app::controls::TilemapControls;
use gfx_app::mouse_controls::{MouseControlSystem, MouseInputState};
use graphics::camera::CameraControlSystem;
use character::controls::CharacterControlSystem;
use critter::{CharacterSprite, ZombieSprite};
use character;
use zombie;
use graphics;

pub fn run<W, D, F>(window: &mut W) -> GameStatus
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send {
  let (mut device_renderer, enc_queue) = DeviceRenderer::new(window.create_buffers(2));

  let mut w = specs::World::new();
  setup_world(&mut w, window.get_viewport_size());
  dispatch_loop(window, &mut device_renderer, &mut w, enc_queue)
}

fn setup_world(world: &mut World, viewport_size: (u32, u32)) {
  let view_matrix = Dimensions::get_view_matrix();
  world.register::<terrain::TerrainDrawable>();
  world.register::<graphics::camera::CameraInputState>();
  world.register::<character::CharacterDrawable>();
  world.register::<zombie::ZombieDrawable>();
  world.register::<bullet::BulletDrawable>();
  world.register::<CharacterSprite>();
  world.register::<ZombieSprite>();
  world.register::<character::controls::CharacterInputState>();
  world.register::<MouseInputState>();

  world.add_resource(terrain::tilemap::generate());
  world.add_resource(Dimensions::new(viewport_size.0, viewport_size.1));
  world.add_resource(graphics::camera::CameraInputState::new());
  world.add_resource(character::controls::CharacterInputState::new());
  world.add_resource(MouseInputState::new());
  world.add_resource(DeltaTime(0.0));
  world.create_entity()
    .with(terrain::TerrainDrawable::new(view_matrix))
    .with(character::CharacterDrawable::new(view_matrix))
    .with(zombie::ZombieDrawable::new(view_matrix))
    .with(bullet::BulletDrawable::new())
    .with(CharacterSprite::new())
    .with(ZombieSprite::new())
    .with(graphics::camera::CameraInputState::new())
    .with(character::controls::CharacterInputState::new())
    .with(MouseInputState::new()).build();
}

fn dispatch_loop<W, D, F>(window: &mut W,
                          device_renderer: &mut DeviceRenderer<D>,
                          w: &mut World,
                          encoder_queue: EncoderQueue<D>) -> GameStatus
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send {


  let draw = {
    let rtv = window.get_render_target_view();
    let dsv = window.get_depth_stencil_view();
    DrawSystem::new(window.get_factory(), &rtv, &dsv, encoder_queue)
  };

  let (_, game_state) = mpsc::channel::<GameStatus>();

  let (terrain_system, terrain_control) = CameraControlSystem::new();
  let (character_system, character_control) = CharacterControlSystem::new();
  let (mouse_system, mouse_control) = MouseControlSystem::new();
  let controls = TilemapControls::new(terrain_control, character_control, mouse_control);

  let mut dispatcher = DispatcherBuilder::new()
    .add(draw, "drawing", &[])
    .add(terrain::PreDrawSystem::new(), "draw-prep-terrain", &["drawing"])
    .add(character::PreDrawSystem::new(), "draw-prep-character", &["drawing"])
    .add(zombie::PreDrawSystem::new(), "draw-prep-zombie", &["drawing"])
    .add(bullet::PreDrawSystem::new(), "draw-prep-bullet", &["drawing"])
    .add(terrain_system, "terrain-system", &[])
    .add(character_system, "character-system", &[])
    .add(mouse_system, "mouse-system", &[])
    .build();

  window.set_controls(controls);

  let mut last_time = time::Instant::now();
  loop {
    let elapsed = last_time.elapsed();
    let delta = f64::from(elapsed.subsec_nanos()) / 1e9 + elapsed.as_secs() as f64;
    last_time = time::Instant::now();

    dispatcher.dispatch(&w.res);
    w.maintain();

    *w.write_resource::<DeltaTime>() = DeltaTime(delta);

    device_renderer.draw(window.get_device());
    window.swap_window();

    if let Some(quit_status) = window.poll_events() {
      return quit_status;
    }

    if let Ok(quit_status) = game_state.try_recv() {
      return quit_status;
    }
  }
}
