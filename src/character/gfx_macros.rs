use gfx;

gfx_defines! {
    constant CharacterData {
        data: [f32; 4] = "data",
    }

    constant CharacterPosition {
      transform: [[f32; 4]; 4] = "transform",
    }

    constant CharacterSheetSettings {
        world_size: [f32; 4] = "u_WorldSize",
        charactersheet_size: [f32; 4] = "u_TilesheetSize",
        offsets: [f32; 2] = "u_TileOffsets",
    }

    vertex VertexData {
        pos: [f32; 3] = "a_Pos",
        buf_pos: [f32; 2] = "a_BufPos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<VertexData> = (),
        character: gfx::ConstantBuffer<CharacterData> = "b_Character",
        character_cb: gfx::ConstantBuffer<CharacterSheetSettings> = "b_PsLocals",
        charactersheet: gfx::TextureSampler<[f32; 4]> = "t_CharacterSheet",
        out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
        out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}
