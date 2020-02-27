// divisor example
use std::io;

fn main() {
    let mut number = String::new();

    // read line from keyboard
    println!("Give me a nubmer:");
    io::stdin()
        .read_line(&mut number)
        .expect("error on read");

    let number: usize = number.trim().parse().expect("error on parse");

    for i in 2..=number {
        if number % i == 0 {
            println!("can be divided by: {}", i)
        }
    }
}
