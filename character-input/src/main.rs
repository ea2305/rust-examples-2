/**
 * Read name from user and print a custom message
 * origin source: https://www.practicepython.org/exercise/2014/01/29/01-character-input.html
 */
// standar library to read input from user
use std::io;

fn main() {
    // we declarete a new string with mutable reference
    // called name
    let mut name = String::new();
    let mut age = String::new();

    println!("Please provide a name: ");
    // init reader from stdin
    io::stdin()
        .read_line(&mut name)       // read line from cli prompt
        .expect("error on read");   // on error return string error

    // privde a new message to users figurate out what program needs
    println!("yeah! now prived your age: ");
    // read age as a string
    io::stdin()
        .read_line(&mut age)
        .expect("error on read age.");

    // parse value as a usize
    let age: usize = age.trim().parse().expect("error on parse age");

    // print a message with name interpolation.
    println!("Hello, {} and you have {} years old", name, age);
}
