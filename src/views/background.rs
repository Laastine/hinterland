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
    renderer.copy_sprite(&self.sprite, Rectangle {
      x: -40.0,
      y: -40.0,
      w: size.0 * 10.0,
      h: size.0 * 10.0,
    })
  }
}
