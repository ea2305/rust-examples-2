/**
 * Write a program (using functions!) that asks the user for a long string containing multiple words. 
 * Print back to the user the same string, except with the words in backwards order. For example, say I type the string:
 */

use std::io;

fn main() {
    let mut sentence = String::new();
    println!("Write a sentence to continue: ");

    io::stdin()
        .read_line(&mut sentence)
        .expect("Error on read");

    let word: String = sentence.trim().split(' ').rev().collect::<Vec<&str>>().join(" ");
    println!("{:?}", word);
}
