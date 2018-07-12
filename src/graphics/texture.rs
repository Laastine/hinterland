use cgmath::Point2;
use gfx::{Factory, format::Rgba8, handle::ShaderResourceView, Resources, texture};
use gfx_app::ColorFormat;
use image;
use std::io::Cursor;

pub fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> ShaderResourceView<R, [f32; 4]> where R: Resources, F: Factory<R> {
  let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = texture::Kind::D2(width as texture::Size, height as texture::Size, texture::AaMode::Single);
  match factory.create_texture_immutable_u8::<Rgba8>(kind, texture::Mipmap::Provided, &[&img]) {
    Ok(val) => val.1,
    Err(e) => panic!("Couldn't load texture {:?}", e)
  }
}

pub fn load_raw_texture<R, F>(factory: &mut F, data: &[u8], size: Point2<i32>) -> ShaderResourceView<R, [f32; 4]>
                              where R: Resources, F: Factory<R> {
  let kind = texture::Kind::D2(size.x as texture::Size, size.y as texture::Size, texture::AaMode::Single);
  let mipmap = texture::Mipmap::Provided;
  match factory
    .create_texture_immutable_u8::<ColorFormat>(kind, mipmap, &[data]) {
    Ok(val) => val.1,
    Err(e) => panic!("Couldn't load texture {:?}", e)
  }
}