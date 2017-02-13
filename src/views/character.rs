use sdl2::render::Renderer;
use game::gfx::{Sprite, CopySprite, AnimatedSprite};
use game::data::Rectangle;
use game::constants::{CHARACTER_POS_W, CHARACTER_POS_H, CHARACTER_W, CHARACTER_H, FIRE_SPRITE_START_INDEX};
use views::{Orientation};
use data::{load_character};

#[derive(PartialEq)]
pub enum Stance {
  Running = 0,
  Firing = 1,
}

pub struct Character {
  pub rect: Rectangle,
  pub sprites: AnimatedSprite,
  pub current: Orientation,
  pub heading: Orientation,
  pub move_anim_index: u32,
  pub fire_anim_index: u32,
}

impl Character {
  pub fn new(renderer: &mut Renderer) -> Character {
    Character {
      rect: Rectangle {
        x: CHARACTER_POS_W,
        y: CHARACTER_POS_H,
        w: CHARACTER_W,
        h: CHARACTER_H,
      },
      sprites: Character::get_sprite(renderer, 30.0),
      current: Orientation::Down,
      heading: Orientation::Down,
      move_anim_index: 0,
      fire_anim_index: 0,
    }
  }

  fn get_sprite(renderer: &mut Renderer, fps: f64) -> AnimatedSprite {
    let character_spritesheet = Sprite::load(&renderer, "assets/character.png").unwrap();

    let character_datapoints = load_character();
    let mut character_sprites = Vec::with_capacity(512);

    for x in 0..(FIRE_SPRITE_START_INDEX - 1) {
      character_sprites.push(character_spritesheet.region(character_datapoints[x]).unwrap());
    }

    for x in FIRE_SPRITE_START_INDEX..255 {
      character_sprites.push(character_spritesheet.region(character_datapoints[x]).unwrap());
    }

    AnimatedSprite::with_fps(character_sprites, fps)
  }

  pub fn update(&mut self, dt: f64, dx: f64, dy: f64, stance: Stance) {
    self.sprites.add_time(dt);
    self.current =
      if dx == 0.0 && dy < 0.0       { Orientation::Up }
      else if dx > 0.0 && dy < 0.0   { Orientation::UpRight }
      else if dx < 0.0 && dy < 0.0   { Orientation::UpLeft }
      else if dx == 0.0 && dy == 0.0 { self.heading }
      else if dx > 0.0 && dy == 0.0  { Orientation::Right }
      else if dx < 0.0 && dy == 0.0  { Orientation::Left }
      else if dx == 0.0 && dy > 0.0  { Orientation::Down }
      else if dx > 0.0 && dy > 0.0   { Orientation::DownRight }
      else if dx < 0.0 && dy > 0.0   { Orientation::DownLeft }
      else { unreachable!() };

    self.heading = self.current;

    if dx == 0.0 && dy == 0.0 && stance != Stance::Firing {
      let index = self.heading as usize * 28;
      self.sprites.set_curr_frames(index, index+1);
    } else {
      match stance {
        Stance::Running => {
          let index = self.current as usize * 28;
          self.sprites.set_curr_frames(index, index+13);
        },
        Stance::Firing => {
          let index = FIRE_SPRITE_START_INDEX + self.current as usize * 5;
          self.sprites.set_curr_frames(index, index+4);
        }
      }
    }
  }

  pub fn render(&mut self, renderer: &mut Renderer) {
    renderer.copy_sprite(&self.sprites, self.rect);
  }
}
