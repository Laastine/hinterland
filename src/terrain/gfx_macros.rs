use gfx;
use gfx_app::ColorFormat;

gfx_defines! {
    constant TileMapData {
        data: [f32; 4] = "data",
    }

    constant Bounds {
        data: [[f32; 4]; 4] = "bounds",
    }

    constant Projection {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    constant TilemapSettings {
        world_size: [f32; 4] = "u_WorldSize",
        tilesheet_size: [f32; 4] = "u_TilesheetSize",
        offsets: [f32; 2] = "u_TileOffsets",
    }

    vertex VertexData {
        pos: [f32; 3] = "a_Pos",
        buf_pos: [f32; 2] = "a_BufPos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<VertexData> = (),
        projection_cb: gfx::ConstantBuffer<Projection> = "b_VsLocals",
        bounds: gfx::ConstantBuffer<Bounds> = "Bounds",
        tilemap: gfx::ConstantBuffer<TileMapData> = "b_TileMap",
        tilemap_cb: gfx::ConstantBuffer<TilemapSettings> = "b_PsLocals",
        tilesheet: gfx::TextureSampler<[f32; 4]> = "t_TileSheet",
//        out: gfx::RenderTarget<ColorFormat> = "out_color",
        out_color: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
        out_depth: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}
