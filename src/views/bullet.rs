use sdl2::pixels::Color;
use game::data::Rectangle;
use game::{Game};
use game::constants::{BULLET_W, BULLET_H, BULLET_SPEED};
use views::{Orientation, Point};

#[derive(Clone, Debug)]
pub struct Bullet {
  pub rect: Rectangle,
  pub direction: Orientation
}

impl Bullet {
  pub fn new(point: Rectangle, orientation: Orientation) -> Bullet {
    let mut correction = Point { x: 0.0, y: 0.0 };
    match orientation {
      Orientation::Right => {
        correction.x = point.x + 55.0;
        correction.y = point.y + 5.0;
      },
      Orientation::UpRight => {
        correction.x = point.x + 50.0;
        correction.y = point.y + 5.0;
      },
      Orientation::Up => {
        correction.x = point.x + 48.0;
        correction.y = point.y;
      },
      Orientation::UpLeft => {
        correction.x = point.x + 50.0;
        correction.y = point.y + 10.0;
      },
      Orientation::Left => {
        correction.x = point.x + 40.0;
        correction.y = point.y + 5.0;
      },
      Orientation::DownLeft => {
        correction.x = point.x + 40.0;
        correction.y = point.y;
      },
      Orientation::Down => {
        correction.x = point.x + 30.0;
        correction.y = point.y + 20.0;
      },
      Orientation::DownRight => {
        correction.x = point.x + 60.0;
        correction.y = point.y + 20.0;
      }
    }

    Bullet {
      rect: Rectangle {
        x: correction.x,
        y: correction.y,
        w: BULLET_W,
        h: BULLET_H,
      },
      direction: orientation
    }
  }
}

pub trait Projectile {
  fn update(self: Box<Self>, game: &mut Game, dt: f64) -> Option<Box<Projectile>>;

  fn render(&self, game: &mut Game);

  fn rect(&self) -> Rectangle;
}

impl Projectile for Bullet {
  fn update(mut self: Box<Self>, game: &mut Game, dt: f64) -> Option<Box<Projectile>> {
    let (w, h) = game.output_size();

    match self.direction {
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
