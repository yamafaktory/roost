extern crate gfx_device_gl;

use entity::Entity;
use nalgebra::{Dynamic, Matrix, MatrixVec, Vector2};
use piston_window::Texture;

pub type AnimatedSprite = Sprites;
pub type Entities = Vec<Entity>;
pub type Sprite = Result<Tex, String>;
pub type Sprites = Vec<Sprite>;
pub type Tex = Texture<gfx_device_gl::Resources>;
pub type Vec2 = Vector2<f64>;
pub type World = Matrix<u8, Dynamic, Dynamic, MatrixVec<u8, Dynamic, Dynamic>>;
