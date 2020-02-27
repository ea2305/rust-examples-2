use std::io;
use std::cmp::Ordering;
use rand::Rng;
/**
 * Generate a random number between 1 and 9 (including 1 and 9). Ask the user to guess the number, then tell them whether they guessed too low, too high, or exactly right.
 * URL: https://www.practicepython.org/exercise/2014/04/02/09-guessing-game-one.html
 */

fn main() {
    let mut rnd = rand::thread_rng();
    let number = rnd.gen_range(1, 11);

    loop {
        println!("Write a number between [1,9] to guess");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("error input");

        let input: usize = input
            .trim()
            .parse()
            .expect("error to parse");

        match input.cmp(&number) {
            Ordering::Less => println!("to small!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
            Ordering::Greater => println!("to big!"),
        }
    }

    println!("Finish the number was: {}", number);
}
