extern crate sdl2;
extern crate chrono;
extern crate conv;
#[macro_use]
extern crate json;
extern crate tiled;

mod game;
mod views;
mod data;

fn main() {
  game::spawn("Hacknslash", |game| {
    Box::new(views::GameView::new(game))
  });
}
