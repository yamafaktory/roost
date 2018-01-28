use collision::Collision;
use constants::{PIXEL_TOLERANCE, SCREEN_SIZE, SPRITE_SIZE, STEP};
use direction::Direction;
use either::Either;
use piston_window::*;
use piston_window::Key;
use sprite::create_sprite;
use types::{AnimatedSprite, Entities, Vec2, World};

pub struct Player {
    pub direction: Direction,
    pub collision: Collision,
    pub next_position: Vec2,
    pub position: Vec2,
    pub scale: Vec2,
    pub sprite_index: usize,
    pub sprites: AnimatedSprite,
}

impl Player {
    pub fn new(
        direction: Direction,
        position: Vec2,
        scale: Vec2,
        // We need the window context.
        window: &mut PistonWindow,
    ) -> Self {
        let mut sprites: AnimatedSprite = Vec::new();
        let mut inject_sprites = |direction: &String| {
            let mut inner_sprites: AnimatedSprite = Vec::new();
            for index in 0..3 {
                let index_string = index.to_string();
                inner_sprites.push(create_sprite(
                    window,
                    direction.to_string() + &"-".to_string() + &index_string[..]
                        + ".png",
                ));
            }
            inner_sprites
        };
        // Left sprites.
        sprites.append(&mut inject_sprites(&"left".to_string()));
        // Right sprites.
        sprites.append(&mut inject_sprites(&"right".to_string()));
        // Up sprites.
        sprites.append(&mut inject_sprites(&"up".to_string()));
        // Down sprites.
        sprites.append(&mut inject_sprites(&"down".to_string()));
        Self {
            collision: Collision::None,
            direction,
            next_position: position,
            position,
            scale,
            // Start width first going down sprite.
            sprite_index: 9,
            sprites: sprites,
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
    fn position_to_matrix(
        &self,
        multi: bool,
    ) -> Either<(usize, usize), Vec<(usize, usize)>> {
        let get_pos_tuple = |(x_offset, y_offset): (f64, f64)| {
            let (x, y) = (
                ((self.next_position.x + x_offset) / SPRITE_SIZE).ceil(),
                ((self.next_position.y + y_offset) / SPRITE_SIZE).ceil(),
            );
            return (
                (if y >= 1.0 { y - 1.0 } else { 0.0 }) as usize,
                (if x >= 1.0 { x - 1.0 } else { 0.0 }) as usize,
            );
        };
        if multi {
            let adjusted_sprite_size = SPRITE_SIZE - PIXEL_TOLERANCE;
            // Get all the corner positions of the sprite.
            let mut positions = Vec::new();
            // Top left corner.
            positions.push(get_pos_tuple((0.0, 0.0)));
            // Top right corner.
            positions.push(get_pos_tuple((adjusted_sprite_size, 0.0)));
            // Bottom left corner.
            positions.push(get_pos_tuple((0.0, adjusted_sprite_size)));
            // Bottom right corner.
            positions.push(get_pos_tuple((
                adjusted_sprite_size,
                adjusted_sprite_size,
            )));
            return Either::Right(positions);
        } else {
            // Use the center of the sprite.
            let half_sprite_size = SPRITE_SIZE / 2.0;
            return Either::Left(get_pos_tuple((
                half_sprite_size,
                half_sprite_size,
            )));
        }
    }
    fn collide_world(&self, world: &World, entities: &Entities) -> bool {
        let mut collisions = 0;
        if let Either::Right(positions) = self.position_to_matrix(true) {
            positions
                .iter()
                .map(|&pos| {
                    let (row, column) = pos;
                    let column = world.column(column);
                    let row = column.row(row);
                    let mut iter = row.iter().enumerate();
                    if let Some((_, sprite_number)) = iter.next() {
                        let entity = &entities[*sprite_number as usize];
                        if *sprite_number != 0 && !entity.is_traversable() {
                            collisions = collisions + 1;
                        }
                    }
                })
                .collect::<Vec<_>>();
        }
        collisions > 0
    }
    fn check_surroundings(&self, world: &World) -> bool {
        return if let Either::Left((row, column)) =
            self.position_to_matrix(false)
        {
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

                _ => world.slice((row - 1, column - 1), (3, 3)),
            };
            let mut obstacles: u8 = 0;
            for row in 0..surroundings.column(0).iter().count() {
                for (_, value) in surroundings.row(row).iter().enumerate() {
                    obstacles += value;
                }
            }
            obstacles != 0
        } else {
            false
        };
    }
    pub fn update_position(&mut self, world: &World, entities: &Entities) {
        let (scale_x, scale_y, screen_size, step) =
            (1.0, 1.0, SCREEN_SIZE as f64, self.get_step());

        match self.direction {
            Direction::Left => if self.position.x > 0.0 {
                self.next_position.x -= step;
                self.sprite_index = if self.sprite_index < 2 {
                    self.sprite_index + 1
                } else {
                    0
                };
            },
            Direction::Right => {
                if self.position.x < screen_size - (SPRITE_SIZE / scale_x) {
                    self.next_position.x += step;
                }
                self.sprite_index =
                    if self.sprite_index > 2 && self.sprite_index < 5 {
                        self.sprite_index + 1
                    } else {
                        3
                    }
            }
            Direction::Up => if self.position.y > 0.0 {
                self.next_position.y -= step;
                self.sprite_index =
                    if self.sprite_index > 5 && self.sprite_index < 8 {
                        self.sprite_index + 1
                    } else {
                        6
                    };
            },
            Direction::Down => {
                if self.position.y < screen_size - (SPRITE_SIZE / scale_y) {
                    self.next_position.y += step;
                }
                self.sprite_index =
                    if self.sprite_index > 8 && self.sprite_index < 11 {
                        self.sprite_index + 1
                    } else {
                        9
                    };
            }
            Direction::Neutral => {}
        }

        if self.collide_world(world, entities) {
            self.next_position = self.position;
            self.collision = Collision::Some;
        } else {
            // Update the user position if not colliding.
            self.position = self.next_position;
            // Check for an incoming collision.
            if self.check_surroundings(world) {
                self.collision = Collision::Incoming;
            } else {
                self.collision = Collision::None;
            }
        }
    }
}
