use sdl2::pixels::Color;
use game::data::Rectangle;
use game::{Game};
use game::constants::{BULLET_W, BULLET_H, BULLET_SPEED};
use views::Orientation;

#[derive(Clone, Debug)]
pub struct Bullet {
  pub rect: Rectangle,
}

impl Bullet {
  pub fn new(point: Rectangle) -> Bullet {
    Bullet {
      rect: Rectangle {
        x: point.x,
        y: point.y,
        w: BULLET_W,
        h: BULLET_H,
      }
    }
  }
}

pub trait Projectile {
  fn update(self: Box<Self>, game: &mut Game, orientation: Orientation, dt: f64) -> Option<Box<Projectile>>;

  fn render(&self, game: &mut Game);

  fn rect(&self) -> Rectangle;
}

impl Projectile for Bullet {
  fn update(mut self: Box<Self>, game: &mut Game, orientation: Orientation, dt: f64) -> Option<Box<Projectile>> {
    let (w, h) = game.output_size();

    match orientation {
      Orientation::Right => {
        self.rect.x += BULLET_SPEED * dt;
      },
      Orientation::UpRight => {
        self.rect.x += BULLET_SPEED * dt / 2.0;
        self.rect.y -= BULLET_SPEED * dt;
      },
      Orientation::Up => {
        self.rect.y -= BULLET_SPEED * dt / 2.0;
      },
      Orientation::UpLeft => {
        self.rect.x -= BULLET_SPEED * dt / 2.0;
        self.rect.y -= BULLET_SPEED * dt;
      },
      Orientation::Left => {
        self.rect.x -= BULLET_SPEED * dt;
      },
      Orientation::DownLeft => {
        self.rect.x -= BULLET_SPEED * dt;
        self.rect.y += BULLET_SPEED * dt / 2.0;
      },
      Orientation::Down => {
        self.rect.y += BULLET_SPEED * dt;
      },
      Orientation::DownRight => {
        self.rect.x += BULLET_SPEED * dt;
        self.rect.y += BULLET_SPEED * dt / 2.0;
      }
    }

    if self.rect.x > w || self.rect.x < 0.0 ||
      self.rect.y > h || self.rect.y < 0.0 {
      None
    } else {
      Some(self)
    }
  }

  fn render(&self, game: &mut Game) {
    game.renderer.set_draw_color(Color::RGBA(50, 50, 50, 0));
    game.renderer.fill_rect(self.rect.to_sdl().unwrap());
  }

  fn rect(&self) -> Rectangle {
    self.rect
  }
}
