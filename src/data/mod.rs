use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::string::String;
use std::path::Path;
use json::{JsonValue, parse};

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

pub fn load_character() -> JsonValue {
  let character_json = read_sprite_file("./assets/character.json");
  match parse(&character_json) {
    Ok(res) => res,
    Err(e) => panic!("Character JSON parse error {:?}", e),
  }
}
