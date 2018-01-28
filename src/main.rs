#![feature(custom_attribute)]

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
        Vec2::new(SCREEN_SIZE as f64 / 2.0, SCREEN_SIZE as f64 / 2.0),
        Vec2::new(1.0, 1.0),
        &mut window,
    );

    // Instantiate the stage.
    let stage = Stage::new();

    // Instantiate the sprites.
    let sprites = Sprite::new(&mut window).sprites;

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
