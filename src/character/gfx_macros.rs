use gfx;

gfx_defines! {
    constant CharacterData {
        data: [f32; 4] = "data",
    }

    constant CharacterPosition {
      transform: [[f32; 4]; 4] = "transform",
    }

    vertex VertexData {
        pos: [f32; 3] = "a_Pos",
        buf_pos: [f32; 2] = "a_BufPos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<VertexData> = (),
        character: gfx::ConstantBuffer<CharacterData> = "b_Character",
        character_cb: gfx::ConstantBuffer<CharacterPosition> = "b_PsLocals",
        charactersheet: gfx::TextureSampler<[f32; 4]> = "t_CharacterSheet",
        out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
        out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}
