use constants::{SCREEN_SIZE, SPRITE_SIZE, STEP};
use direction::Direction;
use piston_window::Key;
use types::{Tex, Vec2, World};

pub struct Player {
    pub direction: Direction,
    pub position: Vec2,
    pub scale: Vec2,
    pub sprite: Result<Tex, String>,
}

impl Player {
    pub fn new(
        direction: Direction,
        position: Vec2,
        scale: Vec2,
        sprite: Result<Tex, String>,
    ) -> Self {
        Self {
            direction,
            position,
            scale,
            sprite,
        }
    }
    fn get_step(&self) -> f64 { STEP }
    pub fn get_position(&self) -> (f64, f64) {
        (self.position.x, self.position.y)
    }
    pub fn init_move(&mut self, key: Key) -> () {
        match key {
            Key::Left => {
                self.direction = Direction::Left;
            }
            Key::Right => {
                self.direction = Direction::Right;
            }
            Key::Up => {
                self.direction = Direction::Up;
            }
            Key::Down => {
                self.direction = Direction::Down;
            }
            _ => (),
        };
    }
    pub fn stop_move(&mut self) -> () { self.direction = Direction::Neutral }
    fn pos_to_matrix(&self) -> (f64, f64) {
        (
            (self.position.x / SPRITE_SIZE).round(),
            (self.position.y / SPRITE_SIZE).round(),
        )
    }
    fn is_colliding_world(&self, world: &World) -> bool {
        let (x, y) = self.pos_to_matrix();
        world.row(x as usize)[y as usize] != 0
    }
    pub fn update_position(&mut self, world: &World) -> () {
        let (half_sprite_size, screen_size, step, mut next_position) = (
            SPRITE_SIZE / 2.0,
            SCREEN_SIZE as f64,
            self.get_step(),
            self.position,
        );

        match self.direction {
            Direction::Left => if self.position.x > 0.0 {
                next_position.x -= step
            },
            Direction::Right => {
                if self.position.x < screen_size - half_sprite_size {
                    next_position.x += step;
                }
            }
            Direction::Up => if self.position.y > 0.0 {
                next_position.y -= step;
            },
            Direction::Down => {
                if self.position.y < screen_size - half_sprite_size {
                    next_position.y += step;
                }
            }
            Direction::Neutral => {}
        }

        println!("------{:?}", self.is_colliding_world(world));

        // Update the user position.
        self.position = next_position;
    }
}
