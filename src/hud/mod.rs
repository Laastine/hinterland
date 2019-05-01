use std::collections::HashMap;

use cgmath::Point2;
use gfx;
use rusttype::FontCollection;
use specs;
use specs::{ReadStorage, WriteStorage};

use crate::character::CharacterDrawable;
use crate::gfx_app::ColorFormat;
use crate::gfx_app::DepthFormat;
use crate::graphics::{mesh::RectangularMesh};
use crate::graphics::texture::{text_texture, Texture};
use crate::shaders::{Position, text_pipeline};

pub mod font;
pub mod hud_objects;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/text.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/text.f.glsl");

pub struct TextDrawable {
  text: String,
  position: Position,
}

impl<'a> TextDrawable {
  pub fn new(text: &str, position: Position) -> TextDrawable {
    TextDrawable {
      text: text.to_string(),
      position,
    }
  }

  pub fn update(&mut self, new_text: String) {
    self.text = new_text;
  }
}

impl specs::prelude::Component for TextDrawable {
  type Storage = specs::storage::HashMapStorage<TextDrawable>;
}

pub struct TextDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, text_pipeline::Data<R>>,
  texture_cache: HashMap<String, Texture<R>>,
  pub current_text: String,
}

impl<R: gfx::Resources> TextDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                texts: &[&str],
                current_text: &str,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>) -> TextDrawSystem<R>
    where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let font_bytes = &include_bytes!("../../assets/DejaVuSans.ttf")[..];
    let font = FontCollection::from_bytes(font_bytes as &[u8])
      .unwrap_or_else(|e| panic!("Font loading error: {}", e))
      .into_font().unwrap_or_else(|e| panic!("into_font error: {}", e));

    let mut texture_cache: HashMap<String, Texture<R>> = HashMap::new();

    text_texture(factory, &font, texts, &mut texture_cache);

    let pso = factory.create_pipeline_simple(SHADER_VERT, SHADER_FRAG, text_pipeline::new())
      .expect("HUD shader loading error");

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
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);
    if self.current_text.trim() != drawable.text.trim() {
      self.current_text = drawable.text.to_owned();
      self.bundle.data.text_sheet.0 = self.texture_cache[&drawable.text].raw.clone();
    }
    self.bundle.encode(encoder);
  }
}

pub struct PreDrawSystem;

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (ReadStorage<'a, CharacterDrawable>,
                     WriteStorage<'a, hud_objects::HudObjects>);

  fn run(&mut self, (character_drawable, mut hud_objects): Self::SystemData) {
    use specs::join::Join;

    for (cd, huds) in (&character_drawable, &mut hud_objects).join() {
      let new_ammo_text = format!("Ammo {}", cd.stats.ammunition);
      let new_mag_text = format!("Magazines {}/2", cd.stats.magazines);
      huds.objects[1].update(new_ammo_text);
      huds.objects[2].update(new_mag_text);
    }
  }
}
