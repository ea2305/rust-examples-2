/**
 * Ask the user for a string and print out whether this string is a palindrome or not. (A palindrome is a string that reads the same forwards and backwards.)
 * URL: https://www.practicepython.org/exercise/2014/03/12/06-string-lists.html
 */
use std::io;

fn main() {
    // init string
    let mut word = String::new();

    // start to read line
    io::stdin()
        .read_line(&mut word)
        .expect("Error on read");

    word.pop(); // remove end of line

    // split into vec of chars
    let items: Vec<char> = word.chars().rev().collect();
    // reverse and rejoin into a string
    let drow: String = items.iter().collect();

    // evaluate the original and the new one
    if drow == word {
        println!("Is a palindrome :) !");
    } else {
        println!("Is not a palindrome :(");
    }
}
