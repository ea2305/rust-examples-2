use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

#[derive(Debug, Clone)]
pub struct Block {
  pub x: f64,
  pub y: f64
}

impl Block {
  pub fn new () -> Self {
    Block {
      x: 0.0,
      y: 0.0
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

  pub fn update (&mut self, x: f64, y: f64) {
    self.x = x;
    self.y = y;
  }
}
