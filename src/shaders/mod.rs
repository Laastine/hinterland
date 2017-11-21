use gfx;

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

  constant CharacterSheet {
    div: f32 = "a_div",
    index: f32 = "a_index",
  }

  constant Position {
    position: [f32; 2] = "a_position",
  }

  pipeline bullet_pipeline {
    vbuf: gfx::VertexBuffer<VertexData> = (),
    projection_cb: gfx::ConstantBuffer<Projection> = "b_VsLocals",
    position_cb: gfx::ConstantBuffer<Position> = "b_CharacterPosition",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }

  pipeline critter_pipeline {
    vbuf: gfx::VertexBuffer<VertexData> = (),
    projection_cb: gfx::ConstantBuffer<Projection> = "b_VsLocals",
    position_cb: gfx::ConstantBuffer<Position> = "b_CharacterPosition",
    character_sprite_cb: gfx::ConstantBuffer<CharacterSheet> = "b_CharacterSprite",
    charactersheet: gfx::TextureSampler<[f32; 4]> = "t_CharacterSheet",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }

  pipeline tilemap_pipeline {
    vbuf: gfx::VertexBuffer<VertexData> = (),
    position_cb: gfx::ConstantBuffer<Position> = "b_TileMapPosition",
    projection_cb: gfx::ConstantBuffer<Projection> = "b_VsLocals",
    tilemap: gfx::ConstantBuffer<TileMapData> = "b_TileMap",
    tilemap_cb: gfx::ConstantBuffer<TilemapSettings> = "b_PsLocals",
    tilesheet: gfx::TextureSampler<[f32; 4]> = "t_TileSheet",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }

  constant Projection {
    model: [[f32; 4]; 4] = "u_Model",
    view: [[f32; 4]; 4] = "u_View",
    proj: [[f32; 4]; 4] = "u_Proj",
  }
}

impl VertexData {
  pub fn new(pos: [f32; 3], buf_pos: [f32; 2]) -> VertexData {
    VertexData {
      pos,
      buf_pos,
    }
  }
}
