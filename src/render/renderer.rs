use gfx;


#[derive(Debug)]
pub struct TextureRenderer {
    pso: gfx::PipelineState,
    ss: SpriteSheet,
}

impl TextureRenderer {
    pub fn new(ss: SpriteSheet) -> Self {
        let pso = factory
            .create_pipeline_simple(
                include_bytes!("shaders/simple_texture_150.glslv"),
                include_bytes!("shaders/simple_texture_150.glslf"),
                pipe::new(),
            )
            .unwrap();
        TextureRenderer { pso, ss }
    }

    pub fn draw(&self, encoder: Encoder, vbuffer: Buffer<R, Vec<Vertex>>, slice: Slice<R>) {
        let mut data = pipe::Data {};
    }
}
