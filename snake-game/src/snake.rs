use piston_window::{ Context, G2d };
use piston_window::types::Color;

use crate::block::Block;
use piston_window::*;

const SNAKE_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
const UPDATE_RATE: f64 = 0.3;

#[derive(Debug)]
pub struct Snake {
  pub alive: bool,
  pub body: Vec<Block>,
  pub last_move: Key,
  pub waiting_time: f64,
  playing: bool
}

impl Snake {
  pub fn new () -> Snake {
    Snake {
      body: vec![],
      alive: true,
      playing: true,
      last_move: Key::Right,
      waiting_time: 0.0
    }
  }

  pub fn keydown (&mut self, pressed_key: Key) {
    match pressed_key {
      Key::Space => {
        self.craft();
      },
      Key::Up => {
        if self.last_move != Key::Down {
          self.last_move = pressed_key;
        }
      },
      Key::Down => {
        if self.last_move != Key::Up {
          self.last_move = pressed_key;
        }
      },
      Key::Left => {
        if self.last_move != Key::Right {
          self.last_move = pressed_key;
        }
      },
      Key::Right => {
        if self.last_move != Key::Left {
          self.last_move = pressed_key;
        }
      },
      _ => { self.last_move = pressed_key; }
    }
  }

  pub fn craft (&mut self) {
    self.playing = true;
    self.body = vec![];
    // test length
    self.body.push(Block { x: 7.0, y: 1.0 });
    self.body.push(Block { x: 6.0, y: 1.0 });
    self.body.push(Block { x: 5.0, y: 1.0 });
    self.body.push(Block { x: 4.0, y: 1.0 });
    // test length
    self.body.push(Block { x: 3.0, y: 1.0 });
    self.body.push(Block { x: 2.0, y: 1.0 });
    self.body.push(Block { x: 1.0, y: 1.0 });
    self.last_move = Key::Right;
  }

  pub fn update (&mut self, dt: f64, targets: Vec<Block>) -> bool {
    self.waiting_time += dt;

    let mut header = Block::new();
    let mut collition = false;

    if self.waiting_time > UPDATE_RATE {
      self.waiting_time = 0.0;

      if self.playing {
        let mut last_block = Block::new();

        for (index, piece) in self.body.iter_mut().enumerate() {
          if index == 0 {
            last_block = piece.clone();
            header = piece.clone();

            match self.last_move {
              Key::Up   =>  piece.update(piece.x, piece.y - 1.0),
              Key::Down =>  piece.update(piece.x, piece.y + 1.0),
              Key::Left =>  piece.update(piece.x - 1.0, piece.y),
              Key::Right => piece.update(piece.x + 1.0, piece.y),
              _ => piece.update(piece.x + 1.0, piece.y)
            }

            //collition validation 
            for barriers in &targets {
              if barriers.x == piece.x && barriers.y == piece.y {
                self.playing = false;
                return false;
              }
            }
          } else {
            if header.x == piece.x && header.y == piece.y {
              collition = true;
            }
            let curr = piece.clone();
            piece.update(last_block.x, last_block.y);
            last_block = curr;            
          }
        }        
      }
    }

    if collition {
      self.playing = false;
    }
    return self.playing && !collition;
  }

  pub fn eat (&mut self, food: Block) -> bool {
    let header = self.body.get(0).unwrap();
    // food verification
    if food.x == header.x && food.y == header.y {
      self.body.push(Block { x: 0.0, y: 0.0 });
      return true;
    }
    false
  }

  pub fn paint (&mut self, con: &Context, g: &mut G2d) {
    if self.playing {
      for piece in &mut self.body {
        piece.paint(SNAKE_COLOR, &con, g);
      }
    }
  }
}