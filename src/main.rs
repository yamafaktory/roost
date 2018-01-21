#![feature(custom_attribute)]

extern crate gfx_device_gl;
extern crate nalgebra;
extern crate piston_window;

mod constants;
mod direction;
mod player;
mod types;
mod stage;
mod sprite;

use constants::{BACKGROUND_COLOR, SCREEN_SIZE, SPRITE_NUMBER, SPRITE_SIZE};
use piston_window::*;
use sprite::generate_sprites;
use stage::Stage;
use types::{Sprites, Vec2, World};

fn render(
    event: Event,
    window: &mut PistonWindow,
    world: &World,
    player: &player::Player,
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

fn main() {
    // Main window settings.
    let mut window: PistonWindow =
        WindowSettings::new("roost", [SCREEN_SIZE, SCREEN_SIZE])
            .exit_on_esc(true)
            .opengl(OpenGL::V3_2)
            .resizable(false)
            .vsync(true)
            .build()
            .unwrap();
    window.set_lazy(true);

    // Instantiate the player.
    let mut player: player::Player = player::Player::new(
        direction::Direction::Neutral,
        Vec2::new(SCREEN_SIZE as f64 / 2.0, SCREEN_SIZE as f64 / 2.0),
        Vec2::new(1.0, 1.0),
        &mut window,
    );

    // Instantiate the stage.
    let stage = Stage::new();

    // Instantiate the sprites.
    let sprites: Sprites =
        generate_sprites(vec!["s", "grass", "tree-a", "tree-b"], &mut window);

    // Configure the events.
    let mut events = Events::new(EventSettings::new().max_fps(60).ups(200));

    // Main event loop.
    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            player.init_move(key);
        }

        if let Some(Button::Keyboard(_)) = event.release_args() {
            player.stop_move();
        }

        if let Some(_) = event.update_args() {
            player.update_position(&stage.world);
        }

        if let Some(_) = event.render_args() {
            render(event, &mut window, &stage.world, &player, &sprites);
        }
    }
}
