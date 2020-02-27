/**
 * Ask the user for a number and determine whether the number is prime or not. (For those who have forgotten, a prime number is a number that has no divisors.).
 * You can (and should!) use your answer to Exercise 4 to help you. Take this opportunity to practice using functions, described below.
 * URL: https://www.practicepython.org/exercise/2014/04/16/11-check-primality-functions.html
 */

use std::io;

fn main() {
    let mut input = String::new();
    // get input from user
    io::stdin()
        .read_line(&mut input)
        .expect("error on read");

    // cast number
    let input: isize = input
        .trim()
        .parse()
        .expect("error on parse");

    let is_prime = is_prime(input);

    if is_prime {
        println!("{} is prime!", input);
    } else {
        println!("{} is not prime!", input);
    }
}

fn is_prime (number: isize) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}
