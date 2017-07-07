use cgmath::{SquareMatrix, Matrix4, Transform, Point3, Vector3};
use gfx_app::{ColorFormat, DepthFormat};
use gfx;
use gfx::{Resources, Factory, texture};
use gfx::handle::ShaderResourceView;
use gfx::format::Rgba8;
use physics::Dimensions;
use specs;
use genmesh::{Vertices, Triangulate};
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use image;
use std::io::Cursor;
use terrain::gfx_macros::{TileMapData, VertexData, Bounds, Projection, pipe};
use game::constants::{TILEMAP_BUF_LENGTH};

#[macro_use]
pub mod gfx_macros;
pub mod terrain;

pub fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> Result<ShaderResourceView<R, [f32; 4]>, String> where R: Resources, F: Factory<R> {
  let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
  let (width, height) = img.dimensions();
  let kind = texture::Kind::D2(width as texture::Size, height as texture::Size, texture::AaMode::Single);
  let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
  Ok(view)
}

fn cartesian_to_isometric(point_x: f32, point_y: f32) -> (f32, f32) {
  ((point_x - point_y), (point_x + point_y) / 2.0)
}

impl TileMapData {
  pub fn new_empty() -> TileMapData {
    TileMapData { data: [32.0, 32.0, 0.0, 0.0] }
  }

  pub fn new(data: [f32; 4]) -> TileMapData {
    TileMapData { data: data }
  }
}

#[derive(Debug)]
pub struct Drawable {
  bounds: Bounds,
}

impl Drawable {
  pub fn new() -> Drawable {
    Drawable { bounds: Bounds { data: [[0.0; 4]; 4] } }
  }

  pub fn update(&mut self, world_to_clip: &Matrix4<f32>) {
    self.bounds.data = (*world_to_clip).into();
  }
}

impl specs::Component for Drawable {
  type Storage = specs::HashMapStorage<Drawable>;
}

const SHADER_VERT: &'static [u8] = include_bytes!("tilemap.v.glsl");
const SHADER_FRAG: &'static [u8] = include_bytes!("tilemap.f.glsl");

pub struct DrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> DrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>,
                terrain: &terrain::Terrain)
                -> DrawSystem<R>
    where F: gfx::Factory<R>
  {
    use gfx::traits::FactoryExt;

    let tile_size = 32;
    let width = 32;
    let height = 32;
    let total_size = 64;
    let half_width = (tile_size * width) / 2;
    let half_height = (tile_size * height) / 2;
    let total_size = width * height;

    let tilesheet_bytes = &include_bytes!("../../assets/maps/terrain.png")[..];
    let tilesheet_width = 32;
    let tilesheet_height = 32;
    let tilesheet_tilesize = 32;

    let tilesheet_total_width = tilesheet_width * tilesheet_tilesize;
    let tilesheet_total_height = tilesheet_height * tilesheet_tilesize;
    let plane = Plane::subdivide(width, width);

    let vertex_data: Vec<VertexData> = plane.shared_vertex_iter()
      .map(|(x, y)| {
        let (raw_x, raw_y) = cartesian_to_isometric(x, y);
        let vertex_x = half_width as f32 * raw_x;
        let vertex_y = half_height as f32 * raw_y;

        let (u_pos, v_pos) = ((raw_x / 4.0 - raw_y / 2.0) + 0.5, (raw_x / 4.0 + raw_y / 2.0) + 0.5);
        let tilemap_x = u_pos * width as f32;
        let tilemap_y = v_pos * height as f32;

        VertexData {
          pos: [vertex_x, vertex_y, 0.0],
          buf_pos: [tilemap_x as f32, tilemap_y as f32]
        }
      })
      .collect();

    let index_data: Vec<u32> = plane.indexed_polygon_iter()
      .triangulate()
      .vertices()
      .map(|i| i as u32)
      .collect();

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

    let tile_texture = load_texture(factory, tilesheet_bytes).unwrap();

    let view: AffineMatrix3<f32> = Transform::look_at(
      Point3::new(0.0, 0.0, 2000.0),
      Point3::new(0.0, 0.0, 0.0),
      Vector3::unit_y(),
    );

    let program = factory.link_program(SHADER_VERT, SHADER_FRAG).unwrap();

    let pso = factory
      .create_pipeline_from_program(&program,
                                    gfx::Primitive::TriangleStrip,
                                    gfx::state::Rasterizer::new_fill(),
                                    pipe::new())
      .unwrap();

    let params = pipe::Data {
      vbuf: vertex_buf,
      projection_cb: factory.create_constant_buffer(1),
      tilemap: factory.create_constant_buffer(TILEMAP_BUF_LENGTH),
      tilemap_cb: factory.create_constant_buffer(1),
      tilesheet: (tile_texture, factory.create_sampler_linear()),
      bounds: factory.create_constant_buffer(1),
//      out: rtv,
      out_color: rtv,
      out_depth: dsv,
    };

    DrawSystem { bundle: gfx::Bundle::new(slice, pso, params)}
  }

  pub fn draw<C>(&mut self, drawable: &Drawable, encoder: &mut gfx::Encoder<R, C>)
    where C: gfx::CommandBuffer<R>
  {
    encoder.clear(&self.bundle.data.out_color,
                  [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0]);
    encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
    //update_view
    //buffer update
    //clear
    encoder.update_constant_buffer(&self.bundle.data.bounds, &drawable.bounds);
//    encoder.update_buffer(&self.bundle.data.tilemap, &drawable.bounds.as_slice(), 0).unwrap();  //tilemap
//    encoder.update_constant_buffer(&self.params.projection_cb, &self.projection);   //projection
    self.bundle.encode(encoder);
  }
}

#[derive(Debug)]
pub struct PreDrawSystem;

impl PreDrawSystem {
  pub fn new() -> PreDrawSystem {
    PreDrawSystem {}
  }
}

impl<C> specs::System<C> for PreDrawSystem {
  fn run(&mut self, arg: specs::RunArg, _: C) {
    use specs::Join;
    let (mut terrain, dim) =
      arg.fetch(|w| (w.write::<Drawable>(), w.read_resource::<Dimensions>()));

    let world_to_clip = dim.world_to_clip();

    for t in (&mut terrain).join() {
      t.update(&world_to_clip);
    }
  }
}
