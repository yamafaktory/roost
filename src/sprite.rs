extern crate find_folder;

use piston_window::*;
use types::{Sprites, Tex};

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
    sprites: Vec<&'static str>,
    window: &mut PistonWindow,
) -> Sprites {
    return sprites
        .iter()
        .map(|&s| {
            let mut s = s.to_string();
            let ext = ".png".to_string();
            s.push_str(&ext);
            return create_sprite(window, s);
        })
        .collect::<Sprites>();
}
