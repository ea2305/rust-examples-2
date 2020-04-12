extern crate rand;
extern crate piston_window;
use piston_window::{ rectangle };
use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};

mod block;
mod wall;
mod snake;
mod food;

const GAME_OVER: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let title = String::from("Snake x");
    
    let mut window: PistonWindow =
        WindowSettings::new(title, [25 * 15; 2])
            .build()
            .unwrap();

    // create wall content
    let mut wall = wall::Wall { size: 15.0, nodes: vec![]};
    wall.craft();

    // create snake
    let mut snake = snake::Snake::new();
    snake.craft();
    let mut food = generate_food();
    let mut playing = false;

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            snake.keydown(key);
        }

        window.draw_2d(&event, |con, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            if playing {
                snake.paint(&con, g);
                wall.paint(&con, g);
                if food.collition(snake.body.clone()) {
                    food = generate_food();
                } else {
                    food.paint(&con, g);
                }
            } else {
                rectangle(
                    GAME_OVER,
                    [0.0, 0.0, 25.0 * 15.0, 25.0 * 15.0],
                    con.transform,
                    g
                );
            }
        });

        event.update(|arg| {
            playing = snake.update(
                arg.dt,
                wall.nodes.clone()
            );

            if snake.eat(food.node.clone()) {
                food = generate_food();
            }
        });
    }
}

fn generate_food () -> food::Food {
    let mut rng = thread_rng();
    let food_x: f64 = rng.gen_range(1, 14) as f64;
    let food_y: f64 = rng.gen_range(1, 14) as f64;
    println!("{:?} {:?}", food_x, food_y);
    food::Food { node: block::Block { x: food_x, y: food_y }}
}