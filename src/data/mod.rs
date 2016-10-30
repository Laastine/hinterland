use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::string::String;
use std::path::Path;

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

