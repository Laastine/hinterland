use bullet::BulletDrawable;
use bullet::bullets::Bullets;
use cgmath;
use cgmath::{Deg, Point2};
use character::CharacterDrawable;
use character::controls::CharacterInputState;
use critter::CritterData;
use data;
use game::constants::{ASPECT_RATIO, NORMAL_DEATH_SPRITE_OFFSET, SPRITE_OFFSET, ZOMBIE_STILL_SPRITE_OFFSET, ZOMBIE_SHEET_TOTAL_WIDTH};
use game::get_random_bool;
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::{Dimensions, get_orientation, load_texture, overlaps};
use graphics::camera::CameraInputState;
use graphics::orientation::{Orientation, Stance};
use shaders::{CharacterSheet, critter_pipeline, Position, Projection, VertexData};
use specs;
use specs::{Fetch, ReadStorage, WriteStorage};
use zombie::zombies::Zombies;

pub mod zombies;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/character.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/character.f.glsl");

#[derive(Debug, Clone)]
pub struct ZombieDrawable {
  projection: Projection,
  pub position: Position,
  previous_position: Position,
  orientation: Orientation,
  pub stance: Stance,
  direction: Orientation,
  pub movement_direction: Point2<f32>,
  zombie_idx: usize,
  zombie_death_idx: usize,
}

impl ZombieDrawable {
  pub fn new(position: Position) -> ZombieDrawable {
    let view = Dimensions::get_view_matrix();
    ZombieDrawable {
      projection: Projection {
        model: view.into(),
        view: view.into(),
        proj: cgmath::perspective(Deg(60.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      position,
      previous_position: Position::new([0.0, 0.0]),
      orientation: Orientation::Left,
      stance: Stance::Still,
      direction: Orientation::Left,
      movement_direction: Point2::new(0.0, 0.0),
      zombie_idx: 0,
      zombie_death_idx: 0,
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState, c: &CharacterDrawable, bullets: &[BulletDrawable]) {
    self.projection = *world_to_clip;

    let offset_delta =
      Position::new([ci.x_movement - self.previous_position.position[0], ci.y_movement - self.previous_position.position[1]]);

    self.previous_position = Position::new([
      ci.x_movement,
      ci.y_movement
    ]);

    if self.stance != Stance::NormalDeath && self.stance != Stance::CriticalDeath {
      self.direction = get_orientation(
        Point2::new(self.position.position[0], self.position.position[1]),
        Point2::new(c.position.position[0], c.position.position[1]));
    }

    self.position = Position::new([
      self.position.position[0] + offset_delta.position[0] + (self.movement_direction.x),
      self.position.position[1] + offset_delta.position[1] - (self.movement_direction.y)
    ]);
    bullets.iter().for_each(|bullet| {
      if overlaps(self.position, bullet.position, 80.0, 80.0) && self.stance != Stance::NormalDeath && self.stance != Stance::CriticalDeath {
        self.stance =
          if get_random_bool() {
            Stance::NormalDeath
          } else {
            Stance::CriticalDeath
          };
      }
    });
  }

  pub fn update_walk(&mut self) {
    if self.zombie_idx < 7 {
      self.zombie_idx += 1;
    } else {
      self.zombie_idx = 0;
    }
  }

  pub fn update_still(&mut self) {
    if self.zombie_idx < 3 {
      self.zombie_idx += 1;
    } else {
      self.zombie_idx = 0;
    }
  }

  pub fn update_normal_death(&mut self) {
    if self.zombie_death_idx < 5 {
      self.zombie_death_idx += 1;
    }
  }

  pub fn update_critical_death(&mut self) {
    if self.zombie_death_idx < 7 {
      self.zombie_death_idx += 1;
    }
  }
}

pub struct ZombieDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, critter_pipeline::Data<R>>,
  data: Vec<CritterData>,
}

impl<R: gfx::Resources> ZombieDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> ZombieDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let zombie_bytes = include_bytes!("../../assets/zombie.png");

    let vertex_data: Vec<VertexData> =
      vec![
        VertexData::new([-25.0, -35.0], [0.0, 1.0]),
        VertexData::new([25.0, -35.0], [1.0, 1.0]),
        VertexData::new([25.0, 35.0], [1.0, 0.0]),
        VertexData::new([-25.0, 35.0], [0.0, 0.0]),
      ];

    let index_data: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

    let char_texture = load_texture(factory, zombie_bytes).unwrap();
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

    let data = data::load_zombie();

    ZombieDrawSystem {
      bundle: gfx::Bundle::new(slice, pso, pipeline_data),
      data,
    }
  }

  fn get_next_sprite(&self, drawable: &mut ZombieDrawable) -> CharacterSheet {
    let zombie_sprite =
      if drawable.stance == Stance::Still {
        let sprite_idx = (drawable.direction as usize * 4 + drawable.zombie_idx) as usize;
        (&self.data[sprite_idx], sprite_idx)
      } else if drawable.orientation != Orientation::Still && drawable.stance == Stance::Walking {
        let sprite_idx = (drawable.direction as usize * 8 + drawable.zombie_idx + ZOMBIE_STILL_SPRITE_OFFSET) as usize;
        (&self.data[sprite_idx], sprite_idx)
      } else if drawable.orientation != Orientation::Still && drawable.stance == Stance::NormalDeath {
        let sprite_idx = (drawable.direction as usize * 6 + drawable.zombie_death_idx + NORMAL_DEATH_SPRITE_OFFSET) as usize;
        (&self.data[sprite_idx], sprite_idx)
      } else if drawable.orientation != Orientation::Still && drawable.stance == Stance::CriticalDeath {
        let sprite_idx = (drawable.direction as usize * 8 + drawable.zombie_death_idx) as usize;
        (&self.data[sprite_idx], sprite_idx)
      } else {
        drawable.direction = drawable.orientation;
        let sprite_idx = (drawable.orientation as usize * 8 + drawable.zombie_idx + ZOMBIE_STILL_SPRITE_OFFSET) as usize;
        (&self.data[sprite_idx], sprite_idx)
      };

    let (y_div, row_idx) = if drawable.stance == Stance::NormalDeath || drawable.stance == Stance::CriticalDeath {
      (0.0, 2)
    } else {
      (1.0, 2)
    };

    let elements_x = ZOMBIE_SHEET_TOTAL_WIDTH / (zombie_sprite.0.data[2] + SPRITE_OFFSET);
    CharacterSheet {
      x_div: elements_x,
      y_div,
      row_idx,
      index: zombie_sprite.1 as f32,
    }
  }

  pub fn draw<C>(&mut self,
                 mut drawable: &mut ZombieDrawable,
                 encoder: &mut gfx::Encoder<R, C>)
                 where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);
    encoder.update_constant_buffer(&self.bundle.data.character_sprite_cb,
                                   &self.get_next_sprite(&mut drawable));
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

impl<'a> specs::System<'a> for PreDrawSystem {
  #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
  type SystemData = (WriteStorage<'a, Zombies>,
                     ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     ReadStorage<'a, CharacterDrawable>,
                     ReadStorage<'a, Bullets>,
                     Fetch<'a, Dimensions>);


  fn run(&mut self, (mut zombies, camera_input, character_input, character, bullets, dim): Self::SystemData) {
    use specs::Join;

    for (zs, camera, ci, c, bs) in (&mut zombies, &camera_input, &character_input, &character, &bullets).join() {
      let world_to_clip = dim.world_to_projection(camera);

      for z in &mut zs.zombies {
        z.update(&world_to_clip, ci, c, &bs.bullets);
      }
    }
  }
}
