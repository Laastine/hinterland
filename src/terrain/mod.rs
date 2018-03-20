use cgmath;
use character::controls::CharacterInputState;
use game::constants::{ASPECT_RATIO, TILE_MAP_BUF_LENGTH};
use game::constants::{TILES_PCS_H, TILES_PCS_W};
use genmesh::{Triangulate, Vertices};
use genmesh::generators::{IndexedPolygon, Plane, SharedVertex};
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::{can_move, Dimensions};
use graphics::{coords_to_tile, load_texture};
use graphics::camera::CameraInputState;
use shaders::{Position, Projection, tilemap_pipeline, TileMapData, TilemapSettings, VertexData};
use specs;
use specs::{Fetch, ReadStorage, WriteStorage};

pub mod tile_map;

fn cartesian_to_isometric(point_x: f32, point_y: f32) -> (f32, f32) {
  ((point_x - point_y), (point_x + point_y) / 2.0)
}

#[derive(Debug)]
pub struct TerrainDrawable {
  projection: Projection,
  pub position: Position,
}

impl TerrainDrawable {
  pub fn new() -> TerrainDrawable {
    let view = Dimensions::get_view_matrix();
    TerrainDrawable {
      projection: Projection {
        model: view.into(),
        view: view.into(),
        proj: cgmath::perspective(cgmath::Deg(75.0f32), ASPECT_RATIO, 0.1, 4000.0).into(),
      },
      position: Position::new([0.0, 0.0]),
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &mut CharacterInputState) {
    self.projection = *world_to_clip;
    let new_position = Position::new([ci.x_movement, ci.y_movement]);
    if can_move(new_position) {
      ci.is_colliding = false;
      self.position = new_position;
      coords_to_tile(self.position);
    } else {
      ci.is_colliding = true;
    }
  }
}

impl specs::Component for TerrainDrawable {
  type Storage = specs::HashMapStorage<TerrainDrawable>;
}

const SHADER_VERT: &[u8] = include_bytes!("../shaders/terrain.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/terrain.f.glsl");

pub struct TerrainDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, tilemap_pipeline::Data<R>>,
  data: Vec<TileMapData>,
  is_tile_map_dirty: bool,
}

impl<R: gfx::Resources> TerrainDrawSystem<R> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<R, DepthFormat>)
                -> TerrainDrawSystem<R>
                where F: gfx::Factory<R> {
    use gfx::traits::FactoryExt;

    let tile_size = 32;
    let width = TILES_PCS_W;
    let height = TILES_PCS_H;
    let half_width = (tile_size * width) / 2;
    let half_height = (tile_size * height) / 2;

    let tile_sheet_bytes = &include_bytes!("../../assets/maps/terrain.png")[..];
    let plane = Plane::subdivide(width, width);
    let vertex_data: Vec<VertexData> =
      plane.shared_vertex_iter()
           .map(|vertex| {
             let (raw_x, raw_y) = cartesian_to_isometric(vertex.pos[0], vertex.pos[1]);
             let vertex_x = half_width as f32 * raw_x;
             let vertex_y = half_height as f32 * raw_y;

             let (u_pos, v_pos) = ((raw_x / 4.0 - raw_y / 2.0) + 0.5, (raw_x / 4.0 + raw_y / 2.0) + 0.5);
             let tile_map_x = u_pos * width as f32;
             let tile_map_y = v_pos * height as f32;

             VertexData {
               pos: [vertex_x, vertex_y],
               buf_pos: [tile_map_x as f32, tile_map_y as f32],
             }
           })
           .collect();

    let index_data: Vec<u32> =
      plane.indexed_polygon_iter()
           .triangulate()
           .vertices()
           .map(|i| i as u32)
           .collect();

    let (vertex_buf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

    let tile_texture = load_texture(factory, tile_sheet_bytes).unwrap();

    let pso = factory
      .create_pipeline_simple(SHADER_VERT, SHADER_FRAG, tilemap_pipeline::new())
      .unwrap();

    let pipeline_data = tilemap_pipeline::Data {
      vbuf: vertex_buf,
      position_cb: factory.create_constant_buffer(1),
      projection_cb: factory.create_constant_buffer(1),
      tilemap: factory.create_constant_buffer(TILE_MAP_BUF_LENGTH),
      tilemap_cb: factory.create_constant_buffer(1),
      tilesheet: (tile_texture, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    let terrain = tile_map::Terrain::new();

    TerrainDrawSystem {
      bundle: gfx::Bundle::new(slice, pso, pipeline_data),
      data: terrain.tiles,
      is_tile_map_dirty: true,
    }
  }

  pub fn draw<C>(&mut self,
                 drawable: &TerrainDrawable,
                 encoder: &mut gfx::Encoder<R, C>)
                 where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);
    if self.is_tile_map_dirty {
      encoder.update_buffer(&self.bundle.data.tilemap, self.data.as_slice(), 0).unwrap();
      encoder.update_constant_buffer(&self.bundle.data.tilemap_cb, &TilemapSettings {
        world_size: [64.0, 64.0],
        tilesheet_size: [32.0, 32.0],
      });
      self.is_tile_map_dirty = false
    }

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

impl<'a> specs::System<'a> for PreDrawSystem {
  type SystemData = (WriteStorage<'a, TerrainDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     WriteStorage<'a, CharacterInputState>,
                     Fetch<'a, Dimensions>);

  fn run(&mut self, (mut terrain, camera_input, mut character_input, dim): Self::SystemData) {
    use specs::Join;

    for (t, camera, ci) in (&mut terrain, &camera_input, &mut character_input).join() {
      let world_to_clip = dim.world_to_projection(camera);
      t.update(&world_to_clip, ci);
    }
  }
}
