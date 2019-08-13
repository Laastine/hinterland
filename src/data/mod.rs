use std::{fs::File, io::BufReader, io::prelude::*, path::Path, string::String, vec::Vec};

use json;
use json::JsonValue;
use tiled;
use tiled::Map;

use crate::critter::CritterData;
use crate::game::constants::{CHARACTER_BUF_LENGTH, CHARACTER_JSON_PATH, ZOMBIE_JSON_PATH};

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
    Some(l) => l
  };
  let y_index = match layer.tiles.iter().rev().nth(y) {
    None => panic!("X value out of index {:?}", map.layers[0]),
    Some(x) => x
  };
  match y_index.get(x) {
    None => panic!("Y value out of index {:?}", layer.tiles[y]),
    Some(val) => *val
  }
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
    Err(e) => panic!("read file {} error {}", filename, e),
  }
}

fn get_frame_data(character: &JsonValue, key: &String) -> CritterData {
  CritterData::new([
    character["frames"][key]["frame"]["x"].as_f32().unwrap(),
    character["frames"][key]["frame"]["y"].as_f32().unwrap(),
    character["frames"][key]["frame"]["w"].as_f32().unwrap(),
    character["frames"][key]["frame"]["h"].as_f32().unwrap(),
  ])
}

pub fn load_character() -> Vec<CritterData> {
  let mut sprites = Vec::with_capacity(CHARACTER_BUF_LENGTH + 64);
  let character_json = read_sprite_file(CHARACTER_JSON_PATH);
  let character = match json::parse(&character_json) {
    Ok(res) => res,
    Err(e) => panic!("Character {} parse error {:?}", CHARACTER_JSON_PATH, e),
  };

  for x in 0..16 {
    for y in 0..14 {
      let key = &format!("run_{}_{}", x, y);
      sprites.push(get_frame_data(&character, key));
    }
  }

  for x in 0..15 {
    for y in 0..4 {
      let key = &format!("fire_{}_{}", x, y);
      sprites.push(get_frame_data(&character, key));
    }
  }

  sprites
}

pub fn load_zombie() -> Vec<CritterData> {
  let mut sprites = Vec::with_capacity(256);
  let zombie_json = read_sprite_file(ZOMBIE_JSON_PATH);
  let zombie = match json::parse(&zombie_json) {
    Ok(res) => res,
    Err(e) => panic!("Zombie {} parse error {:?}", ZOMBIE_JSON_PATH, e),
  };

  for x in 0..7 {
    for y in 0..7 {
      let key = &format!("critical_{}_{}", x, y);
      sprites.push(get_frame_data(&zombie, key));
    }
  }
  for x in 0..7 {
    for y in 0..5 {
      let key = &format!("normal_{}_{}", x, y);
      sprites.push(get_frame_data(&zombie, key));
    }
  }
  for x in 0..7 {
    for y in 0..4 {
      let key = &format!("still_{}_{}", x, y);
      sprites.push(get_frame_data(&zombie, key));
    }
  }

  for x in 0..7 {
    for y in 0..7 {
      let key = &format!("walk_{}_{}", x, y);
      sprites.push(get_frame_data(&zombie, key));
    }
  }
  sprites
}
