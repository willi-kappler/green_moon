
use sprite_sheet::SpriteSheet;

pub struct ResourceManager {
    title: String,
    window_width: u16,
    window_height: u16,

    sprite_sheets: Vec<SpriteSheet>,
}

impl ResourceManager {
    pub fn new(title: &str, window_width: u16, window_height: u16) -> ResourceManager {
        ResourceManager {
            title: title.to_string(),
            window_width: window_width,
            window_height: window_height,

            sprite_sheets: Vec::new(),
        }
    }

    pub fn add_sprite_sheet(&self, file_name: &str, tile_width: u16, tile_height: u16) {
        // TODO
    }

    pub fn load_resources(&self, file_name: &str) {
        // TODO
    }
}
