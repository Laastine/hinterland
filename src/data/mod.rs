use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use std::path::Path;
use std::vec::Vec;
use std::io::BufReader;
use json;
use game::constants::{CHARACTER_JSON_PATH, CHARACTER_BUF_LENGTH, ZOMBIE_JSON_PATH};
use tiled::Map;
use tiled;
use critter::CritterData;

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

pub fn load_character() -> Vec<CritterData> {
  let mut sprites = Vec::with_capacity(CHARACTER_BUF_LENGTH + 64);
  let character_json = read_sprite_file(CHARACTER_JSON_PATH);
  let character = match json::parse(&character_json) {
    Ok(res) => res,
    Err(e) => panic!("Character JSON parse error {:?}", e),
  };
  for x in 0..16 {
    for y in 0..14 {
      let key = &format!("run_{}_{}", x, y);
      sprites.push(CritterData::new([
        character["frames"][key]["frame"]["x"].as_f32().unwrap(),
        character["frames"][key]["frame"]["y"].as_f32().unwrap(),
        character["frames"][key]["frame"]["w"].as_f32().unwrap(),
        character["frames"][key]["frame"]["h"].as_f32().unwrap(),
      ]));
    }
  }

  for x in 0..15 {
    for y in 0..4 {
      let key = &format!("fire_{}_{}", x, y);
      sprites.push(CritterData::new([
        character["frames"][key]["frame"]["x"].as_f32().unwrap(),
        character["frames"][key]["frame"]["y"].as_f32().unwrap(),
        character["frames"][key]["frame"]["w"].as_f32().unwrap(),
        character["frames"][key]["frame"]["h"].as_f32().unwrap(),
      ]));
    }
  }

  sprites
}

pub fn load_zombie() -> Vec<CritterData> {
  let mut sprites = Vec::with_capacity(128);
  let character_json = read_sprite_file(ZOMBIE_JSON_PATH);
  let zombie = match json::parse(&character_json) {
    Ok(res) => res,
    Err(e) => panic!("Character JSON parse error {:?}", e),
  };
  for x in 0..7 {
    for y in 0..4 {
      let key = &format!("still_{}_{}", x, y);
      sprites.push(CritterData::new([
        zombie["frames"][key]["frame"]["x"].as_f32().unwrap(),
        zombie["frames"][key]["frame"]["y"].as_f32().unwrap(),
        zombie["frames"][key]["frame"]["w"].as_f32().unwrap(),
        zombie["frames"][key]["frame"]["h"].as_f32().unwrap()
      ]));
    }
  }

  for x in 0..7 {
    for y in 0..7 {
      let key = &format!("walk_{}_{}", x, y);
      sprites.push(CritterData::new([
        zombie["frames"][key]["frame"]["x"].as_f32().unwrap(),
        zombie["frames"][key]["frame"]["y"].as_f32().unwrap(),
        zombie["frames"][key]["frame"]["w"].as_f32().unwrap(),
        zombie["frames"][key]["frame"]["h"].as_f32().unwrap()
      ]));
    }
  }
  sprites
}
