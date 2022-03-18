use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess the number:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please type a number!".yellow());
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Your guess is smaller than the true value.".red()),
            Ordering::Greater => println!("{}", "Your guess is greater than the true value.".red()),
            Ordering::Equal => {
                println!("{}", "Your guess is right!".green());
                break;
            }
        };
    }
}
