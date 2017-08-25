use gfx;
use game::gfx_macros::Projection;

gfx_defines! {
  constant CharacterIdx {
    idx: f32 = "idx",
  }

  vertex VertexData {
    pos: [f32; 3] = "a_Pos",
    buf_pos: [f32; 2] = "a_BufPos",
  }

  pipeline pipe {
    vbuf: gfx::VertexBuffer<VertexData> = (),
    projection_cb: gfx::ConstantBuffer<Projection> = "b_VsLocals",
    charactersheet: gfx::TextureSampler<[f32; 4]> = "t_CharacterSheet",
    character_idx: gfx::ConstantBuffer<CharacterIdx> = "t_CharacterIdx",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }
}
