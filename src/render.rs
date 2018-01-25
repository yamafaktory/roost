use constants::{BACKGROUND_COLOR, SPRITE_NUMBER, SPRITE_SIZE};
use piston_window::*;
use player::Player;
use types::{Sprites, World};

pub fn render(
    event: Event,
    window: &mut PistonWindow,
    world: &World,
    player: &Player,
    sprites: &Sprites,
) -> () {
    let ((x, y), scale) = (player.get_position(), player.scale);

    window.draw_2d(&event, |context, graphics| {
        clear(BACKGROUND_COLOR, graphics);

        // Render the player sprite.
        match player.sprites[player.sprite_index] {
            Ok(ref sprite) => {
                image(
                    sprite,
                    context.transform.trans(x, y).scale(scale.x, scale.y),
                    graphics,
                );
            }
            Err(ref e) => println!("Player sprite error: {:?}", e),
        }

        // Render the world sprites.
        for row in 0..SPRITE_NUMBER {
            for (column, value) in world.row(row).iter().enumerate() {
                // Skip zero values.
                if *value == 0 {
                    continue;
                }

                // Map images to matrix values.
                if let Some(i) = sprites.get(*value as usize) {
                    match *i {
                        Ok(ref sprite) => {
                            image(
                                sprite,
                                context
                                    .transform
                                    .trans(
                                        column as f64 * SPRITE_SIZE,
                                        row as f64 * SPRITE_SIZE,
                                    )
                                    .scale(1.0, 1.0),
                                graphics,
                            );
                        }
                        Err(ref e) => println!("World sprites error: {:?}", e),
                    }
                }
            }
        }
    });
}
