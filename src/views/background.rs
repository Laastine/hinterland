use game::gfx::{CopySprite, Sprite};
use game::data::Rectangle;
use sdl2::render::Renderer;

#[derive(Clone)]
pub struct Background {
  pub pos: f64,
  pub sprite: Sprite,
}

impl Background {
  pub fn render(&mut self, renderer: &mut Renderer) {
    let size = self.sprite.size();

    let (_, window_h) = renderer.output_size().unwrap();
    renderer.copy_sprite(&self.sprite, Rectangle {
      x: 0.0,
      y: 0.0,
      w: size.0,
      h: window_h as f64,
    })
  }
}
