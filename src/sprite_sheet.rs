use std::path::Path;

use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::surface::Surface;
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
    image_frames: Vec<Texture>,
}

impl SpriteSheet {
    pub fn new(sheet_file: &str, tile_width: u32, tile_height: u32, canvas: &mut Canvas) -> Result<SpriteSheet> {
        let mut image_frames = Vec::new();

        if tile_width <= 1 { return Err(ErrorKind::TileWidthZero.into()); }
        if tile_height <= 1 { return Err(ErrorKind::TileHeightZero.into()); }

        let sheet = canvas.renderer.load_texture(Path::new(sheet_file))?;
        let properties = sheet.query();

        if properties.width % tile_width != 0 { return Err(ErrorKind::TileWidthDoesntFit.into()); }
        if properties.height % tile_height != 0 { return Err(ErrorKind::TileHeightDoesntFit.into()); }

        let mut x = 0;
        let mut y = 0;

        while x <= properties.width {
            while y <= properties.height {
                {
                    let mut render_target = canvas.renderer.render_target().ok_or(ErrorKind::RenderTargetError)?;
                    render_target.create_and_set(PixelFormatEnum::RGB24, tile_width,
                        tile_height).chain_err(|| "Could not create new texture from canvas renderer")?;
                }
                canvas.renderer.copy(&sheet, Some(Rect::new(x as i32, y as i32, tile_width, tile_height)), None);
                {
                    let mut render_target = canvas.renderer.render_target().ok_or(ErrorKind::RenderTargetError)?;
                    let new_texture = render_target.reset()?.ok_or(ErrorKind::RenderTargetReserError)?;
                    image_frames.push(new_texture);
                }

                x += tile_width;
                y += tile_height;
            }
        }

        Ok(SpriteSheet {
            tile_width: tile_width,
            tile_height: tile_height,
            image_frames: image_frames,
        })
    }

    pub fn empty() -> SpriteSheet {
        SpriteSheet {
            tile_width: 0,
            tile_height: 0,
            image_frames: Vec::new(),
        }
    }
}
