#[macro_use]
extern crate json;
extern crate sdl2;
extern crate conv;
extern crate tiled;

mod game;
mod views;
mod data;

fn main() {
  game::spawn("Zombie shooter", |game| {
    Box::new(views::GameView::new(game))
  });
}
