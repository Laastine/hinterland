use gfx;
use gfx_app::ColorFormat;
use gfx_app::DepthFormat;
use graphics::texture::load_raw_texture;
use rusttype::FontCollection;
use shaders::{Position, text_pipeline, VertexData};
use specs;

mod font;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/text.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/text.f.glsl");

#[derive(Debug, Clone)]
pub struct TextDrawable {
  text: String,
  position: Position,
}

impl<'a> TextDrawable {
  pub fn new(text: &str, position: Position) -> TextDrawable {
    TextDrawable {
      text: text.to_string(),
      position
    }
  }
}

impl specs::prelude::Component for TextDrawable {
  type Storage = specs::storage::DenseVecStorage<TextDrawable>;
}

pub struct TextDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, text_pipeline::Data<R>>,
}

impl<R: gfx::Resources> TextDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> TextDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let vertex_data: [VertexData; 4] = [
      VertexData::new([-1.0, -1.0], [0.0, 1.0]),
      VertexData::new([1.0, -1.0], [1.0, 1.0]),
      VertexData::new([1.0, 1.0], [1.0, 0.0]),
      VertexData::new([-1.0, 1.0], [0.0, 0.0]),
    ];

    let index_data: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
    let pso = factory
      .create_pipeline_simple(SHADER_VERT, SHADER_FRAG, text_pipeline::new())
      .unwrap();

    let font_bytes = &include_bytes!("../../assets/DejaVuSans.ttf")[..];
    let font = FontCollection::from_bytes(font_bytes as &[u8])
      .unwrap_or_else(|e| panic!("Font loading error: {}", e))
      .into_font().unwrap_or_else(|e| panic!("into_font error: {}", e));
    let (size, texture_data) = font::draw_text(&font, 100.0, "v0.3.5");

    let text_texture = load_raw_texture(factory, &texture_data[..], size);

    let pipeline_data = text_pipeline::Data {
      vbuf: vertex_buf,
      position_cb: factory.create_constant_buffer(1),
      text_sheet: (text_texture, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    TextDrawSystem {
      bundle: gfx::Bundle::new(slice, pso, pipeline_data),
    }
  }

  pub fn draw<C>(&self,
                 drawable: &TextDrawable,
                 encoder: &mut gfx::Encoder<R, C>)
                 where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);
    self.bundle.encode(encoder);
  }
}
