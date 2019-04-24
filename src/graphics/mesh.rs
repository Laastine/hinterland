use crate::graphics::shaders::Vertex;

pub fn create_vertices(width: f32, height: f32) -> (Vec<Vertex>, Vec<u16>) {
  let vertex_data: &[Vertex; 4] = &[
    Vertex::new([-width, -height, 0.0], [0.0, 1.0]),
    Vertex::new([width, -height, 0.0], [1.0, 1.0]),
    Vertex::new([width, height, 0.0], [1.0, 0.0]),
    Vertex::new([-width, height, 0.0], [0.0, 0.0]),
  ];

  let index_data = &[0, 1, 2, 2, 3, 0];

  (vertex_data.to_vec(), index_data.to_vec())
}
