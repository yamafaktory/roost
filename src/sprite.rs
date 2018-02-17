extern crate find_folder;

use entity::Entity;
use piston_window::*;
use types::{Entities, Sprites, Tex};

pub struct Sprite {
    pub entities: Entities,
    pub sprites: Sprites,
}

impl Sprite {
    pub fn new(window: &mut PistonWindow) -> Self {
        // Sadly can't make it a constant due to current Rust limitation!
        let entities = vec![
            Entity::Traversable(""),
            Entity::Traversable("grass-a"),
            Entity::Traversable("grass-b"),
            Entity::Block("tree-a"),
            Entity::Block("tree-b"),
            Entity::Block("tree-c"),
        ];
        let sprites = generate_sprites(&entities, window);
        Self { entities, sprites }
    }
}

pub fn create_sprite(
    window: &mut PistonWindow,
    src: String,
) -> Result<Tex, String> {
    let assets = find_folder::Search::ParentsThenKids(1, 1)
        .for_folder("assets")
        .unwrap();

    return Texture::from_path(
        &mut window.factory,
        assets.join(src),
        Flip::None,
        &TextureSettings::new(),
    );
}

pub fn generate_sprites(
    sprites: &Entities,
    window: &mut PistonWindow,
) -> Sprites {
    return sprites
        .iter()
        .map(|s| {
            let ref mut s = String::from(s.to_string());
            let ext = ".png".to_string();
            s.push_str(&ext);
            return create_sprite(window, s.to_string());
        })
        .collect::<Sprites>();
}
