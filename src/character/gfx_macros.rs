use gfx;
use game::gfx_macros::Projection;

gfx_defines! {
  vertex VertexData {
    pos: [f32; 3] = "a_Pos",
    buf_pos: [f32; 2] = "a_BufPos",
  }

  constant CharacterSheet {
    div: f32 = "a_div",
    index: f32 = "a_index",
  }

  pipeline pipe {
    vbuf: gfx::VertexBuffer<VertexData> = (),
    projection_cb: gfx::ConstantBuffer<Projection> = "b_VsLocals",
    character_sprite_cb: gfx::ConstantBuffer<CharacterSheet> = "b_CharacterSprite",
    charactersheet: gfx::TextureSampler<[f32; 4]> = "t_CharacterSheet",
    out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }
}
