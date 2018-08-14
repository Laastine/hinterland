use cgmath::Point2;
use character::CharacterDrawable;
use gfx;
use gfx_app::ColorFormat;
use gfx_app::DepthFormat;
use graphics::{mesh::RectangularMesh, texture::{load_raw_texture, text_texture}};
use graphics::texture::Texture;
use rusttype::FontCollection;
use shaders::text_pipeline;
use specs;
use std::collections::HashMap;
use specs::{ReadStorage, WriteStorage};

pub mod font;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/text.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/text.f.glsl");

#[derive(Debug, Clone)]
pub struct TextDrawable {
  text: String,
}

impl<'a> TextDrawable {
  pub fn new(text: &str) -> TextDrawable {
    TextDrawable {
      text: text.to_string(),
    }
  }

  pub fn update(&mut self, new_text: String) {
    self.text = new_text;
  }
}

impl specs::prelude::Component for TextDrawable {
  type Storage = specs::storage::DenseVecStorage<TextDrawable>;
}

pub struct TextDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, text_pipeline::Data<R>>,
  texture_cache: HashMap<String, Texture<R>>,
  pub current_text: String,
}

impl<R: gfx::Resources> TextDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> TextDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let font_bytes = &include_bytes!("../../assets/DejaVuSans.ttf")[..];
    let font = FontCollection::from_bytes(font_bytes as &[u8])
      .unwrap_or_else(|e| panic!("Font loading error: {}", e))
      .into_font().unwrap_or_else(|e| panic!("into_font error: {}", e));

    let mut texture_cache: HashMap<String, Texture<R>> = HashMap::new();

    let texts = vec!["Ammo 0", "Ammo 1", "Ammo 2", "Ammo 3",
                                "Ammo 4", "Ammo 5", "Ammo 6",
                                "Ammo 7", "Ammo 8", "Ammo 9", "Ammo 10"];

    text_texture(factory, &font, texts.as_slice(), &mut texture_cache);

    let pso =
      match factory.create_pipeline_simple(SHADER_VERT, SHADER_FRAG, text_pipeline::new()) {
        Ok(val) => val,
        Err(err) => panic!("HUD shader loading error {:?}", err)
      };

    let current_text = "Ammo 10";
    let texture = texture_cache[current_text].clone();

    let rect_mesh = RectangularMesh::new(factory, texture, Point2::new(1.0, 1.0));

    let pipeline_data = text_pipeline::Data {
      vbuf: rect_mesh.mesh.vertex_buffer,
      position_cb: factory.create_constant_buffer(1),
      text_sheet: (rect_mesh.mesh.texture.raw, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    TextDrawSystem {
      bundle: gfx::Bundle::new(rect_mesh.mesh.slice, pso, pipeline_data),
      texture_cache,
      current_text: current_text.to_string(),
    }
  }

  pub fn draw<C>(&mut self,
                 drawable: &TextDrawable,
                 encoder: &mut gfx::Encoder<R, C>)
                 where C: gfx::CommandBuffer<R> {
    if self.current_text.trim() != drawable.text.trim() {
      self.current_text = drawable.text.to_owned();
      self.bundle.data.text_sheet.0 = self.texture_cache[&drawable.text].raw.clone();
    }
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
  type SystemData = (ReadStorage<'a, CharacterDrawable>,
                     WriteStorage<'a, TextDrawable>);

  fn run(&mut self, (character_drawable, mut text_drawable): Self::SystemData) {
    use specs::join::Join;

    for (cd, td) in (&character_drawable, &mut text_drawable).join() {
      let new_text = format!("Ammo {}", cd.stats.ammunition);
      td.update(new_text);
    }
  }
}
