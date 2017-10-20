use shaders::{pipe, VertexData, CharacterSheet, Position, Projection};
use graphics::orientation::{Orientation, Stance};
use cgmath::{Matrix4, Point3, Vector3};
use game::constants::{ASPECT_RATIO, VIEW_DISTANCE};
use cgmath;
use specs;

const SHADER_VERT: &'static [u8] = include_bytes!("../shaders/character.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("../shaders/character.f.glsl");

pub struct ZombieDrawable {
  projection: Projection,
  position: Position,
  orientation: Orientation,
  stance: Stance,
  direction: Orientation
}

impl ZombieDrawable {
  pub fn new(view: Matrix4<f32>) -> ZombieDrawable {
    ZombieDrawable {
      projection: Projection {
        model: Matrix4::from(view).into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(60.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      position: Position {
        position: [0.0, 0.0],
      },
      orientation: Orientation::Right,
      stance: Stance::Normal,
      direction: Orientation::Right,
    }
  }

  pub fn update(&mut self) {
    let new_position = Position {
      position: [0.0, 0.0]
    };

    self.stance = Stance::Normal;
    let dx = new_position.position[0] - self.position.position[0];
    let dy = new_position.position[1] - self.position.position[1];
    self.orientation =
      if dx == 0.0 && dy < 0.0       { Orientation::Down }
      else if dx > 0.0 && dy < 0.0   { Orientation::DownRight }
      else if dx < 0.0 && dy < 0.0   { Orientation::DownLeft }
      else if dx == 0.0 && dy == 0.0 { Orientation::Still }
      else if dx > 0.0 && dy == 0.0  { Orientation::Right }
      else if dx < 0.0 && dy == 0.0  { Orientation::Left }
      else if dx == 0.0 && dy > 0.0  { Orientation::Up }
      else if dx > 0.0 && dy > 0.0   { Orientation::UpRight }
      else if dx < 0.0 && dy > 0.0   { Orientation::UpLeft }
      else { unreachable!() };
    self.position = new_position;
  }
}

impl specs::Component for ZombieDrawable {
  type Storage = specs::VecStorage<ZombieDrawable>;
}


