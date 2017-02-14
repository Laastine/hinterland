use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use std::path::Path;
use std::vec::Vec;
use std::io::{BufReader};
use json;
use game::data::Rectangle;
use tiled::{Map};
use tiled;

pub fn load_map_file(filename: &str) -> Map {
  let file = match File::open(&Path::new(&filename)) {
    Ok(f) => f,
    Err(e) => panic!("File {} not found: {}", filename, e),
  };
  let reader = BufReader::new(file);
  match tiled::parse(reader) {
    Ok(m) => m,
    Err(e) => panic!("Map parse error {:?}", e)
  }
}

pub fn get_tile(map: &Map, layer_index: usize, x: usize, y: usize) -> u32 {
  let layer = match map.layers.get(layer_index) {
    None => panic!("layer_index value out of index {:?}", map.layers),
    Some(ref l) => *l
  };
  let y_index = match layer.tiles.iter().nth(y) {
    None => panic!("x value out of index {:?}", map.layers[0]),
    Some(ref x) => *x
  };
  let val = match y_index.get(x) {
    None => panic!("y value out of index {:?}", layer.tiles[y]),
    Some(ref val) => **val
  };
  val
}

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
  let character = match json::parse(&character_json) {
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

pub fn load_zombie() -> Vec<Rectangle> {
  let mut sprites = Vec::with_capacity(256);
  let mut idle_sprite_names = Vec::with_capacity(64);
  let mut walk_sprite_names = Vec::with_capacity(64);
  let character_json = read_sprite_file("./assets/zombie.json");
  let zombie = match json::parse(&character_json) {
    Ok(res) => res,
    Err(e) => panic!("Character JSON parse error {:?}", e),
  };

  for x in 0..7 {
    for y in 0..3 {
      idle_sprite_names.push(format!("idle_{}_{}", x, y));
    }
  }

  for x in 0..7 {
    for y in 0..3 {
      walk_sprite_names.push(format!("walk_{}_{}", x, y));
    }
  }

  for &ref idle_sprite in &idle_sprite_names {
    let x = zombie["frames"][idle_sprite]["frame"]["x"].as_f64().unwrap();
    let y = zombie["frames"][idle_sprite]["frame"]["y"].as_f64().unwrap();
    let w = zombie["frames"][idle_sprite]["frame"]["w"].as_f64().unwrap();
    let h = zombie["frames"][idle_sprite]["frame"]["h"].as_f64().unwrap();
    sprites.push(Rectangle {
      w: w,
      h: h,
      x: x,
      y: y,
    });
  }

  for &ref walk_sprite in &walk_sprite_names {
    let x = zombie["frames"][walk_sprite]["frame"]["x"].as_f64().unwrap();
    let y = zombie["frames"][walk_sprite]["frame"]["y"].as_f64().unwrap();
    let w = zombie["frames"][walk_sprite]["frame"]["w"].as_f64().unwrap();
    let h = zombie["frames"][walk_sprite]["frame"]["h"].as_f64().unwrap();
    sprites.push(Rectangle {
      w: w,
      h: h,
      x: x,
      y: y,
    });
  }
  sprites
}
