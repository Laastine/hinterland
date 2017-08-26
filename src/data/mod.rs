use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use std::path::Path;
use std::vec::Vec;
use std::io::BufReader;
use json;
use game::constants::{CHARACTER_JSON_PATH, ZOMBIE_JSON_PATH};
use tiled::Map;
use tiled;
use character::CharacterData;

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

pub fn get_map_tile(map: &Map, layer_index: usize, x: usize, y: usize) -> u32 {
  let layer = match map.layers.get(layer_index) {
    None => panic!("Layer_index value out of index {:?}", map.layers),
    Some(ref l) => *l
  };
  let y_index = match layer.tiles.iter().rev().nth(y) {
    None => panic!("X value out of index {:?}", map.layers[0]),
    Some(ref x) => *x
  };
  let val = match y_index.get(x) {
    None => panic!("Y value out of index {:?}", layer.tiles[y]),
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

pub fn load_character() -> Vec<CharacterData> {
  let mut sprites = Vec::with_capacity(512);
  let mut move_sprite_names = Vec::with_capacity(256);
//  let mut fire_sprite_names = Vec::with_capacity(256);
  let character_json = read_sprite_file(CHARACTER_JSON_PATH);
  let character = match json::parse(&character_json) {
    Ok(res) => res,
    Err(e) => panic!("Character JSON parse error {:?}", e),
  };

  for x in 0..15 {
    for y in 0..14 {
      move_sprite_names.push(format!("run_{}_{}", x, y));
    }
  }
//  for x in 0..15 {
//    for y in 0..3 {
//      fire_sprite_names.push(format!("fire_{}_{}", x, y));
//    }
//  }

//  for &ref move_sprite in &move_sprite_names {
  for x in 0..15 {
    for y in 0..14 {
      let ref key = format!("run_{}_{}", x, y);
      sprites.push(CharacterData::new([
        character["frames"][key]["frame"]["x"].as_f32().unwrap(),
        character["frames"][key]["frame"]["y"].as_f32().unwrap(),
        character["frames"][key]["frame"]["w"].as_f32().unwrap(),
        character["frames"][key]["frame"]["h"].as_f32().unwrap(),
        x as f32,
        y as f32
      ]));
    }
  }
//  }

//  for &ref fire_sprite in &fire_sprite_names {
//    sprites.push(CharacterData::new([
//      character["frames"][fire_sprite]["frame"]["x"].as_f32().unwrap(),
//      character["frames"][fire_sprite]["frame"]["y"].as_f32().unwrap(),
//      character["frames"][fire_sprite]["frame"]["w"].as_f32().unwrap(),
//      character["frames"][fire_sprite]["frame"]["h"].as_f32().unwrap()
//    ]));
//  }
  sprites
}

#[allow(dead_code)]
pub fn load_zombie() -> Vec<(f64, f64)> {
  let mut sprites = Vec::with_capacity(256);
  let mut idle_sprite_names = Vec::with_capacity(64);
  let mut walk_sprite_names = Vec::with_capacity(64);
  let character_json = read_sprite_file(ZOMBIE_JSON_PATH);
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
    sprites.push((x, y));
  }

  for &ref walk_sprite in &walk_sprite_names {
    let x = zombie["frames"][walk_sprite]["frame"]["x"].as_f64().unwrap();
    let y = zombie["frames"][walk_sprite]["frame"]["y"].as_f64().unwrap();
    sprites.push((x, y));
  }
  sprites
}
