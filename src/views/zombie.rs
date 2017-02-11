use game::gfx::{Sprite, CopySprite};
use game::data::Rectangle;
use game::constants::{ZOMBIE_POS_W, ZOMBIE_POS_H, ZOMBIE_W, ZOMBIE_H};
use sdl2::render::Renderer;
use views::{Orientation};

pub struct Zombie {
  pub rect: Rectangle,
  pub sprites: Vec<Sprite>,
  pub current: Orientation,
  pub heading: Orientation,
  pub idle_anim_index: u32,
}

impl Zombie {
  pub fn new(sprites: Vec<Sprite>) -> Zombie {
    Zombie {
      rect: Rectangle {
        x: ZOMBIE_POS_W,
        y: ZOMBIE_POS_H,
        w: ZOMBIE_W,
        h: ZOMBIE_H,
      },
      sprites: sprites.clone(),
      current: Orientation::Down,
      heading: Orientation::Down,
      idle_anim_index: 0,
    }
  }

  pub fn render(&mut self, renderer: &mut Renderer) {
    self.idle_anim_index = if self.idle_anim_index < 3 { self.idle_anim_index + 1 } else { 0 };
    renderer.copy_sprite(&self.sprites[self.idle_anim_index as usize], self.rect);
  }
}
