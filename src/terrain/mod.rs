use cgmath::Point2;
use character::controls::CharacterInputState;
use game::constants::{ASPECT_RATIO, TILE_MAP_BUF_LENGTH, TILES_PCS_H, TILES_PCS_W, VIEW_DISTANCE};
use genmesh::{generators::{IndexedPolygon, Plane, SharedVertex}, Triangulate, Vertices};
use gfx;
use gfx_app::{ColorFormat, DepthFormat};
use graphics::{camera::CameraInputState, can_move_to_tile, coords_to_tile, dimensions::{Dimensions, get_projection, get_view_matrix}, is_map_edge};
use graphics::mesh::Mesh;
use graphics::texture::{load_texture, Texture};
use shaders::{Position, Projection, tilemap_pipeline, TilemapSettings, VertexData};
use specs;
use specs::prelude::{Read, ReadStorage, WriteStorage};

pub mod path_finding;
mod path_finding_test;
pub mod tile_map;

fn cartesian_to_isometric(point_x: f32, point_y: f32) -> (f32, f32) {
  ((point_x - point_y), (point_x + point_y) / 1.78)
}

#[derive(Debug)]
pub struct TerrainDrawable {
  projection: Projection,
  pub position: Position,
  pub tile_position: Point2<i32>,
}

impl TerrainDrawable {
  pub fn new() -> TerrainDrawable {
    let view = get_view_matrix(VIEW_DISTANCE);
    let projection = get_projection(view, ASPECT_RATIO);
    TerrainDrawable {
      projection,
      position: Position::origin(),
      tile_position: coords_to_tile(Position::origin())
    }
  }

  pub fn update(&mut self, world_to_clip: &Projection, ci: &mut CharacterInputState) {
    self.projection = *world_to_clip;
    if can_move_to_tile(ci.movement) {
      ci.is_colliding = false;
      self.position = ci.movement;
      self.tile_position = coords_to_tile(self.position);
    } else {
      ci.is_colliding = true;
    }
  }
}

impl specs::prelude::Component for TerrainDrawable {
  type Storage = specs::storage::HashMapStorage<TerrainDrawable>;
}

const SHADER_VERT: &[u8] = include_bytes!("../shaders/terrain.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/terrain.f.glsl");

pub struct TerrainDrawSystem<R: gfx::Resources> {
  bundle: gfx::pso::bundle::Bundle<R, tilemap_pipeline::Data<R>>,
  terrain: tile_map::Terrain,
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

    let plane = Plane::subdivide(width, width);
    let vertex_data: Vec<VertexData> =
      plane.shared_vertex_iter()
           .map(|vertex| {
             let (raw_x, raw_y) = cartesian_to_isometric(vertex.pos[0], vertex.pos[1]);
             let vertex_x = half_width as f32 * raw_x;
             let vertex_y = half_height as f32 * raw_y;

             let (u_pos, v_pos) = ((raw_x / 4.0 - raw_y / 2.25) + 0.5, (raw_x / 4.0 + raw_y / 2.25) + 0.5);
             let tile_map_x = u_pos * width as f32;
             let tile_map_y = v_pos * height as f32;

             VertexData {
               pos: [vertex_x, vertex_y],
               uv: [tile_map_x as f32, tile_map_y as f32],
             }
           })
           .collect();

    let index_data =
      plane.indexed_polygon_iter()
           .triangulate()
           .vertices()
           .map(|i| i as u16)
           .collect::<Vec<u16>>();

    let tile_sheet_bytes = &include_bytes!("../../assets/maps/terrain.png")[..];
    let tile_texture = load_texture(factory, tile_sheet_bytes);

    let mesh = Mesh::new(factory, &vertex_data.as_slice(), index_data.as_slice(), Texture::new(tile_texture, None));

    let pso = factory.create_pipeline_simple(SHADER_VERT, SHADER_FRAG, tilemap_pipeline::new())
                     .map_err(|err| panic!("Terrain shader loading error {:?}", err))
                     .unwrap();

    let pipeline_data = tilemap_pipeline::Data {
      vbuf: mesh.vertex_buffer,
      position_cb: factory.create_constant_buffer(1),
      projection_cb: factory.create_constant_buffer(1),
      tilemap: factory.create_constant_buffer(TILE_MAP_BUF_LENGTH),
      tilemap_cb: factory.create_constant_buffer(1),
      tilesheet: (mesh.texture.raw, factory.create_sampler_linear()),
      out_color: rtv,
      out_depth: dsv,
    };

    let terrain = tile_map::Terrain::new();

    TerrainDrawSystem {
      bundle: gfx::Bundle::new(mesh.slice, pso, pipeline_data),
      terrain,
      is_tile_map_dirty: true,
    }
  }

  pub fn draw<C>(&mut self,
                 drawable: &TerrainDrawable,
                 encoder: &mut gfx::Encoder<R, C>)
                 where C: gfx::CommandBuffer<R> {
    encoder.update_constant_buffer(&self.bundle.data.projection_cb, &drawable.projection);
    encoder.update_constant_buffer(&self.bundle.data.position_cb, &drawable.position);

    self.handle_map_update(drawable);

    if self.is_tile_map_dirty {
      encoder.update_buffer(&self.bundle.data.tilemap, self.terrain.tiles.as_slice(), 0).unwrap();
      encoder.update_constant_buffer(&self.bundle.data.tilemap_cb, &TilemapSettings {
        world_size: [TILES_PCS_W as f32, TILES_PCS_H as f32],
        tilesheet_size: [32.0, 32.0],
      });
      self.is_tile_map_dirty = false
    }

    self.bundle.encode(encoder);
  }

  fn handle_map_update(&mut self, drawable: &TerrainDrawable) {
    if is_map_edge(drawable.tile_position) {
      let map_idx = self.terrain.curr_tile_set_idx;
      if map_idx == 0 {
        self.terrain.change_map(1);
      } else if map_idx == 1 {
        self.terrain.change_map(0);
      }
      self.is_tile_map_dirty = true
    }
  }
}

#[derive(Debug)]
pub struct PreDrawSystem;

impl PreDrawSystem {
  pub fn new() -> PreDrawSystem {
    PreDrawSystem {}
  }
}

impl<'a> specs::prelude::System<'a> for PreDrawSystem {
  type SystemData = (WriteStorage<'a, TerrainDrawable>,
                     ReadStorage<'a, CameraInputState>,
                     WriteStorage<'a, CharacterInputState>,
                     Read<'a, Dimensions>);

  fn run(&mut self, (mut terrain, camera_input, mut character_input, dim): Self::SystemData) {
    use specs::join::Join;

    for (t, camera, ci) in (&mut terrain, &camera_input, &mut character_input).join() {
      let world_to_clip = dim.world_to_projection(camera);
      t.update(&world_to_clip, ci);
    }
  }
}
