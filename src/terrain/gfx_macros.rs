use gfx;
use shaders::Projection;

gfx_defines! {
  constant TileMapData {
      data: [f32; 4] = "data",
  }

  constant TilemapSettings {
      world_size: [f32; 2] = "u_WorldSize",
      tilesheet_size: [f32; 2] = "u_TilesheetSize",
  }

  vertex VertexData {
      pos: [f32; 3] = "a_Pos",
      buf_pos: [f32; 2] = "a_BufPos",
  }

  constant Position {
    position: [f32; 2] = "a_position",
  }

  pipeline pipe {
      vbuf: gfx::VertexBuffer<VertexData> = (),
      position_cb: gfx::ConstantBuffer<Position> = "b_TileMapPosition",
      projection_cb: gfx::ConstantBuffer<Projection> = "b_VsLocals",
      tilemap: gfx::ConstantBuffer<TileMapData> = "b_TileMap",
      tilemap_cb: gfx::ConstantBuffer<TilemapSettings> = "b_PsLocals",
      tilesheet: gfx::TextureSampler<[f32; 4]> = "t_TileSheet",
      out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
      out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }
}
