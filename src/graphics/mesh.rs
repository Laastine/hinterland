use cgmath::Point2;
use gfx;
use gfx::Resources;
use gfx::traits::FactoryExt;
use graphics::texture::Texture;
use shaders::VertexData;

#[derive(Clone, Debug)]
pub struct Mesh<R> where R: Resources {
  pub slice: gfx::Slice<R>,
  pub vertex_buffer: gfx::handle::Buffer<R, VertexData>,
  pub texture: Texture<R>,
}

#[derive(Clone, Debug)]
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
    let index_data = &[0, 1, 2, 2, 3, 0];
    let mesh = Mesh::new(factory, vertex_data, index_data, texture);
    RectangularMesh {
      mesh,
      size
    }
  }
}

impl<R> Mesh<R> where R: gfx::Resources {
  pub fn new<F>(factory: &mut F, vertices: &[VertexData], indices: &[u16], texture: Texture<R>) -> Mesh<R> where F: gfx::Factory<R> {
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
    Mesh {
      slice,
      vertex_buffer,
      texture,
    }
  }
}
