
// External
use sdl2;

// Internal
use game::Game;
use game_manager::GameManager;
use canvas::Canvas;
use scene::Scene;
use scene_manager::SceneManager;
use sprite::Sprite;

error_chain! {
    errors {
        ScenesUndefined
        WidthUndefined
        HeightUndefined
        NameUndefined
    }
}

pub struct GameBuilder<'a> {
    width: Option<u32>,
    height: Option<u32>,
    name: Option<String>,

    all_scenes: Vec<Box<Scene>>,
    start_scene: usize,

    all_sprites: Vec<Sprite<'a>>,
}

impl<'a> GameBuilder<'a> {
    pub fn new() -> GameBuilder<'a> {
        GameBuilder {
            width: Some(800),
            height: Some(600),
            name: Some("Made with Green Moon".to_string()),

            all_scenes: Vec::new(),
            start_scene: 0,
            all_sprites: Vec::new(),
        }
    }

    pub fn size(mut self, width: u32, height: u32) -> GameBuilder<'a> {
        self.width = Some(width);
        self.height = Some(height);

        self
    }

    pub fn name(mut self, name: &str) -> GameBuilder<'a> {
        self.name = Some(name.to_string());

        self
    }

    pub fn add_scene<T: Scene + 'static>(mut self, scene: T) -> GameBuilder<'a> {
        self.all_scenes.push(Box::new(scene));

        self
    }

    pub fn start_scene(mut self, scene_id: usize) -> GameBuilder<'a> {
        self.start_scene = scene_id;

        self
    }

    pub fn add_sprite(mut self, sprite: Sprite<'a>) -> GameBuilder<'a> {
        self.all_sprites.push(sprite);

        self
    }

    pub fn build(self) -> Result<GameManager<'a>> {
        match self {
            GameBuilder { all_scenes: ref scenes, .. } if scenes.is_empty() => Err(ErrorKind::ScenesUndefined.into()),

            _ => {
                let width = self.width.ok_or(ErrorKind::WidthUndefined)?;
                let height = self.height.ok_or(ErrorKind::HeightUndefined)?;
                let name = self.name.ok_or(ErrorKind::NameUndefined)?;

                let context = sdl2::init()?;
                let video_subsystem = context.video()?;
                let window = video_subsystem.window(&name, width, height)
                    .position_centered().build().chain_err(|| "Could not open SDL2 window")?;

                let renderer = window.renderer().accelerated().build().chain_err(|| "Could not open SDL2 window")?;

                let event_pump = context.event_pump()?;

                let game = Game {
                    width: width,
                    height: height,
                    name: name,

                    canvas: Canvas { renderer: renderer },
                    all_sprites: self.all_sprites,

                    context: context,
                    event_pump: event_pump,
                };

                let scene_manager = SceneManager {
                    all_scenes: self.all_scenes,
                    current_scene: self.start_scene,
                };

                Ok(GameManager {
                    scene_manager: scene_manager,
                    game: game,
                })
            }
        }
    }
}
