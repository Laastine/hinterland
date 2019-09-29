use cgmath::{Matrix2, Point2, Vector2};
use cgmath::Angle;
use cgmath::Deg;
use gfx;
use gfx::Resources;
use gfx::traits::FactoryExt;

use crate::graphics::orientation::Orientation;
use crate::graphics::texture::Texture;
use crate::shaders::VertexData;

const DEFAULT_INDEX_DATA: &[u16] = &[0, 1, 2, 2, 3, 0];

fn rectangle_mesh(w: f32, h: f32) -> [VertexData; 4] {
  [
    VertexData::new([-w, -h], [0.0, 1.0]),
    VertexData::new([w, -h], [1.0, 1.0]),
    VertexData::new([w, h], [1.0, 0.0]),
    VertexData::new([-w, h], [0.0, 0.0]),
  ]
}

fn edit_vertices(w: f32, h: f32, scale: Option<Matrix2<f32>>, rotation: Option<f32>, orientation: Option<Orientation>) -> Vec<VertexData> {
  let scale_matrix = scale.unwrap_or_else(|| Matrix2::new(1.0, 0.0, 0.0, 1.0));

  let rot = rotation.unwrap_or(0.0);

  let rot_x = 15.0;
  let rot_y = 15.0;

  rectangle_mesh(w, h).to_vec().iter()
    .map(|el| {
      let cos = Angle::cos(Deg(rot));
      let sin = Angle::sin(Deg(rot));

      let x_skew = match orientation {
        Some(Orientation::Right) => Angle::tan(Deg(-rot_x)),
        Some(Orientation::Left) => Angle::tan(Deg(rot_x)),
        Some(Orientation::DownLeft) => Angle::tan(Deg(-rot_x)),
        Some(Orientation::DownRight) => Angle::tan(Deg(-rot_x)),
        Some(Orientation::UpLeft) => Angle::tan(Deg(rot_x)),
        Some(Orientation::UpRight) => Angle::tan(Deg(-rot_x)),
        _ => 0.0,
      };

      let y_skew = match orientation {
        Some(Orientation::Up) => Angle::tan(Deg(-rot_y)),
        Some(Orientation::Down) => Angle::tan(Deg(rot_y)),
        Some(Orientation::DownRight) => Angle::tan(Deg(-rot_y)),
        Some(Orientation::UpLeft) => Angle::tan(Deg(rot_y)),
        Some(Orientation::UpRight) => Angle::tan(Deg(-rot_y)),
        _ => 0.0,
      };
      let skew_matrix = Matrix2::<f32>::new(1.0, y_skew, x_skew, 1.0);
      let rotation_matrix = Matrix2::<f32>::new(cos, -sin, sin, cos);
      let translate = Vector2::<f32>::new(-14.0, -27.0);

      let edited_vertex_data =
        translate +
          skew_matrix *
            (scale_matrix *
              (rotation_matrix * Vector2::<f32>::new(el.pos[0] as f32, el.pos[1] as f32)));

      VertexData { pos: [edited_vertex_data.x, edited_vertex_data.y], uv: el.uv }
    })
    .collect::<Vec<VertexData>>()
}

#[derive(Clone)]
pub struct PlainMesh<R> where R: Resources {
  pub slice: gfx::Slice<R>,
  pub vertex_buffer: gfx::handle::Buffer<R, VertexData>,
}

impl<R> PlainMesh<R> where R: gfx::Resources {
  pub fn new<F>(factory: &mut F, vertices: &[VertexData], indices: &[u16]) -> PlainMesh<R> where F: gfx::Factory<R> {
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
    PlainMesh {
      slice,
      vertex_buffer,
    }
  }

  pub fn new_with_data<F>(factory: &mut F, size: Point2<f32>, scale: Option<Matrix2<f32>>, rotation: Option<f32>, orientation: Option<Orientation>) -> PlainMesh<R> where F: gfx::Factory<R> {
    let w = size.x;
    let h = size.y;

    let vertex_data = edit_vertices(w, h, scale, rotation, orientation);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data[..], DEFAULT_INDEX_DATA);
    PlainMesh {
      slice,
      vertex_buffer,
    }
  }
}

#[derive(Clone)]
pub struct TexturedMesh<R> where R: Resources {
  pub slice: gfx::Slice<R>,
  pub vertex_buffer: gfx::handle::Buffer<R, VertexData>,
  pub texture: Texture<R>,
}

#[derive(Clone)]
pub struct RectangularTexturedMesh<R> where R: Resources {
  pub mesh: TexturedMesh<R>,
  pub size: Point2<f32>,
}

impl<R> RectangularTexturedMesh<R> where R: gfx::Resources {
  pub fn new<F>(factory: &mut F,
                texture: Texture<R>,
                size: Point2<f32>,
                scale: Option<Matrix2<f32>>,
                rotation: Option<f32>,
                orientation: Option<Orientation>) -> RectangularTexturedMesh<R> where F: gfx::Factory<R> {
    let w = size.x;
    let h = size.y;

    let vertex_data = edit_vertices(w, h, scale, rotation, orientation);

    let mesh = TexturedMesh::new(factory, &vertex_data, &DEFAULT_INDEX_DATA, texture);
    RectangularTexturedMesh {
      mesh,
      size,
    }
  }
}

impl<R> TexturedMesh<R> where R: gfx::Resources {
  pub fn new<F>(factory: &mut F, vertices: &[VertexData], indices: &[u16], texture: Texture<R>) -> TexturedMesh<R> where F: gfx::Factory<R> {
    let mesh = PlainMesh::new(factory, vertices, indices);
    TexturedMesh {
      slice: mesh.slice,
      vertex_buffer: mesh.vertex_buffer,
      texture,
    }
  }
}
