use constants::SPRITE_NUMBER;
use nalgebra::DMatrix;
use types::World;

pub struct Stage {
    pub world: World,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            #[rustfmt_skip]
            // Generate the world map as a matrix.
            world: DMatrix::from_row_slice(SPRITE_NUMBER, SPRITE_NUMBER, &[
                0, 0, 2, 3, 0, 0, 4, 0, 0, 1,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 3, 0, 0, 0, 2, 0, 0,
                0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            ]),
        }
    }
}
