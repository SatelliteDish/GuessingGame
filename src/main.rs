extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
                   .expect("Invalid input, failed to read line!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        println!("Your Guess: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal   => {
                println!("Correct!");
                break;
            },
        }
    }
}