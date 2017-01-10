use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use std::path::Path;
use std::vec::Vec;
use json::{parse};

use game::data::Rectangle;

fn read_sprite_file(filename: &str) -> String {
  let path = Path::new(&filename);
  let mut file = match File::open(&path) {
    Ok(f) => f,
    Err(e) => panic!("File {} not found: {}", filename, e),
  };
  let mut buf = String::new();
  match file.read_to_string(&mut buf) {
    Ok(_) => buf,
    Err(_) => panic!("Couldn't read file {}", filename),
  }
}

pub fn load_character() -> Vec<Rectangle> {
  let mut sprites = Vec::with_capacity(512);
  let mut move_sprite_names = Vec::with_capacity(256);
  let mut fire_sprite_names = Vec::with_capacity(256);
  let character_json = read_sprite_file("./assets/character.json");
  let character = match parse(&character_json) {
    Ok(res) => res,
    Err(e) => panic!("Character JSON parse error {:?}", e),
  };
  for x in 0..15 {
    for y in 0..14 {
      move_sprite_names.push(format!("run_{}_{}", x, y));
    }
  }
  for x in 0..15 {
    for y in 0..3 {
      fire_sprite_names.push(format!("fire_{}_{}", x, y));
    }
  }

  for &ref move_sprite in &move_sprite_names {
    let x = character["frames"][move_sprite]["frame"]["x"].as_f64().unwrap();
    let y = character["frames"][move_sprite]["frame"]["y"].as_f64().unwrap();
    let w = character["frames"][move_sprite]["frame"]["w"].as_f64().unwrap();
    let h = character["frames"][move_sprite]["frame"]["h"].as_f64().unwrap();
    sprites.push(Rectangle {
      w: w,
      h: h,
      x: x,
      y: y,
    });
  }

  for &ref fire_sprite in &fire_sprite_names {
    let x = character["frames"][fire_sprite]["frame"]["x"].as_f64().unwrap();
    let y = character["frames"][fire_sprite]["frame"]["y"].as_f64().unwrap();
    let w = character["frames"][fire_sprite]["frame"]["w"].as_f64().unwrap();
    let h = character["frames"][fire_sprite]["frame"]["h"].as_f64().unwrap();
    sprites.push(Rectangle {
      w: w,
      h: h,
      x: x,
      y: y,
    });
  }
  sprites
}

