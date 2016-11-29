extern crate sdl2;
extern crate sdl2_image;
extern crate chrono;
extern crate conv;
#[macro_use]
extern crate json;

mod game;
mod views;
mod data;
mod logic;

fn main() {
  game::spawn("Hacknslash", |game| {
    Box::new(views::GameView::new(game))
  });
}
