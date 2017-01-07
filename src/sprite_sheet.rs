
error_chain! {
    errors {
        TileWidthZero
        TileHeightZero
    }
}


pub struct SpriteSheet {
    tile_width: u32,
    tile_height: u32,
}

impl SpriteSheet {
    pub fn new(sheet_file: &str, tile_width: u32, tile_height: u32) -> Result<SpriteSheet> {
        Ok(SpriteSheet {
            tile_width: tile_width,
            tile_height: tile_height
        })
    }
}
