use std::time;

use gfx;
use specs::{Builder, prelude::DispatcherBuilder, shred::World, world::WorldExt};

use crate::{bullet, terrain_shape};
use crate::audio::AudioSystem;
use crate::bullet::bullets::Bullets;
use crate::bullet::collision::CollisionSystem;
use crate::character;
use crate::character::controls::CharacterControlSystem;
use crate::critter::CharacterSprite;
use crate::gfx_app::{Window, WindowStatus};
use crate::gfx_app::controls::TilemapControls;
use crate::gfx_app::mouse_controls::{MouseControlSystem, MouseInputState};
use crate::gfx_app::renderer::DeviceRenderer;
use crate::gfx_app::system::DrawSystem;
use crate::graphics;
use crate::graphics::{DeltaTime, dimensions::Dimensions, GameTime};
use crate::graphics::camera::CameraControlSystem;
use crate::hud;
use crate::terrain;
use crate::terrain_object;
use crate::zombie;
use crate::zombie::zombies::Zombies;
use crate::game::constants::SMALL_HILLS;

pub fn run<W, D, F>(window: &mut W)
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send {

  let mut w = WorldExt::new();
  let viewport_size = window.get_viewport_size();
  let dimensions = Dimensions::new(viewport_size.0,
                                   viewport_size.1,
                                   window.get_hidpi_factor(),
                                   window.is_windowed());
  setup_world(&mut w, dimensions);
  dispatch_loop(window, &mut w);
}

fn setup_world(world: &mut World, dimensions: Dimensions) {
  world.register::<terrain::TerrainDrawable>();
  world.register::<graphics::camera::CameraInputState>();
  world.register::<character::CharacterDrawable>();
  world.register::<hud::hud_objects::HudObjects>();
  world.register::<terrain_object::terrain_objects::TerrainObjects>();
  world.register::<terrain_shape::terrain_shape_objects::TerrainShapeObjects>();
  world.register::<Zombies>();
  world.register::<Bullets>();
  world.register::<CharacterSprite>();
  world.register::<character::controls::CharacterInputState>();
  world.register::<MouseInputState>();

  world.insert(dimensions);
  world.insert(character::controls::CharacterInputState::new());
  world.insert(MouseInputState::new());
  world.insert(DeltaTime(0.0));
  world.insert(GameTime(0));

  let mut hills = terrain_shape::terrain_shape_objects::TerrainShapeObjects::new();

  for hill in SMALL_HILLS.iter() {
    hills.small_hill(hill[0], hill[1]);
  }

  world.create_entity()
    .with(terrain::TerrainDrawable::new())
    .with(character::CharacterDrawable::new())
    .with(hud::hud_objects::HudObjects::new())
    .with(terrain_object::terrain_objects::TerrainObjects::new())
    .with(hills)
    .with(Zombies::new())
    .with(Bullets::new())
    .with(CharacterSprite::new())
    .with(graphics::camera::CameraInputState::new())
    .with(character::controls::CharacterInputState::new())
    .with(MouseInputState::new()).build();
}

fn dispatch_loop<W, D, F>(window: &mut W,
                          w: &mut World)
  where W: Window<D, F>,
        D: gfx::Device + 'static,
        F: gfx::Factory<D::Resources>,
        D::CommandBuffer: Send {
  let (mut device_renderer, encoder_queue) = DeviceRenderer::new(window.create_buffers(2));
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
    .with(terrain::PreDrawSystem, "draw-prep-terrain", &["drawing"])
    .with(character::PreDrawSystem, "draw-prep-character", &["drawing"])
    .with(zombie::PreDrawSystem, "draw-prep-zombie", &["drawing"])
    .with(bullet::PreDrawSystem, "draw-prep-bullet", &["drawing"])
    .with(hud::PreDrawSystem, "draw-prep-hud", &[])
    .with(terrain_system, "terrain-system", &[])
    .with(terrain_object::PreDrawSystem, "draw-prep-terrain_object", &["terrain-system"])
    .with(terrain_shape::PreDrawSystem, "draw-prep-terrain_shape_object", &["terrain-system"])
    .with(character_system, "character-system", &[])
    .with(mouse_system, "mouse-system", &[])
    .with(audio_system, "audio-system", &[])
    .with(CollisionSystem, "collision-system", &["mouse-system"])
    .build();

  window.set_controls(controls);

  let start_time = time::Instant::now();
  let mut last_time = time::Instant::now();
  loop {
    let elapsed = last_time.elapsed();
    let delta = f64::from(elapsed.subsec_nanos()) / 1e9 + elapsed.as_secs() as f64;
    // Throttle update speed
    if delta >= 0.0083 {
      last_time = time::Instant::now();
      dispatcher.dispatch(&w);
      w.maintain();

      *w.write_resource::<DeltaTime>() = DeltaTime(delta);
      *w.write_resource::<GameTime>() = GameTime(start_time.elapsed().as_secs());

      device_renderer.draw(window.get_device());

      window.swap_window();
    }

    if let WindowStatus::Close = window.poll_events() {
      break;
    }
  }
}
