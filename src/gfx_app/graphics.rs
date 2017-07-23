use gfx::handle::ShaderResourceView;
use gfx::{texture, Factory, Resources};
use image;
use std::io::Cursor;
use gfx::format::Rgba8;

pub fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> Result<ShaderResourceView<R, [f32; 4]>, String> where R: Resources, F: Factory<R> {
  let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = texture::Kind::D2(width as texture::Size, height as texture::Size, texture::AaMode::Single);
  let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
  Ok(view)
}
