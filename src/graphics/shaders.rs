use std::{fmt::{Display, Formatter, Result}, ops::{Add, Sub}};
use std::fs::read_to_string;
use std::io::Read;
use std::mem::size_of;
use std::slice::from_raw_parts;

use cgmath::BaseFloat;

#[derive(Clone, Default)]
pub struct Projection {
  pub model: [[f32; 4]; 4],
  pub view: [[f32; 4]; 4],
  pub proj: [[f32; 4]; 4],
}

impl Projection {
  pub fn as_raw(&self) -> &[u8] {
    let all = [self.model, self.view, self.proj];
    unsafe {
      from_raw_parts(all.as_ptr() as *const u8, all.len() * size_of::<Projection>())
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct CharacterSpriteSheet {
  pub x_div: f32,
  pub y_div: f32,
  pub row_idx: u32,
  pub index: u32,
}

impl CharacterSpriteSheet {
  pub fn new(x_div: f32, y_div: f32, row_idx: u32, index: u32) -> CharacterSpriteSheet {
    CharacterSpriteSheet {
      x_div,
      y_div,
      row_idx,
      index,
    }
  }

  pub fn as_raw(&self) -> &[u8] {
    let all = [self.x_div, self.y_div, self.row_idx as f32, self.index as f32];
    unsafe {
      from_raw_parts(all.as_ptr() as *const u8, all.len() * size_of::<CharacterSpriteSheet>())
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
  position: [f32; 2],
}

impl Position {
  pub fn new<T: BaseFloat>(x: T, y: T) -> Position where f32: std::convert::From<T> {
    Position { position: [f32::from(x), f32::from(y)] }
  }

  pub fn as_raw(&self) -> &[u8] {
    unsafe {
      from_raw_parts(self.position.as_ptr() as *const u8, self.position.len() * size_of::<Position>())
    }
  }

  pub fn new_from_array(pos: [f32; 2]) -> Position {
    Position { position: pos }
  }

  pub fn origin() -> Position {
    Position { position: [0.0, 0.0] }
  }

  pub fn x(self) -> f32 {
    self.position[0]
  }

  pub fn y(self) -> f32 {
    self.position[1]
  }
}

impl Add for Position {
  type Output = Position;

  fn add(self, other: Position) -> Position {
    Position::new(self.x() + other.x(), self.y() + other.y())
  }
}

impl Sub for Position {
  type Output = Position;

  fn sub(self, other: Position) -> Position {
    Position::new(self.x() - other.x(), self.y() - other.y())
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}, {}", self.x(), self.y())
  }
}

#[derive(Clone, Copy, Debug)]
pub struct TileMapData {
  pub data: [f32; 4],
}

impl TileMapData {
  pub fn new_empty() -> TileMapData {
    TileMapData { data: [32.0, 32.0, 0.0, 0.0] }
  }

  pub fn new(data: [f32; 4]) -> TileMapData {
    TileMapData { data }
  }
}

#[derive(Clone, Copy)]
pub struct Vertex {
  pos: [f32; 4],
  uv: [f32; 2],
}

impl Vertex {
  pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Vertex {
    Vertex {
      pos: [pos[0], pos[1], pos[2], 1.0],
      uv,
    }
  }
}

impl Iterator for Vertex {
  type Item = Vertex;

  fn next(&mut self) -> Option<Self::Item> {
    Some(Vertex { pos: self.pos, uv: self.uv })
  }
}

#[allow(dead_code)]
pub enum ShaderStage {
  Vertex,
  Fragment,
  Compute,
}

pub fn load_glsl(path: &str, stage: ShaderStage) -> Vec<u8> {
  let shader_stage = match stage {
    ShaderStage::Vertex => glsl_to_spirv::ShaderType::Vertex,
    ShaderStage::Fragment => glsl_to_spirv::ShaderType::Fragment,
    ShaderStage::Compute => glsl_to_spirv::ShaderType::Compute,
  };

  let code = read_to_string(&path)
    .unwrap_or_else(|e| panic!("Unable to read {:?}: {:?}", path, e));

  let mut output = glsl_to_spirv::compile(&code, shader_stage).expect("Shader compile error");
  let mut spv = vec![];
  output.read_to_end(&mut spv).expect("Shader read error");
  spv
}
