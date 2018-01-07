use constants::{SCREEN_SIZE, SPRITE_SIZE, STEP};
use direction::Direction;
use piston_window::Key;
use types::{Tex, Vec2, World};

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

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
    fn pos_to_matrix(
        &self,
        multi: bool,
    ) -> Either<(usize, usize), Vec<(usize, usize)>> {
        let get_pos_tuple = |(x_offset, y_offset): (f64, f64)|{
            (
                (((self.next_position.y + y_offset) / SPRITE_SIZE)
                    .ceil() - 1.0) as usize,
                (((self.next_position.x + x_offset) / SPRITE_SIZE)
                    .ceil() - 1.0) as usize,
            )
        };
        if multi {
            // Get all the corner positions of the sprite.
            let mut positions = Vec::new();
            // Top left corner.
            positions.push(get_pos_tuple((0.0, 0.0)));
            // Top right corner.
            positions.push(get_pos_tuple((SPRITE_SIZE, 0.0)));
            // Bottom left corner.
            positions.push(get_pos_tuple((0.0, SPRITE_SIZE)));
             // Bottom right corner.
            positions.push(get_pos_tuple((SPRITE_SIZE, SPRITE_SIZE)));
            return Either::Right(positions);
        } else {
            let half_sprite_size = SPRITE_SIZE / 2.0;
            return Either::Left((
                (((self.next_position.y + half_sprite_size) / SPRITE_SIZE)
                    .ceil() - 1.0) as usize,
                (((self.next_position.x + half_sprite_size) / SPRITE_SIZE)
                    .ceil() - 1.0) as usize,
            ));
        }
    }
    fn collide_world(&self, world: &World) -> bool {
        let mut collisions = 0;
        if let Either::Right(positions) = self.pos_to_matrix(true) {
            positions
                .iter()
                .map(|&pos| {
                    let (row, column) = pos;
                    let column = world.column(column);
                    let row = column.row(row);
                    let mut iter = row.iter().enumerate();
                    if let Some((_, sprite_number)) = iter.next() {
                        if *sprite_number != 0 {
                            collisions = collisions + 1;
                        }
                    }
                })
                .collect::<Vec<_>>();
        }
        collisions > 0
        // let (row, column) = self.pos_to_matrix(true);
        // let column = world.column(column);
        // let row = column.row(row);
        // let mut iter = row.iter().enumerate();
        // return if let Some((_, sprite_number)) = iter.next() {
        //     *sprite_number != 0
        // } else {
        //     false
        // };
    }
    fn will_collide_world(&self, world: &World) -> bool {
        // let (row, column) = self.pos_to_matrix(false);
        // println!(
        //     "{} & {}",
        //     self.next_position.y % SPRITE_SIZE,
        //     self.next_position.x % SPRITE_SIZE
        // );
        // // Extract a slice of the matrix to get the surroundings.
        // let surroundings = match (row, column) {
        //     (0, 1...8) => world.slice((0, column - 1), (2, 3)),
        //     (0, 9) => world.slice((0, column - 1), (2, 2)),
        //     (1...8, 9) => world.slice((row - 1, column - 1), (3, 2)),

        //     (1...8, 0) => world.slice((row - 1, 0), (3, 2)),
        //     (9, 0) => world.slice((row - 1, 0), (2, 2)),
        //     (9, 1...8) => world.slice((row - 1, column - 1), (2, 3)),

        //     (0, 0) => world.slice((0, 0), (2, 2)),
        //     (9, 9) => world.slice((row - 1, column - 1), (2, 2)),

        //     _ => world.slice((row - 1, column - 1), (3, 3)),
        // };
        // let mut obstacles: u8 = 0;
        // for row in 0..surroundings.column(0).iter().count() {
        //     for (_, value) in surroundings.row(row).iter().enumerate() {
        //         obstacles += value;
        //     }
        // }
        // println!("{}", surroundings);
        // obstacles != 0
        false
    }
    pub fn update_position(&mut self, world: &World) {
        let (scale_x, scale_y, screen_size, step) =
            (1.0, 1.0, SCREEN_SIZE as f64, self.get_step());

        match self.direction {
            Direction::Left => if self.position.x > 0.0 {
                self.next_position.x -= step;
            },
            Direction::Right => {
                if self.position.x < screen_size - (SPRITE_SIZE / scale_x) {
                    self.next_position.x += step;
                }
            }
            Direction::Up => if self.position.y > 0.0 {
                self.next_position.y -= step;
            },
            Direction::Down => {
                if self.position.y < screen_size - (SPRITE_SIZE / scale_y) {
                    self.next_position.y += step;
                }
            }
            Direction::Neutral => {}
        }

        if self.collide_world(world) {
            self.next_position = self.position;
            // self.position = self.next_position;
            println!("💥");
        } else {
            // Update the user position if not colliding.
            self.position = self.next_position;
        }
    }
}
