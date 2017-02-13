use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use sdl2::render::{Renderer, Texture};
use sdl2::image::{LoadTexture};

use game::data::Rectangle;

pub trait Renderable {
  fn render(&self, renderer: &mut Renderer, dest: Rectangle);
}

#[derive(Clone)]
pub struct Sprite {
  texture: Rc<RefCell<Texture>>,
  src: Rectangle,
}

impl Renderable for Sprite {
  fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
    renderer.copy(&mut self.texture.borrow_mut(), self.src.to_sdl(), dest.to_sdl()).unwrap();
  }
}

impl Sprite {
  pub fn new(texture: Texture) -> Sprite {
    let tex_query = texture.query();

    Sprite {
      texture: Rc::new(RefCell::new(texture)),
      src: Rectangle {
        w: tex_query.width as f64,
        h: tex_query.height as f64,
        x: 0.0,
        y: 0.0,
      }
    }
  }

  pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
    match renderer.load_texture(Path::new(path)).ok() {
      Some(f) => Some(f).map(Sprite::new),
      None => panic!("File not fount {}", path),
    }
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
        texture: self.texture.clone(),
        src: new_src,
      })
    } else {
      None
    }
  }

  pub fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
    renderer.copy(&mut self.texture.borrow_mut(), self.src.to_sdl(), dest.to_sdl()).unwrap();
  }
}

#[derive(Clone)]
pub struct AnimatedSprite {
  sprites: Vec<Sprite>,
  start_index: usize,
  end_index: usize,
  frame_delay: f64,
  current_time: f64,
}

impl AnimatedSprite {
  pub fn new(sprites: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
    AnimatedSprite {
      sprites: sprites,
      frame_delay: frame_delay,
      start_index: 0,
      end_index: 1,
      current_time: 0.0,
    }
  }

  pub fn with_fps(sprites: Vec<Sprite>, fps: f64) -> AnimatedSprite {
    if fps <= 0.0 {
      panic!("Passed non positive value to AnimatedSprite::with_fps");
    }

    AnimatedSprite::new(sprites, 1.0 / fps)
  }

  pub fn set_curr_frames(&mut self, start_index: usize, end_index: usize) {
    self.start_index = start_index;
    self.end_index = end_index;
  }

  pub fn frames(&self) -> usize {
    self.end_index - self.start_index
  }

  pub fn add_time(&mut self, dt: f64) {
    self.current_time += dt;
    if self.current_time < 0.0 {
      self.current_time = (self.frames() - 1) as f64 * self.frame_delay;
    }
  }
}

impl Renderable for AnimatedSprite {
  fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
    let current_frame =
    (self.current_time / self.frame_delay) as usize % self.frames();
    let curr_sprites = &self.sprites[self.start_index..self.end_index];
    let sprite = &curr_sprites[current_frame];
    sprite.render(renderer, dest);
  }
}

pub trait CopySprite<T> {
  fn copy_sprite(&mut self, sprite: &T, dest: Rectangle);
}

impl<'w, T: Renderable> CopySprite<T> for Renderer<'w> {
  fn copy_sprite(&mut self, renderable: &T, dest: Rectangle) {
    renderable.render(self, dest);
  }
}
