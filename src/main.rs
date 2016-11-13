extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let lower_bound: u32 = 1;
    let upper_bound: u32 = 100;
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(lower_bound, upper_bound + 1);
    println!("The secret number is between {} and {}", lower_bound, upper_bound);

    let mut no_guesses: u32 = 0;
    loop {
        no_guesses += 1;
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);

        //Parse guess string to unsigned 32 bit integer.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won! You figured out the number in {} guesses", no_guesses);
                break;
            }
        }
    }

}
