use game::{Game, View, ViewAction};
use game::data::Rectangle;
use game::gfx::{CopySprite, Sprite};
use sdl2::pixels::Color;

const PLAYER_SPEED: f64 = 150.0;
const CHARACTER_W: f64 = 43.0;
const CHARACTER_H: f64 = 39.0;

#[derive(Clone, Copy)]
enum CharacterFrame {
  UpNorm   = 0,
  UpFast   = 1,
  UpSlow   = 2,
  MidFast  = 3,
  MidSlow  = 4,
  DownNorm = 5,
  DownFast = 6,
  DownSlow = 7
}

struct Character {
  rect: Rectangle,
  sprites: Vec<Sprite>,
  current: CharacterFrame,
}

pub struct GameView {
  player: Character,
}

impl GameView {
  pub fn new(game: &mut Game) -> GameView {
    let spritesheet = Sprite::load(&mut game.renderer, "assets/warrior.png").unwrap();
    let mut sprites = Vec::with_capacity(9);

    for x in 0..8 {
      sprites.push(spritesheet.region(Rectangle {
        w: CHARACTER_W,
        h: CHARACTER_H,
        x: CHARACTER_W * x as f64,
        y: 0 as f64,
      }).unwrap());
    }
    GameView {
      player: Character {
        rect: Rectangle {
          x: 64.0,
          y: 64.0,
          w: CHARACTER_W,
          h: CHARACTER_H,
        },
        sprites: sprites,
        current: CharacterFrame::DownNorm,
      }
    }
  }
}

impl View for GameView {
  fn render(&mut self, game: &mut Game, elapsed: f64) -> ViewAction {
    if game.events.now.quit || game.events.now.key_escape == Some(true) {
      return ViewAction::Quit;
    }

    let diagonal = (game.events.key_up ^ game.events.key_down) && (game.events.key_left ^ game.events.key_right);
    let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 } * PLAYER_SPEED * elapsed;
    let dx = match (game.events.key_left, game.events.key_right) {
      (true, true) | (false, false) => 0.0,
      (true, false) => -moved,
      (false, true) => moved,
    };

    let dy = match (game.events.key_up, game.events.key_down) {
      (true, true) | (false, false) => 0.0,
      (true, false) => -moved,
      (false, true) => moved,
    };

    self.player.rect.x += dx;
    self.player.rect.y += dy;

    let movable_region = Rectangle {
      x: 0.0,
      y: 0.0,
      w: game.output_size().0 * 0.70,
      h: game.output_size().1,
    };

    self.player.rect = self.player.rect.move_inside(movable_region).unwrap();
    self.player.current =
    if dx == 0.0 && dy < 0.0       { CharacterFrame::UpNorm }
    else if dx > 0.0 && dy < 0.0   { CharacterFrame::UpFast }
    else if dx < 0.0 && dy < 0.0   { CharacterFrame::UpSlow }
    else if dx == 0.0 && dy == 0.0 { CharacterFrame::DownNorm }
    else if dx > 0.0 && dy == 0.0  { CharacterFrame::MidFast }
    else if dx < 0.0 && dy == 0.0  { CharacterFrame::MidSlow }
    else if dx == 0.0 && dy > 0.0  { CharacterFrame::DownNorm }
    else if dx > 0.0 && dy > 0.0   { CharacterFrame::DownFast }
    else if dx < 0.0 && dy > 0.0   { CharacterFrame::DownSlow }
    else { unreachable!() };

    game.renderer.set_draw_color(Color::RGB(0, 0, 0));
    game.renderer.clear();

    game.renderer.set_draw_color(Color::RGB(170, 170, 170));
    game.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

    game.renderer.copy_sprite(
      &self.player.sprites[self.player.current as usize],
      self.player.rect);

    ViewAction::None
  }
}
