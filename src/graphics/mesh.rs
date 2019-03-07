use cgmath::{Matrix2, Point2, Vector2};
use cgmath::Angle;
use cgmath::Deg;
use gfx;
use gfx::Resources;
use gfx::traits::FactoryExt;

use crate::graphics::texture::Texture;
use crate::shaders::VertexData;

const DEFAULT_INDEX_DATA: &[u16] = &[0, 1, 2, 2, 3, 0];

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

  pub fn new_with_data<F>(factory: &mut F, size: Point2<f32>, rotation: Option<f32>) -> PlainMesh<R> where F: gfx::Factory<R> {
    let w = size.x;
    let h = size.y;
    let rot = rotation.unwrap_or(0.0);

    let vertex_data: Vec<VertexData> = vec![
      VertexData::new([-w, -h], [0.0, 1.0]),
      VertexData::new([w, -h], [1.0, 1.0]),
      VertexData::new([w, h], [1.0, 0.0]),
      VertexData::new([-w, h], [0.0, 0.0]),
    ].iter()
      .map(|el| {
        let cos = Angle::cos(Deg(rot));
        let sin = Angle::sin(Deg(rot));
        let rotated =
          Matrix2::<f32>::new(cos, -sin, sin, cos) * Vector2::<f32>::new(el.pos[0] as f32, el.pos[1] as f32);
        VertexData { pos: [rotated.x, rotated.y], uv: el.uv }
      })
      .collect();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data[..], DEFAULT_INDEX_DATA);
    PlainMesh {
      slice,
      vertex_buffer,
    }
  }
}

#[derive(Clone)]
pub struct Mesh<R> where R: Resources {
  pub slice: gfx::Slice<R>,
  pub vertex_buffer: gfx::handle::Buffer<R, VertexData>,
  pub texture: Texture<R>,
}

#[derive(Clone)]
pub struct RectangularMesh<R> where R: Resources {
  pub mesh: Mesh<R>,
  pub size: Point2<f32>,
}

impl<R> RectangularMesh<R> where R: gfx::Resources {
  pub fn new<F>(factory: &mut F, texture: Texture<R>, size: Point2<f32>) -> RectangularMesh<R> where F: gfx::Factory<R> {
    let w = size.x;
    let h = size.y;
    let vertex_data: &[VertexData; 4] = &[
      VertexData::new([-w, -h], [0.0, 1.0]),
      VertexData::new([w, -h], [1.0, 1.0]),
      VertexData::new([w, h], [1.0, 0.0]),
      VertexData::new([-w, h], [0.0, 0.0]),
    ];
    let mesh = Mesh::new(factory, vertex_data, &DEFAULT_INDEX_DATA, texture);
    RectangularMesh {
      mesh,
      size,
    }
  }
}

impl<R> Mesh<R> where R: gfx::Resources {
  pub fn new<F>(factory: &mut F, vertices: &[VertexData], indices: &[u16], texture: Texture<R>) -> Mesh<R> where F: gfx::Factory<R> {
    let mesh = PlainMesh::new(factory, vertices, indices);
    Mesh {
      slice: mesh.slice,
      vertex_buffer: mesh.vertex_buffer,
      texture,
    }
  }
}
