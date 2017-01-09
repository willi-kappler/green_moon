use std::path::Path;

use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use canvas::Canvas;

error_chain! {
    errors {
        TileWidthZero
        TileHeightZero
        TileWidthDoesntFit
        TileHeightDoesntFit
        FileNotFound
        RenderTargetError
        RenderTargetReserError
    }
}


pub struct SpriteSheet {
    tile_width: u32,
    tile_height: u32,
    sheet_file: String,
    image_frames: Vec<Texture>,
    resource_loaded: bool,
}

impl SpriteSheet {
    pub fn new(sheet_file: &str, tile_width: u32, tile_height: u32) -> Result<SpriteSheet> {
        if tile_width <= 1 { return Err(ErrorKind::TileWidthZero.into()); }
        if tile_height <= 1 { return Err(ErrorKind::TileHeightZero.into()); }

        Ok(SpriteSheet {
            tile_width: tile_width,
            tile_height: tile_height,
            sheet_file: sheet_file.to_string(),
            image_frames: Vec::new(),
            resource_loaded: false,
        })
    }

    pub fn load_resources(&mut self, canvas: &mut Canvas) -> Result<()>  {
        let sheet = canvas.renderer.load_texture(Path::new(&self.sheet_file))?;
        let properties = sheet.query();

        if properties.width % self.tile_width != 0 { return Err(ErrorKind::TileWidthDoesntFit.into()); }
        if properties.height % self.tile_height != 0 { return Err(ErrorKind::TileHeightDoesntFit.into()); }

        let mut x = 0;
        let mut y = 0;

        while x <= properties.width {
            while y <= properties.height {
                {
                    let mut render_target = canvas.renderer.render_target().ok_or(ErrorKind::RenderTargetError)?;
                    render_target.create_and_set(PixelFormatEnum::RGB24, self.tile_width,
                        self.tile_height).chain_err(|| "Could not create new texture from canvas renderer")?;
                }
                let _ = canvas.renderer.copy(&sheet, Some(Rect::new(x as i32, y as i32, self.tile_width, self.tile_height)), None);
                {
                    let mut render_target = canvas.renderer.render_target().ok_or(ErrorKind::RenderTargetError)?;
                    let new_texture = render_target.reset()?.ok_or(ErrorKind::RenderTargetReserError)?;
                    self.image_frames.push(new_texture);
                }

                x += self.tile_width;
                y += self.tile_height;
            }
        }

        self.resource_loaded = true;

        Ok(())
    }
}
