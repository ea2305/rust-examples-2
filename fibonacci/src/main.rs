
// Write a program that asks the user how many Fibonnaci numbers to generate and then generates them.
// Take this opportunity to think about how you can use functions. Make sure to ask the user to enter the number of numbers in the sequence to generate.(Hint: The Fibonnaci seqence
// is a sequence of numbers where the next number in the sequence is the sum of the previous two numbers in the sequence. The sequence looks like this: 1, 1, 2, 3, 5, 8, 13, …)
use std::io;

fn main() {
    let mut numbers = String::new();

    println!("how many fibo numbers do you want? ----------------------------------------");
    io::stdin()
        .read_line(&mut numbers)
        .expect("read error");

    let numbers: i32 = numbers
        .trim()
        .parse()
        .expect("parse error");

    println!("first version numbers ----------------------------------------");
    fibonacci_1(numbers);
    println!("optimized version O(n) numbers ----------------------------------------");
    fibonacci_2(numbers);
}

fn fibonacci_1 (n: i32) {
    let mut count: i32 = 0;
    let mut a = 1;
    let mut b = 1;

    loop {
        if count == n {
            break;
        }

        println!("{}", a);
        a += b;
        count += 1;

        if count == n {
            break;
        }

        println!("{}", b);
        b += a;
        count += 1;
    }
}

fn fibonacci_2 (n: i32) {
    let mut c = 1;
    let mut a = 0;
    let mut b = 1;

    for i in 0..n {
        println!("{}: {}", i + 1, c);
        c = a + b;
        a = b;
        b = c;
    }
}