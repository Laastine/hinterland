use terrain::gfx_macros::{TilemapSettings, Projection, VertexData, pipe, TileMapData};
use std::io::Cursor;
use gfx;
use image;
use cgmath;
use gfx_app::{WindowTargets}; //
use gfx::{Resources, Factory, texture};
use gfx::handle::{ShaderResourceView};
use gfx::format::{Rgba8};
use gfx::traits::{FactoryExt};
use cgmath::{SquareMatrix, Matrix4, AffineMatrix3};
use cgmath::{Point3, Vector3};
use cgmath::{Transform};
use game::constants::{MAP_FILE_PATH};
use genmesh::{Vertices, Triangulate};
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use tilemap::{TileMap};
use data::{load_map_file, load_character, load_zombie, get_map_tile};


const TILEMAP_BUF_LENGTH: usize = 4096;

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

pub struct TileMapPlane<R> where R: Resources {
  pub params: pipe::Data<R>,
  pub slice: gfx::Slice<R>,
  projection: Projection,
  is_projection_dirty: bool,
  tilemap_settings: TilemapSettings,
  is_tilemap_dirty: bool,
  pub data: Vec<TileMapData>,
}

impl<R> TileMapPlane<R> where R: Resources {
  pub fn new<F>(factory: &mut F, width: usize, height: usize, tile_size: usize,
                targets: WindowTargets<R>)
                -> TileMapPlane<R> where F: gfx::Factory<R> {
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

    let params = pipe::Data {
      vbuf: vertex_buf,
      projection_cb: factory.create_constant_buffer(1),
      tilemap: factory.create_constant_buffer(TILEMAP_BUF_LENGTH),
      tilemap_cb: factory.create_constant_buffer(1),
      tilesheet: (tile_texture, factory.create_sampler_linear()),
      out_color: targets.color,
      out_depth: targets.depth,
    };

    let mut charmap_data = Vec::with_capacity(total_size);

    for _ in 0..total_size {
      charmap_data.push(TileMapData::new_empty());
    }

    let view: AffineMatrix3<f32> = Transform::look_at(
      Point3::new(0.0, 0.0, 2000.0),
      Point3::new(0.0, 0.0, 0.0),
      Vector3::unit_y(),
    );

    TileMapPlane {
      slice: slice,
      params: params,
      projection: Projection {
        model: Matrix4::identity().into(),
        view: view.mat.into(),
        proj: cgmath::perspective(cgmath::deg(60.0f32), targets.aspect_ratio, 0.1, 4000.0).into(),
      },
      is_projection_dirty: true,
      tilemap_settings: TilemapSettings {
        world_size: [width as f32, height as f32, tile_size as f32, 0.0],
        tilesheet_size: [tilesheet_width as f32, tilesheet_height as f32, tilesheet_total_width as f32, tilesheet_total_height as f32],
        offsets: [0.0, 0.0],
      },
      is_tilemap_dirty: true,
      data: charmap_data,
    }
  }

  pub fn resize(&mut self, targets: WindowTargets<R>) {
    self.params.out_color = targets.color;
    self.params.out_depth = targets.depth;
    self.projection.proj = cgmath::perspective(cgmath::deg(60.0f32), targets.aspect_ratio, 0.1, 4000.0).into();
    self.is_projection_dirty = true;
  }

  pub fn prepare_buffers<C>(&mut self, encoder: &mut gfx::Encoder<R, C>, update_data: bool) where C: gfx::CommandBuffer<R> {
    if update_data {
      encoder.update_buffer(&self.params.tilemap, self.data.as_slice(), 0).unwrap();  //vertex data
    }
    if self.is_projection_dirty {
      encoder.update_constant_buffer(&self.params.projection_cb, &self.projection);
      self.is_projection_dirty = false;
    }
    if self.is_tilemap_dirty {
      encoder.update_constant_buffer(&self.params.tilemap_cb, &self.tilemap_settings);
      self.is_tilemap_dirty = false;
    }
  }

  pub fn clear<C>(&self, encoder: &mut gfx::Encoder<R, C>) where C: gfx::CommandBuffer<R> {
    encoder.clear(&self.params.out_color,
                  [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0]);
    encoder.clear_depth(&self.params.out_depth, 1.0);
  }

  pub fn update_view(&mut self, view: &AffineMatrix3<f32>) {
    self.projection.view = view.mat.into();
    self.is_projection_dirty = true;
  }
}

pub struct Tilemap<R> where R: Resources {
  pub limit_coords: [usize; 2],
  pub focus_coords: [usize; 2],
  tilemap_size: [usize; 2],
  tilemap_plane: TileMapPlane<R>,
  focus_dirty: bool,
  tiles: Vec<usize>,
}

impl<R: gfx::Resources> TileMap<R> {
  pub fn set_focus(&mut self, focus: [usize; 2]) {
    if focus[0] <= self.limit_coords[0] && focus[1] <= self.limit_coords[1] {
      self.focus_coords = focus;
      let mut charmap_ypos = 0;
      for ypos in self.focus_coords[1]..self.focus_coords[1] + self.charmap_size[1] {
        let mut charmap_xpos = 0;
        for xpos in self.focus_coords[0]..self.focus_coords[0] + self.charmap_size[0] {
          let tile_idx = (ypos * self.tilemap_size[0]) + xpos;
          let charmap_idx = (charmap_ypos * self.charmap_size[0]) + charmap_xpos;
          self.tilemap_plane.data[charmap_idx] = self.tiles[tile_idx];
          charmap_xpos += 1;
        }
        charmap_ypos += 1;
      }
      self.focus_dirty = true;
    } else {
      panic!("tried to set focus to {:?} with tilemap_size of {:?}", focus, self.tilemap_size);
    }
  }

  fn calc_index(&self, xpos: usize, ypos: usize) -> usize {
    (ypos * self.tilemap_size[0]) + xpos
  }

  pub fn set_tile(&mut self, xpos: usize, ypos: usize, data: [f32; 4]) {
    let idx = self.calc_index(xpos, ypos);
    self.tiles[idx] = TileMapData::new(data);
  }

  pub fn populate_tilemap(&mut self, tilemap_size: [usize; 2]) {
    let map = load_map_file(MAP_FILE_PATH);
    for ypos in 0..self.tilemap_size[1] {
      for xpos in 0..self.tilemap_size[0] {
        let map_val = get_map_tile(&map, 0, xpos as usize, ypos as usize);
        let tex_x = map_val % 32;
        let tex_y = (map_val - tex_x) / 32;
        self.set_tile(xpos, ypos, [(tex_x-1) as f32, tex_y as f32, 0.0, 0.0]);
      }
    }
  }

  pub fn load_player(&mut self) {
    let player = load_character();
    println!("player {:?}", player);
    let zombie = load_zombie();
    println!("zombie {:?}", zombie);
  }
}
