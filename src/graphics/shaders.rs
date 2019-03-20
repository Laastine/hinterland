use cgmath::BaseFloat;
use std::fs::read_to_string;
use std::{fmt::{Display, Formatter, Result}, ops::{Add, Sub}};
use std::io::Read;

#[derive(Clone, Copy, Debug)]
pub struct Position {
  position: [f32; 2],
}

impl Position {
  pub fn new<T: BaseFloat>(x: T, y: T) -> Position where f32: std::convert::From<T> {
    Position { position: [f32::from(x), f32::from(y)] }
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
    Some(Vertex {pos: self.pos, uv: self.uv})
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
