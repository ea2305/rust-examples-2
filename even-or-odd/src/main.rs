/**
 * Ask the user for a number. Depending on whether the number is even or odd, print out an appropriate message
 * url: https://www.practicepython.org/exercise/2014/02/05/02-odd-or-even.html
 */

use std::io;

fn main() {
    // generate new variable reference
    let mut number = String::new();

    // read a number from user
    println!("Write a number: ");
    io::stdin()
        .read_line(&mut number) // reference string mutation pointer
        .expect("Error on read");

    // shadow last reference with new parsed value
    let number: usize = number.trim().parse().expect("error on parse");

    // if number is even
    if number % 2 == 0 {
        println!("Is even");
    } else {
        println!("Is odd");
    }
}
