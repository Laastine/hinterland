extern crate getopts;
#[macro_use]
extern crate gfx;

use getopts::Options;

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

fn print_usage() {
  println!("USAGE:\nhinterland [FLAGS]\n\nFLAGS:\n-f, --frame_rate\t\tPrint current frame rate to console\n-h, --help\t\t\tPrints help information\n-v, --version\t\t\tPrints version information\n-w, --windowed_mode\t\tRun game in windowed mode");
}

fn print_version() {
  println!("{} - {}", GAME_TITLE, GAME_VERSION)
}

pub fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  let mut opts = Options::new();
  opts.optflag("w", "windowed_mode", "Run game in windowed mode");
  opts.optflag("f", "frame_rate", "Print current frame rate to console");
  opts.optflag("h", "help", "Prints help information");
  opts.optflag("v", "version", "Prints version information");

  let matches = match opts.parse(&args[1..]) {
    Ok(matching_args) => { matching_args }
    Err(err) => { panic!(err.to_string()) }
  };

  if matches.opt_present("help") {
    print_usage();
    return;
  }

  if matches.opt_present("version") {
    print_version();
    return;
  }

  let game_opt = GameOptions::new(
    matches.opt_present("windowed_mode"),
    matches.opt_present("frame_rate"));
  let mut window = gfx_app::WindowContext::new(game_opt);
  gfx_app::init::run(&mut window);
}
