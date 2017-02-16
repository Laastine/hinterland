use game::gfx::{Sprite, CopySprite, AnimatedSprite};
use game::data::Rectangle;
use game::constants::{ZOMBIE_POS_W, ZOMBIE_POS_H, ZOMBIE_W, ZOMBIE_H, ZOMBIE_PATH};
use sdl2::render::Renderer;
use views::{Orientation};
use data::{load_zombie};

pub struct Zombie {
  pub sprite: AnimatedSprite,
  pub rect: Rectangle,
  pub current: Orientation,
  pub heading: Orientation,
  pub idle_anim_index: u32,
}

impl Zombie {
  pub fn new(renderer: &mut Renderer) -> Zombie {
    Zombie {
      rect: Rectangle {
        x: ZOMBIE_POS_W,
        y: ZOMBIE_POS_H,
        w: ZOMBIE_W,
        h: ZOMBIE_H,
      },
      sprite: Zombie::get_sprites(renderer, 8.0),
      current: Orientation::Down,
      heading: Orientation::Down,
      idle_anim_index: 0,
    }
  }

  fn get_sprites(renderer: &mut Renderer, fps: f64) -> AnimatedSprite {
    let zombie_spritesheet = Sprite::load(&renderer, ZOMBIE_PATH).unwrap();
    let mut zombie_sprites = Vec::with_capacity(512);
    let zombie_datapoints = load_zombie();

    for x in 0..(zombie_datapoints.len() - 1) {
      zombie_sprites.push(zombie_spritesheet.region(zombie_datapoints[x]).unwrap());
    }

    AnimatedSprite::with_fps(zombie_sprites, fps)
  }

  pub fn update(&mut self, dt: f64) {
    self.sprite.add_time(dt);
    self.sprite.set_curr_frames(34, 38);
    self.rect.x += dt * 40.0;
    self.rect.y += dt * 20.0;
  }

  pub fn render(&mut self, renderer: &mut Renderer) {
    renderer.copy_sprite(&self.sprite, self.rect);
  }
}
