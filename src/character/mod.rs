use cgmath;
use character::controls::CharacterInputState;
use critter::{CharacterSprite, CritterData};
use data;
use game::constants::{ASPECT_RATIO, CHARACTER_SHEET_TOTAL_WIDTH, RUN_SPRITE_OFFSET, SPRITE_OFFSET};
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use gfx_app::mouse_controls::MouseInputState;
use graphics::{camera::CameraInputState, Dimensions, get_orientation_from_center, load_texture, orientation::{Orientation, Stance}, overlaps};
use shaders::{CharacterSheet, critter_pipeline, Position, Projection, VertexData};
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};
use std;
use zombie::{ZombieDrawable, zombies::Zombies};

pub mod controls;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/character.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/character.f.glsl");

#[derive(Clone)]
pub struct CharacterDrawable {
  projection: Projection,
  pub position: Position,
  orientation: Orientation,
  pub stance: Stance,
  direction: Orientation,
}

impl CharacterDrawable {
  pub fn new() -> CharacterDrawable {
    let view = Dimensions::get_view_matrix();
    CharacterDrawable {
      projection: Projection {
        model: view.into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(75.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      position: Position::new([0.0, 0.0]),
      orientation: Orientation::Right,
      stance: Stance::Walking,
      direction: Orientation::Right,
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState, mouse_input: &MouseInputState, dimensions: &Dimensions, zombies: &[ZombieDrawable]) {
    self.projection = *world_to_clip;

    fn zombie_not_dead(z: &ZombieDrawable) -> bool {
      z.stance != Stance::NormalDeath &&
      z.stance != Stance::CriticalDeath
    }

    if zombies.iter()
              .any(|z|
                zombie_not_dead(z) &&
                  overlaps(Position::new([ci.x_movement, ci.y_movement]),
                           Position::new([ci.x_movement - z.position.position[0], ci.y_movement - z.position.position[1]]),
                           10.0,
                           20.0)) {
      self.stance = Stance::NormalDeath;
      println!("Player died");
      std::process::exit(0);
    }

    if self.stance != Stance::NormalDeath {
      if ci.is_shooting && mouse_input.left_click_point.is_some() && !ci.is_colliding {
        self.stance = Stance::Firing;
        self.orientation = get_orientation_from_center(mouse_input, dimensions);
      } else if ci.is_colliding {
        self.stance = Stance::Still;
      } else {
        self.stance = Stance::Walking;
        self.orientation = ci.orientation;
      }
    }
  }
}

impl Default for CharacterDrawable {
  fn default() -> Self {
    CharacterDrawable::new()
  }
}

impl specs::prelude::Component for CharacterDrawable {
  type Storage = specs::storage::VecStorage<CharacterDrawable>;
}

pub struct CharacterDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, critter_pipeline::Data<R>>,
  data: Vec<CritterData>,
}

impl<R: gfx::Resources> CharacterDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> CharacterDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let charter_bytes = &include_bytes!("../../assets/character.png")[..];

    let vertex_data: [VertexData; 4] = [
      VertexData::new([-20.0, -28.0], [0.0, 1.0]),
      VertexData::new([20.0, -28.0], [1.0, 1.0]),
      VertexData::new([20.0, 28.0], [1.0, 0.0]),
      VertexData::new([-20.0, 28.0], [0.0, 0.0]),
    ];

    let index_data: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

    let char_texture = load_texture(factory, charter_bytes);
    let pso = factory
      .create_pipeline_simple(SHADER_VERT,
                              SHADER_FRAG,
                              critter_pipeline::new())
      .unwrap();

    let pipeline_data = critter_pipeline::Data {
      vbuf: vertex_buf,
      projection_cb: factory.create_constant_buffer(1),
      position_cb: factory.create_constant_buffer(1),
      character_sprite_cb: factory.create_constant_buffer(1),
      charactersheet: (char_texture, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    let data = data::load_character();

    CharacterDrawSystem {
      bundle: gfx::Bundle::new(slice, pso, pipeline_data),
      data,
    }
  }

  fn get_next_sprite(&self, character_idx: usize, character_fire_idx: usize, drawable: &mut CharacterDrawable) -> CharacterSheet {
    let char_sprite =
      if drawable.orientation == Orientation::Still && drawable.stance == Stance::Walking {
        let sprite_idx = (drawable.direction as usize * 28 + RUN_SPRITE_OFFSET) as usize;
        (&self.data[sprite_idx], sprite_idx)
      } else if drawable.stance == Stance::Walking {
        drawable.direction = drawable.orientation;
        let sprite_idx = (drawable.orientation as usize * 28 + character_idx + RUN_SPRITE_OFFSET) as usize;
        (&self.data[sprite_idx], sprite_idx)
      } else {
        let sprite_idx = (drawable.orientation as usize * 8 + character_fire_idx) as usize;
        (&self.data[sprite_idx], sprite_idx)
      };

    let elements_x = CHARACTER_SHEET_TOTAL_WIDTH / (char_sprite.0.data[2] + SPRITE_OFFSET);
    CharacterSheet {
      x_div: elements_x,
      y_div: 0.0,
      row_idx: 0,
      index: char_sprite.1 as f32,
    }
  }

  pub fn draw<C>(&mut self,
                 mut drawable: &mut CharacterDrawable,
                 character: &CharacterSprite,
                 encoder: &mut gfx::Encoder<R, C>)
                 where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);
    encoder.update_constant_buffer(&self.bundle.data.character_sprite_cb,
                                   &self.get_next_sprite(character.character_idx,
                                                         character.character_fire_idx,
                                                         &mut drawable));
    self.bundle.encode(encoder);
  }
}

#[derive(Debug)]
pub struct PreDrawSystem;

impl PreDrawSystem {
  pub fn new() -> PreDrawSystem {
    PreDrawSystem {}
  }
}

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
  type SystemData = (WriteStorage<'a, CharacterDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     ReadStorage<'a, MouseInputState>,
                     ReadStorage<'a, Zombies>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (mut character, camera_input, character_input, mouse_input, zombies, dim): Self::SystemData) {
    use specs::join::Join;

    for (c, camera, ci, mi, zs) in (&mut character, &camera_input, &character_input, &mouse_input, &zombies).join() {
      let world_to_clip = dim.world_to_projection(camera);
      c.update(&world_to_clip, ci, mi, &dim, &zs.zombies);
    }
  }
}
