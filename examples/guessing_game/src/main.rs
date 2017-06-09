extern crate rand;

use std::io;
use std::cmp::Ordering;
// Rng is a trait that defines methods that rng's implement.
// It must be in scope to use those methods!
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            // Use the io::Result returned
            // Result is an enum of Ok or Err; see Ch6
            .expect("Failed to read line.");

        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            // Unwrap Result()
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
