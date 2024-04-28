use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tryes: i32 = 0;
    println!("Guess Game");
    loop {
        print!("Input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a valid number.");
                continue;
            }
        };

        tryes += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is bigger."),
            Ordering::Greater => println!("The number is smaller."),
            Ordering::Equal => {
                println!("\nYou win! 🥳\nNumber of tryes: {tryes}");
                break;
            }
        }
    }
}
