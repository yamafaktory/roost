#![feature(custom_attribute)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(unknown_lints, ptr_arg)]

extern crate gfx_device_gl;
extern crate nalgebra;
extern crate piston_window;

mod constants;
mod collision;
mod direction;
mod either;
mod entity;
mod player;
mod render;
mod types;
mod stage;
mod sprite;

use constants::SCREEN_SIZE;
use piston_window::*;
use render::render;
use sprite::Sprite;
use stage::Stage;
use types::{Vec2};

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
        Vec2::new(f64::from(SCREEN_SIZE) / 2.0, f64::from(SCREEN_SIZE) / 2.0),
        Vec2::new(1.0, 1.0),
        &mut window,
    );

    // Instantiate the stage.
    let stage = Stage::new();

    // Instantiate the sprites.
    let sprite = Sprite::new(&mut window);

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

        if event.update_args().is_some() {
            player.update_position(&stage.world, &sprite.entities);
        }

        if event.render_args().is_some() {
            render(&event, &mut window, &stage.world, &player, &sprite.sprites);
        }
    }
}
