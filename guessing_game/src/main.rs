use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Welcome to guessing game!!");

    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("Secret number is {}.", secret_number);

    loop {
        println!("Please enter a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!!".red()),
            Ordering::Greater => println!("{}", "Too Big!!".red()),
            Ordering::Equal => {
                println!("{}", "You win!!".green());
                break;
            }
        }

        println!("You guessed: {}", guess);
    }
}
