extern crate rand;
extern crate piston_window;

use piston_window::*;

mod block;
mod wall;

fn main() {
    let title = String::from("Snake x");
    
    let mut window: PistonWindow =
        WindowSettings::new(title, [25 * 15; 2])
            .build()
            .unwrap();

    // create wall content
    let mut wall = wall::Wall { size: 15.0, nodes: vec![]};
    wall.craft();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |con, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            wall.paint(&con, g);
        });
    }
}
