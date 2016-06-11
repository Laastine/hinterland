extern crate sdl2;
extern crate sdl2_image;
extern crate chrono;

mod game;
mod views;

fn main() {
  ::game::spawn("Hacknslash", |game| {
    Box::new(::views::GameView::new(game))
  });
}
