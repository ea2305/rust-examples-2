/**
 * Write a password generator in Python. Be creative with how you generate passwords - 
 * strong passwords have a mix of lowercase letters, uppercase letters, numbers, and symbols. The passwords should be random,
 * generating a new password every time the user asks for a new password. Include your run-time code in a main method.
 */
extern crate rand;

use std::io;
use rand::Rng;


fn main() {
    println!("Hello to password generator: how long the password shold be?\n");
    let number = ask_number();
    let password = generate_password(number);
    println!("Your password is: {}", password);
}

fn ask_number() -> usize {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("error on read");

    let number: usize = number
        .trim()
        .parse()
        .expect("Error to parse");
    number
}

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let list: Vec<&str> = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM,./[]{}-+=()_1234567890"
        .trim()
        .split("")
        .collect();

    let mut password_str = String::new();

    for i in 0..length {
        let index: usize = rng.gen_range(0, list.len());
        password_str.push_str(list[index]);
    }
    password_str
}
