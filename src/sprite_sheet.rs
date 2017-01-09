use std::path::Path;

use sdl2::render::Texture;
use sdl2::image::LoadTexture;

use canvas::Canvas;

error_chain! {
    errors {
        TileWidthZero
        TileHeightZero
        FileNotFound
    }
}


pub struct SpriteSheet {
    tile_width: u32,
    tile_height: u32,
    image_frames: Vec<Texture>,
}

impl SpriteSheet {
    pub fn new(sheet_file: &str, tile_width: u32, tile_height: u32, canvas: &Canvas) -> Result<SpriteSheet> {
        let image_frames = Vec::new();

        let sheet = canvas.renderer.load_texture(Path::new(sheet_file))?;

        if tile_width <= 1 { return Err(ErrorKind::TileWidthZero.into()); }
        if tile_height <= 1 { return Err(ErrorKind::TileHeightZero.into()); }

        Ok(SpriteSheet {
            tile_width: tile_width,
            tile_height: tile_height,
            image_frames: image_frames,
        })
    }
}
