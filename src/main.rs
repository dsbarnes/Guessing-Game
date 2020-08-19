use std::io;
// This is the 'lazy way'
// use rand::prelude::*;

// All generators implement the Rng trait, so we import it
use rand::Rng;

use std::cmp::Ordering;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 100+1);
    println!("Guess the number!");

    loop{
        println!("Enter your guess:");
        let mut guess = String::new();

        io::stdin()
            // read_line expects a mutable String
            .read_line(&mut guess)
            .expect("failed to read line");
     
        // .cmp() expects a number, so we shadow guess
        // parse returns a Result type
        let guess:u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("That was not a number");
                continue;
            }
        };

        println!("You guessed {}", guess);
        // Compare that guess to the secret_number:
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
