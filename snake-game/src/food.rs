use piston_window::{ Context, G2d };
use piston_window::types::Color;

const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

use crate::block::Block;

pub struct Food {
  pub node: Block
}

impl Food {
  pub fn paint (&self, con: &Context, g: &mut G2d) {
    self.node.paint(FOOD_COLOR, &con, g);
  }

  pub fn collition (&self, body: Vec<Block>) -> bool {
    for piece in &body {
      if piece.x == self.node.x && piece.y == self.node.y {
        return true;
      }
    }
    return false
  }
}
