use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

#[derive(Debug)]
pub struct Block {
  pub x: f64,
  pub y: f64
}


impl Block {
  pub fn new (x: f64, y: f64) -> Block {
    Block {
      x,
      y
    }
  }

  pub fn paint (&self, color: Color, con: &Context, g: &mut G2d) {
    rectangle(
      color,
      [self.x * BLOCK_SIZE, self.y * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
      con.transform,
      g
    );
  }
}
