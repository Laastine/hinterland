use cgmath::{Matrix4, Point3, Vector3};
use cgmath;
use character::character::{CharacterSprite, CritterData};
use character::controls::CharacterInputState;
use graphics::orientation::{Orientation, Stance};
use data;
use game::constants::{ASPECT_RATIO, VIEW_DISTANCE, RUN_SPRITE_OFFSET, CHARSHEET_TOTAL_WIDTH, SPRITE_OFFSET};
use shaders::{pipe, VertexData, CharacterSheet, Position, Projection};
use gfx;
use gfx_app::graphics::load_texture;
use gfx_app::mouse_controls::MouseInputState;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::{Dimensions, get_orientation};
use graphics::camera::CameraInputState;
use specs;

mod audio;
pub mod character;
pub mod controls;

const SHADER_VERT: &'static [u8] = include_bytes!("../shaders/character.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("../shaders/character.f.glsl");

pub struct CharacterDrawable {
  projection: Projection,
  position: Position,
  orientation: Orientation,
  pub stance: Stance,
  direction: Orientation,
  audio: audio::CharacterAudio
}

impl CharacterDrawable {
  pub fn new(view: Matrix4<f32>) -> CharacterDrawable {
    CharacterDrawable {
      projection: Projection {
        model: Matrix4::from(view).into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(60.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      position: Position {
        position: [0.0, 0.0],
      },
      orientation: Orientation::Right,
      stance: Stance::Normal,
      direction: Orientation::Right,
      audio: audio::CharacterAudio::new(),
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &CharacterInputState, cs: &mut CharacterSprite, mouse_input: &mut MouseInputState) {
    self.projection = *world_to_clip;
    let new_position = Position {
      position: [ci.x_movement, ci.y_movement]
    };

    if mouse_input.left_click_point.is_some() {
      self.stance = Stance::Firing;
      self.orientation = get_orientation(mouse_input);
      if cs.character_fire_idx == 1 {
        self.audio.play_pistol();
      }
    } else {
      self.stance = Stance::Normal;
      let dx = new_position.position[0] - self.position.position[0];
      let dy = new_position.position[1] - self.position.position[1];
      self.orientation =
        if dx == 0.0 && dy < 0.0       { Orientation::Down }
        else if dx > 0.0 && dy < 0.0   { Orientation::DownRight }
        else if dx < 0.0 && dy < 0.0   { Orientation::DownLeft }
        else if dx == 0.0 && dy == 0.0 { Orientation::Still }
        else if dx > 0.0 && dy == 0.0  { Orientation::Right }
        else if dx < 0.0 && dy == 0.0  { Orientation::Left }
        else if dx == 0.0 && dy > 0.0  { Orientation::Up }
        else if dx > 0.0 && dy > 0.0   { Orientation::UpRight }
        else if dx < 0.0 && dy > 0.0   { Orientation::UpLeft }
        else { unreachable!() };
      self.position = new_position;
    }
  }
}

impl specs::Component for CharacterDrawable {
  type Storage = specs::VecStorage<CharacterDrawable>;
}

pub struct CharacterDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
  data: Vec<CritterData>,
}

impl<R: gfx::Resources> CharacterDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> CharacterDrawSystem<R>
    where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let tilesheet_bytes = &include_bytes!("../../assets/character.png")[..];

    let vertex_data: Vec<VertexData> =
      vec![
        VertexData::new([-20.0, -28.0, 0.0], [0.0, 1.0]),
        VertexData::new([20.0, -28.0, 0.0], [1.0, 1.0]),
        VertexData::new([20.0, 28.0, 0.0], [1.0, 0.0]),
        VertexData::new([-20.0, -28.0, 0.0], [0.0, 1.0]),
        VertexData::new([20.0, 28.0, 0.0], [1.0, 0.0]),
        VertexData::new([-20.0, 28.0, 0.0], [0.0, 0.0]),
      ];

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());

    let char_texture = load_texture(factory, tilesheet_bytes).unwrap();
    let pso = factory
      .create_pipeline_simple(SHADER_VERT,
                              SHADER_FRAG,
                              pipe::new())
      .unwrap();

    let pipeline_data = pipe::Data {
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
      data
    }
  }

  fn get_next_sprite(&self, character_idx: usize, character_fire_idx: usize, drawable: &mut CharacterDrawable) -> CharacterSheet {
    let char_sprite = if drawable.orientation == Orientation::Still && drawable.stance == Stance::Normal {
      let sprite_idx = (drawable.direction as usize * 28 + RUN_SPRITE_OFFSET) as usize;
      (&self.data[sprite_idx], sprite_idx)
    } else if drawable.stance == Stance::Normal {
      drawable.direction = drawable.orientation;
      let sprite_idx = (drawable.orientation as usize * 28 + character_idx + RUN_SPRITE_OFFSET) as usize;
      (&self.data[sprite_idx], sprite_idx)
    } else {
      let sprite_idx = (drawable.orientation as usize * 8 + character_fire_idx) as usize;
      (&self.data[sprite_idx], sprite_idx)
    };

    let elements_x = CHARSHEET_TOTAL_WIDTH / (char_sprite.0.data[2] + SPRITE_OFFSET);
    CharacterSheet {
      div: elements_x,
      index: char_sprite.1 as f32
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
                                   &mut self.get_next_sprite(character.character_idx,
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

impl<C> specs::System<C> for PreDrawSystem {
  fn run(&mut self, arg: specs::RunArg, _: C) {
    use specs::Join;
    let (mut character, dim, mut terrain_input, mut character_input, mut character_sprite, mut mouse_input) =
      arg.fetch(|w| (
        w.write::<CharacterDrawable>(),
        w.read_resource::<Dimensions>(),
        w.write::<CameraInputState>(),
        w.write::<CharacterInputState>(),
        w.write::<CharacterSprite>(),
        w.write::<MouseInputState>()));

    for (c, ti, ci, cs, mi) in (&mut character, &mut terrain_input, &mut character_input, &mut character_sprite, &mut mouse_input).join() {
      let world_to_clip = dim.world_to_projection(ti);
      c.update(&world_to_clip, ci, cs, mi);
    }
  }
}
