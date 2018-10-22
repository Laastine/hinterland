use cgmath::Point2;
use character::{character_stats::CharacterStats, controls::CharacterInputState};
use critter::{CharacterSprite, CritterData};
use data;
use game::constants::{AMMO_POSITIONS, ASPECT_RATIO, CHARACTER_SHEET_TOTAL_WIDTH, RUN_SPRITE_OFFSET, SPRITE_OFFSET, VIEW_DISTANCE};
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use gfx_app::mouse_controls::MouseInputState;
use graphics::{camera::CameraInputState, dimensions::{Dimensions, get_projection, get_view_matrix}, get_orientation_from_center, orientation::{Orientation, Stance}, overlaps, texture::load_texture};
use graphics::mesh::RectangularMesh;
use graphics::texture::Texture;
use shaders::{CharacterSheet, critter_pipeline, Position, Projection};
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};
use std;
use terrain_object::{terrain_objects::TerrainObjects, TerrainObjectDrawable, TerrainTexture};
use zombie::{ZombieDrawable, zombies::Zombies};

pub mod controls;
mod character_stats;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/character.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/character.f.glsl");

#[derive(Clone)]
pub struct CharacterDrawable {
  pub stats: CharacterStats,
  projection: Projection,
  pub position: Position,
  orientation: Orientation,
  pub stance: Stance,
  direction: Orientation,
}

impl CharacterDrawable {
  pub fn new() -> CharacterDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    let stats = CharacterStats::new();
    CharacterDrawable {
      stats,
      projection,
      position: Position::origin(),
      orientation: Orientation::Right,
      stance: Stance::Walking,
      direction: Orientation::Right,
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState, mouse_input: &MouseInputState,
                dimensions: &Dimensions, objs: &mut Vec<TerrainObjectDrawable>, zombies: &[ZombieDrawable]) {
    self.projection = *world_to_clip;

    fn zombie_not_dead(z: &ZombieDrawable) -> bool {
      z.stance != Stance::NormalDeath &&
        z.stance != Stance::CriticalDeath
    }

    for idx in 0..AMMO_POSITIONS.len() {
      self.ammo_pick_up(ci.movement, objs, idx);
    }

    if !cfg!(feature = "godmode") &&
      zombies.iter()
             .any(|z|
               zombie_not_dead(z) &&
                 overlaps(ci.movement,
                          ci.movement - z.position,
                          15.0,
                          30.0)) {
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

  fn ammo_pick_up(&mut self, movement: Position,  objs: &mut Vec<TerrainObjectDrawable>, idx: usize) {
    if objs[idx].object_type == TerrainTexture::Ammo && overlaps(movement, movement - objs[idx].position, 20.0, 20.0) {
      self.stats.magazines = 2;
      objs.remove(idx);
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
    let char_texture = load_texture(factory, charter_bytes);

    let rect_mesh =
      RectangularMesh::new(factory, Texture::new(char_texture, None), Point2::new(20.0, 28.0));

    let pso = factory.create_pipeline_simple(SHADER_VERT, SHADER_FRAG, critter_pipeline::new())
                     .map_err(|err| panic!("Character shader loading error {:?}", err))
                     .unwrap();

    let pipeline_data = critter_pipeline::Data {
      vbuf: rect_mesh.mesh.vertex_buffer,
      projection_cb: factory.create_constant_buffer(1),
      position_cb: factory.create_constant_buffer(1),
      character_sprite_cb: factory.create_constant_buffer(1),
      charactersheet: (rect_mesh.mesh.texture.raw, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    let data = data::load_character();

    CharacterDrawSystem {
      bundle: gfx::Bundle::new(rect_mesh.mesh.slice, pso, pipeline_data),
      data,
    }
  }

  fn get_next_sprite(&self, character_idx: usize, character_fire_idx: usize, drawable: &mut CharacterDrawable) -> CharacterSheet {
    let sprite_idx =
      if drawable.orientation == Orientation::Still && drawable.stance == Stance::Walking {
        (drawable.direction as usize * 28 + RUN_SPRITE_OFFSET)
      } else if drawable.stance == Stance::Walking {
        drawable.direction = drawable.orientation;
        (drawable.orientation as usize * 28 + character_idx + RUN_SPRITE_OFFSET)
      } else {
        (drawable.orientation as usize * 8 + character_fire_idx)
      } as usize;

    let elements_x = CHARACTER_SHEET_TOTAL_WIDTH / (self.data[sprite_idx].data[2] + SPRITE_OFFSET);
    CharacterSheet {
      x_div: elements_x,
      y_div: 0.0,
      row_idx: 0,
      index: sprite_idx as f32,
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
  type SystemData = (WriteStorage<'a, CharacterDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     ReadStorage<'a, CharacterInputState>,
                     ReadStorage<'a, MouseInputState>,
                     WriteStorage<'a, TerrainObjects>,
                     ReadStorage<'a, Zombies>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (mut character, camera_input, character_input, mouse_input, mut terrain_objects, zombies, dim): Self::SystemData) {
    use specs::join::Join;

    for (c, camera, ci, mi, to, zs) in (&mut character, &camera_input, &character_input, &mouse_input, &mut terrain_objects, &zombies).join() {
      let world_to_clip = dim.world_to_projection(camera);
      c.update(&world_to_clip, ci, mi, &dim, &mut to.objects, &zs.zombies);
    }
  }
}
