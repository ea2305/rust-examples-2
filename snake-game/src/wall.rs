use piston_window::{ Context, G2d };
use piston_window::types::Color;

use crate::block::Block;

const WALL_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

#[derive(Debug, Clone)]
pub struct Wall {
  pub size: f64,
  pub nodes: Vec<Block>
}

impl Wall {
  pub fn craft (&mut self) {
    for i in 0..(self.size as u32) {
      let index: f64 = i as f64;
      // generate x top
      let x_top = Block { x: index, y: 0.0 };
      // generate x bottom
      let x_bottom = Block { x: index, y: self.size - 1.0 };
      // generate y left 
      let y_left = Block { x: 0.0, y: index };
      // generate y right     
      let y_right = Block { x: self.size - 1.0, y: index };

      self.nodes.push(x_top);
      self.nodes.push(x_bottom);
      self.nodes.push(y_left);
      self.nodes.push(y_right);
    }
  }

  pub fn paint (&self, con: &Context, g: &mut G2d) {
    for node in &self.nodes {
      node.paint(WALL_COLOR, &con, g);
    }
  }
}