use std::io::{BufReader};
use std::fs::File;
use std::path::Path;
use tiled::parse;

pub fn read_map_file(filename: &str) {
  let mut file = match File::open(&Path::new(&filename)) {
    Ok(f) => f,
    Err(e) => panic!("File {} not found: {}", filename, e),
  };
  let reader = BufReader::new(file);
  let map = parse(reader).unwrap();
  println!("{:?}", map);
}
