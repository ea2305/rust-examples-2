use piston_window::{ rectangle, Context, G2d };
use piston_window::types::Color;


struct Snake {
  body: Vec<Block>
}

impl Snake () {
  pub fn new (&mut self) {
    self.body.push(Block { x: 1, y: 1, size: 25.0 })
  }
}