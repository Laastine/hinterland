use crate::audio::AudioSystem;
use crate::bullet;
use crate::bullet::bullets::Bullets;
use crate::bullet::collision::CollisionSystem;
use crate::character;
use crate::character::controls::CharacterControlSystem;
use crate::critter::CharacterSprite;
use gfx;
use crate::gfx_app::{Window, WindowStatus};
use crate::gfx_app::controls::TilemapControls;
use crate::gfx_app::mouse_controls::{MouseControlSystem, MouseInputState};
use crate::gfx_app::renderer::{DeviceRenderer, EncoderQueue};
use crate::gfx_app::system::DrawSystem;
use crate::graphics;
use crate::graphics::{DeltaTime, dimensions::Dimensions, GameTime};
use crate::graphics::camera::CameraControlSystem;
use crate::hud;
use specs::{Builder, prelude::DispatcherBuilder, world::World};
use std::time;
use crate::terrain;
use crate::terrain_object;
use crate::zombie;
use crate::zombie::zombies::Zombies;

pub fn run<W, D, F>(window: &mut W)
                    where W: Window<D, F>,
                          D: gfx::Device + 'static,
                          F: gfx::Factory<D::Resources>,
                          D::CommandBuffer: Send {
  let (mut device_renderer, enc_queue) = DeviceRenderer::new(window.create_buffers(2));

  let mut w = World::new();
  setup_world(&mut w, window.get_viewport_size(), window.get_hidpi_factor());
  dispatch_loop(window, &mut device_renderer, &mut w, enc_queue);
}

fn setup_world(world: &mut World, viewport_size: (f32, f32), hidpi_factor: f32) {
  world.register::<terrain::TerrainDrawable>();
  world.register::<graphics::camera::CameraInputState>();
  world.register::<character::CharacterDrawable>();
  world.register::<hud::hud_objects::HudObjects>();
  world.register::<terrain_object::terrain_objects::TerrainObjects>();
  world.register::<Zombies>();
  world.register::<Bullets>();
  world.register::<CharacterSprite>();
  world.register::<character::controls::CharacterInputState>();
  world.register::<MouseInputState>();

  world.add_resource(Dimensions::new(viewport_size.0, viewport_size.1, hidpi_factor));
  world.add_resource(character::controls::CharacterInputState::new());
  world.add_resource(MouseInputState::new());
  world.add_resource(DeltaTime(0.0));
  world.add_resource(GameTime(0));

  world.create_entity()
       .with(terrain::TerrainDrawable::new())
       .with(character::CharacterDrawable::new())
       .with(hud::hud_objects::HudObjects::new())
       .with(terrain_object::terrain_objects::TerrainObjects::new())
       .with(Zombies::new())
       .with(Bullets::new())
       .with(CharacterSprite::new())
       .with(graphics::camera::CameraInputState::new())
       .with(character::controls::CharacterInputState::new())
       .with(MouseInputState::new()).build();
}

fn dispatch_loop<W, D, F>(window: &mut W,
                          device_renderer: &mut DeviceRenderer<D>,
                          w: &mut World,
                          encoder_queue: EncoderQueue<D>)
                          where W: Window<D, F>,
                                D: gfx::Device + 'static,
                                F: gfx::Factory<D::Resources>,
                                D::CommandBuffer: Send {
  let draw = {
    let rtv = window.get_render_target_view();
    let dsv = window.get_depth_stencil_view();
    DrawSystem::new(window.get_factory(), &rtv, &dsv, encoder_queue)
  };

  let (audio_system, audio_control) = AudioSystem::new();
  let (terrain_system, terrain_control) = CameraControlSystem::new();
  let (character_system, character_control) = CharacterControlSystem::new();
  let (mouse_system, mouse_control) = MouseControlSystem::new();
  let controls = TilemapControls::new(audio_control, terrain_control, character_control, mouse_control);

  let mut dispatcher = DispatcherBuilder::new()
    .with(draw, "drawing", &[])
    .with(terrain::PreDrawSystem::new(), "draw-prep-terrain", &["drawing"])
    .with(character::PreDrawSystem::new(), "draw-prep-character", &["drawing"])
    .with(zombie::PreDrawSystem::new(), "draw-prep-zombie", &["drawing"])
    .with(bullet::PreDrawSystem::new(), "draw-prep-bullet", &["drawing"])
    .with(hud::PreDrawSystem::new(), "draw-prep-hud", &[])
    .with(terrain_system, "terrain-system", &[])
    .with(terrain_object::PreDrawSystem::new(), "draw-prep-terrain_object", &["terrain-system"])
    .with(character_system, "character-system", &[])
    .with(mouse_system, "mouse-system", &[])
    .with(audio_system, "audio-system", &[])
    .with(CollisionSystem::new(), "collision-system", &["mouse-system"])
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

      device_renderer.draw(window.get_device());

      window.swap_window();
    }

    if let WindowStatus::Close = window.poll_events() {
      break
    }
  }
}
