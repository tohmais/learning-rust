use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // .expect is handling in the case of an error.
        // A function will return a Result enum that has an 'Ok' or 'Err'
        // value based on whether the function was successful.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
    }
}