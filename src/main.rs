extern crate clap;
#[macro_use]
extern crate gfx;

use clap::{App, Arg};

use crate::game::constants::{GAME_TITLE, GAME_VERSION};
use crate::gfx_app::GameOptions;

mod audio;
mod bullet;
mod gfx_app;
mod game;
mod data;
mod critter;
pub mod graphics;
mod hud;
mod terrain_object;
mod terrain;
mod character;
mod shaders;
mod zombie;

pub fn main() {
  let matches = App::new(GAME_TITLE)
    .version(GAME_VERSION)
    .author("Laastine <mikko.kaistnen@kapsi.fi>")
    .about("Hinterland - Isometric shooter game")
    .arg(Arg::with_name("windowed_mode")
      .short("w")
      .long("windowed_mode")
      .help("Run game in windowed mode"))
    .arg(Arg::with_name("frame_rate")
      .short("f")
      .long("frame_rate")
      .help("Print current frame rate to console"))
    .get_matches();
  let game_opt = GameOptions::new(matches.is_present("windowed_mode"),
        matches.is_present("frame_rate"));
  let mut window = gfx_app::WindowContext::new(game_opt);
  gfx_app::init::run(&mut window);
}
