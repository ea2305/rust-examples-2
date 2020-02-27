/**
 * Make a two-player Rock-Paper-Scissors game. (Hint: Ask for player plays (using input), compare them, print out a message of congratulations to the winner, and ask if the players want to start a new game)
 * URL: https://www.practicepython.org/exercise/2014/03/26/08-rock-paper-scissors.html
 */
use std::io;
use rand::Rng;

fn main() {
    let mut rnd = rand::thread_rng();
    let mut start_again = String::new();
    let mut user_option = String::new();
    let options: [String; 3] = [
        String::from("ROCK"),
        String::from("PAPER"),
        String::from("SCISSORS")
    ];

    while start_again.trim().to_uppercase() != "NO" {
        // inital game logic
        println!("Let's play: Select one option: <Rock, Paper or Scissors>");
        println!("---------------------------------------------------------");
        user_option.clear();
        // get user response
        io::stdin()
            .read_line(&mut user_option)
            .expect("error al leer información de juego");

        // generate new random option
        let index = rnd.gen_range(0,options.len());

        let user_str_option = user_option.trim().to_uppercase();

        if user_str_option == "ROCK" || user_str_option == "PAPER" || user_str_option == "SCISSORS" {
            println!("user[{}] vs machine[{}]", user_str_option, options[index]);

            if user_str_option == options[index] {
                println!("]>  Tie!")
            } else if user_str_option == "ROCK" && options[index] == "SCISSORS" {
                println!("]>  You win!")
            } else if user_str_option == "ROCK" && options[index] == "PAPER" {
                println!("]> You lose!")
            } else if user_str_option == "SCISSORS" && options[index] == "PAPER" {
                println!("]>  You win!")
            } else if user_str_option == "SCISSORS" && options[index] == "ROCK" {
                println!("]> You lose!")
            } if user_str_option == "PAPER" && options[index] == "ROCK" {
                println!("]>  You win!")
            } else if user_str_option == "PAPER" && options[index] == "SCISSORS" {
                println!("]> You lose!")
            }

            println!("---------------------------------------------------------");
            // restart?
            start_again.clear();
            println!("Play again? <Type YES or NO>");
            io::stdin()
                .read_line(&mut start_again)
                .expect("error al leer información");
        } else {
            println!("Invalid option, please try again.")
        }
    }

    println!(":* the end");
}
