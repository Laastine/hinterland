use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use physics::Dimensions;
use cgmath;
use cgmath::{Matrix4, Point3, Vector3};
use specs;
use gfx_app::graphics::load_texture;
use character::gfx_macros::{pipe, CharacterData, CharacterSheetSettings, CharacterIdx, VertexData};
use game::gfx_macros::Projection;
use game::constants::{CHARACTER_W, CHARACTER_H, ASPECT_RATIO, VIEW_DISTANCE};
use terrain::controls::InputState;
use data;

pub mod gfx_macros;
pub mod character;

const SHADER_VERT: &'static [u8] = include_bytes!("character.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("character.f.glsl");

impl CharacterData {
  pub fn new(data: [f32; 4]) -> CharacterData {
    CharacterData { data: data }
  }
}

impl VertexData {
  fn new(pos: [f32; 3], buf_pos: [f32; 2]) -> VertexData {
    VertexData {
      pos: pos,
      buf_pos: buf_pos,
    }
  }
}

#[derive(Debug)]
pub struct Drawable {
  projection: Projection,
}

impl Drawable {
  pub fn new() -> Drawable {
    let view: Matrix4<f32> = Matrix4::look_at(
      Point3::new(0.0, 0.0, VIEW_DISTANCE),
      Point3::new(0.0, 0.0, 0.0),
      Vector3::unit_y(),
    );

    let aspect_ratio: f32 = ASPECT_RATIO;

    Drawable {
      projection: Projection {
        model: Matrix4::from(view).into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(60.0f32), aspect_ratio, 0.1, 4000.0).into(),
      }
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection) {
    self.projection = *world_to_clip;
  }
}

impl specs::Component for Drawable {
  type Storage = specs::VecStorage<Drawable>;
}

pub struct DrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
  data: Vec<CharacterData>,
  settings: CharacterSheetSettings,
}

impl<R: gfx::Resources> DrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> DrawSystem<R>
    where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let tilesheet_bytes = &include_bytes!("../../assets/character.png")[..];
    let vertex_data: Vec<VertexData> =
      vec![
        VertexData::new([-40.0, -64.0, 0.0], [0.0, 64.0]),
        VertexData::new([40.0, -64.0, 0.0], [40.0, 64.0]),
        VertexData::new([40.0, 64.0, 0.0], [40.0, 0.0]),
        VertexData::new([-40.0, -64.0, 0.0], [0.0, 64.0]),
        VertexData::new([40.0, 64.0, 0.0], [40.0, 0.0]),
        VertexData::new([-40.0, 64.0, 0.0], [0.0, 0.0]),
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
      character: factory.create_constant_buffer(512),
      character_cb: factory.create_constant_buffer(1),
      charactersheet: (char_texture, factory.create_sampler_linear()),
      character_idx: factory.create_constant_buffer(1),
      out_color: rtv,
      out_depth: dsv,
    };

    let tilesheet_width = 11;
    let tilesheet_height = 19;

    let tilesheet_total_width = tilesheet_width * CHARACTER_W as i32;
    let tilesheet_total_height = tilesheet_height * CHARACTER_H as i32;

    DrawSystem {
      bundle: gfx::Bundle::new(slice, pso, pipeline_data),
      data: data::load_character(),
      settings: CharacterSheetSettings {
        character_size: [CHARACTER_W as f32, CHARACTER_H as f32, CHARACTER_H as f32, 0.0],
        charactersheet_size: [tilesheet_width as f32, tilesheet_height as f32, tilesheet_total_width as f32, tilesheet_total_height as f32],
        offsets: [0.0, 0.0],
      }
    }
  }

  pub fn draw<C>(&mut self,
                 drawable: &Drawable,
                 encoder: &mut gfx::Encoder<R, C>)
    where C: gfx::CommandBuffer<R> {
    encoder.update_buffer(&self.bundle.data.character, &self.data.as_slice(), 0).unwrap();
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
    encoder.update_constant_buffer(&self.bundle.data.character_cb, &self.settings);
    encoder.update_constant_buffer(&self.bundle.data.character_idx, &CharacterIdx {
      idx: 7.0
    });
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
    let (mut character, dim, mut input) =
      arg.fetch(|w| (
        w.write::<Drawable>(),
        w.read_resource::<Dimensions>(),
        w.write::<InputState>()));

    for (c, i) in (&mut character, &mut input).join() {
      let world_to_clip = dim.world_to_projection(i);
      c.update(&world_to_clip);
    }
  }
}
