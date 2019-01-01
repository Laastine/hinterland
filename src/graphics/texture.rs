use std::collections::HashMap;
use std::hash::BuildHasher;
use std::io::Cursor;

use cgmath::Point2;
use gfx::{Factory, format::Rgba8, handle::ShaderResourceView, Resources, texture::{AaMode, Kind, Mipmap, Size}};
use image;
use rusttype::Font;

use crate::gfx_app::ColorFormat;
use crate::hud::font::draw_text;

#[derive(Clone)]
pub struct Texture<R> where R: Resources {
  pub raw: ShaderResourceView<R, [f32; 4]>,
  pub size: Point2<i32>,
}

impl<R> Texture<R> where R: Resources {
  pub fn new(raw: ShaderResourceView<R, [f32; 4]>, size: Option<Point2<i32>>) -> Texture<R> {
    Texture {
      raw,
      size: size.map_or(Point2::new(1, 1), |e| e),
    }
  }
}

pub fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> ShaderResourceView<R, [f32; 4]> where R: Resources, F: Factory<R> {
  let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = Kind::D2(width as Size, height as Size, AaMode::Single);
  match factory.create_texture_immutable_u8::<Rgba8>(kind, Mipmap::Provided, &[&img]) {
    Ok(val) => val.1,
    Err(e) => panic!("Couldn't load texture {:?}", e)
  }
}

pub fn load_raw_texture<R, F>(factory: &mut F, data: &[u8], size: Point2<i32>) -> ShaderResourceView<R, [f32; 4]>
  where R: Resources, F: Factory<R> {
  let kind = Kind::D2(size.x as Size, size.y as Size, AaMode::Single);
  let mipmap = Mipmap::Provided;
  match factory
    .create_texture_immutable_u8::<ColorFormat>(kind, mipmap, &[data]) {
    Ok(val) => val.1,
    Err(e) => panic!("Couldn't load raw texture {:?}", e)
  }
}

pub fn text_texture<'a, R, F, S: BuildHasher>(factory: &mut F,
                                              font: &Font,
                                              texts: &[&str],
                                              texture_cache: &'a mut HashMap<String, Texture<R>, S>)
                                              -> &'a mut HashMap<String, Texture<R>, S>
  where R: Resources, F: Factory<R> {
  let text_texture_height = 100.0;
  texts.iter().for_each(|text| {
    let (texture_size, texture_data) = draw_text(&font, text_texture_height, text);
    let texture = load_raw_texture(factory, &texture_data.as_slice(), texture_size);
    let texture_element = Texture::new(texture, None);
    texture_cache.insert(text.to_string(), texture_element.clone());
  });
  texture_cache
}
