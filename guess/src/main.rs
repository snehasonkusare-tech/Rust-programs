use std::io;
use rand::Rng;

fn main() {
    println!("🎯 Welcome to Guess the Number Game!");
    println!("I'm thinking of a number between 1 and 100...");

    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let num1: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let result = match num1.cmp(&random) {
            std::cmp::Ordering::Less => "Too Small!",
            std::cmp::Ordering::Greater => "Too Big!",
            std::cmp::Ordering::Equal => {
                println!("🎉 You Win! The correct number was {}.", random);
                break;
            }
        };

        println!("{}", result);
    }

    println!("Game Over!");
}

