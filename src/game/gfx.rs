use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use sdl2::render::{Renderer, Texture};
use sdl2::image::{LoadTexture};

use game::data::Rectangle;

#[derive(Clone)]
pub struct Sprite {
  tex: Rc<RefCell<Texture>>,
  src: Rectangle,
}

impl Sprite {
  pub fn new(texture: Texture) -> Sprite {
    let tex_query = texture.query();

    Sprite {
      tex: Rc::new(RefCell::new(texture)),
      src: Rectangle {
        w: tex_query.width as f64,
        h: tex_query.height as f64,
        x: 0.0,
        y: 0.0,
      }
    }
  }

  pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
    renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
  }

  pub fn size(&self) -> (f64, f64) {
    (self.src.w, self.src.h)
  }

  pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
    let new_src = Rectangle {
      x: rect.x + self.src.x,
      y: rect.y + self.src.y,
      ..rect
    };

    if self.src.contains(new_src) {
      Some(Sprite {
        tex: self.tex.clone(),
        src: new_src,
      })
    } else {
      None
    }
  }

  pub fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
    renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl()).unwrap();
  }
}

pub trait CopySprite {
  fn copy_sprite(&mut self, sprite: &Sprite, dest: Rectangle);
}

impl<'window> CopySprite for Renderer<'window> {
  fn copy_sprite(&mut self, sprite: &Sprite, dest: Rectangle) {
   sprite.render(self, dest);
 }
}
