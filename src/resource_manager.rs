
use sprite_sheet::SpriteSheet;

pub struct ResourceManager {
    pub title: String,
    pub window_width: u32,
    pub window_height: u32,

    pub sprite_sheets: Vec<SpriteSheet>,
}

impl ResourceManager {
    pub fn new(title: &str, window_width: u32, window_height: u32) -> ResourceManager {
        ResourceManager {
            title: title.to_string(),
            window_width: window_width,
            window_height: window_height,

            sprite_sheets: Vec::new(),
        }
    }

    pub fn add_sprite_sheet(&mut self, file_name: &str, tile_width: u16, tile_height: u16) {
        self.sprite_sheets.push(SpriteSheet{ tile_width: tile_width, tile_height: tile_height });
    }

    pub fn load_resources(&mut self) {
        // TODO
    }

    pub fn import_resources(&mut self, file_name: &str) {
        // TODO
    }
}
