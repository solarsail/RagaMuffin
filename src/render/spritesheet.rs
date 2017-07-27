use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use device_gl::Resources as R;
use geometry::Rect;
use render::ColorFormat;


type SpriteMap = HashMap<String, (Rect, Rect)>;

#[derive(Deserialize)]
struct SpriteSheetInfo {
    image: String,
    sprites: SpriteMap,
}

pub struct SpriteSheet {
    sprite_map: SpriteMap,
    texture_data: gfx::handle::ShaderResourceView<R, ColorFormat::View>,
}

fn missing_ss() -> (String, SpriteMap) {
    ("atlas.png".into(), HashMap::new())
}

fn parse_ss_file(file: String) -> (String, SpriteMap) {
    let file = match File::open(file) {
        Err(why) => {
            //warning!(why.description());
            return missing_ss();
        }
        Ok(f) => f,
    };
    // deserialize file into SpriteSheetInfo

}

impl SpriteSheet {
    pub fn new(file: String) -> Self {
        let (ssfile, sm) = parse_ss_file(file);
        SpriteSheet {
            sprite_map: sm,
            texture_data: render::load_texture(ssfile),
        }
    }
}
