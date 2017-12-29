use constants::{SCREEN_SIZE, SPRITE_NUMBER, SPRITE_SIZE, STEP};
use direction::Direction;
use piston_window::Key;
use types::{Tex, Vec2, World};

pub struct Player {
    pub direction: Direction,
    pub next_position: Vec2,
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
            next_position: position,
            position,
            scale,
            sprite,
        }
    }
    fn get_step(&self) -> f64 { STEP }
    pub fn get_position(&self) -> (f64, f64) {
        (self.position.x, self.position.y)
    }
    pub fn init_move(&mut self, key: Key) {
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
    pub fn stop_move(&mut self) { self.direction = Direction::Neutral }
    fn pos_to_matrix(&self) -> (usize, usize) {
        let half_sprite_size = SPRITE_SIZE / 2.0;
        println!("({}, {})", self.next_position.y, self.next_position.x);
        return (
            (((self.next_position.y + half_sprite_size) / SPRITE_SIZE).ceil()
                - 1.0) as usize,
            (((self.next_position.x + half_sprite_size) / SPRITE_SIZE).ceil()
                - 1.0) as usize,
        );
    }
    fn will_collide_world(&self, world: &World) -> bool {
        let (row, column) = self.pos_to_matrix();
        println!("[{}, {}]", row, column);
        // Extract a slice of the matrix to get the surroundings.
        let surroundings = match (row, column) {
            (0, 1...8) => world.slice((0, column - 1), (2, 3)),
            (0, 9) => world.slice((0, column - 1), (2, 2)),
            (1...8, 9) => world.slice((row - 1, column - 1), (3, 2)),

            (1...8, 0) => world.slice((row - 1, 0), (3, 2)),
            (9, 0) => world.slice((row - 1, 0), (2, 2)),
            (9, 1...8) => world.slice((row - 1, column - 1), (2, 3)),

            (0, 0) => world.slice((0, 0), (2, 2)),
            (9, 9) => world.slice((row - 1, column - 1), (2, 2)),

            (_, _) => world.slice((row - 1, column - 1), (3, 3)),
        };
        let mut obstacles: u8 = 0;
        for row in 0..surroundings.column(0).iter().count() {
            for (_, value) in surroundings.row(row).iter().enumerate() {
                obstacles += value;
            }
        }
        println!("{}", surroundings);
        obstacles != 0
    }
    pub fn update_position(&mut self, world: &World) {
        let (half_sprite_size, screen_size, step) =
            (SPRITE_SIZE / 2.0, SCREEN_SIZE as f64, self.get_step());

        match self.direction {
            Direction::Left => if self.position.x > 0.0 {
                self.next_position.x -= step;
            },
            Direction::Right => {
                if self.position.x < screen_size - half_sprite_size {
                    self.next_position.x += step;
                }
            }
            Direction::Up => if self.position.y > 0.0 {
                self.next_position.y -= step;
            },
            Direction::Down => {
                if self.position.y < screen_size - half_sprite_size {
                    self.next_position.y += step;
                }
            }
            Direction::Neutral => {}
        }

        if self.will_collide_world(world) {
            self.next_position = self.position;
        } else {
            // Update the user position if not colliding.
            self.position = self.next_position;
        }
    }
}
