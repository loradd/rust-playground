use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = generate_secret_number();
    loop {
        let guessed_number: u32 = match read_user_guess() {
            Ok(guess) => guess,
            Err(_) => break
        };
        if equals(guessed_number, secret_number) {
            break;
        }
    }
}

fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn read_user_guess() -> Result<u32, std::num::ParseIntError> {
    println!("Insert a number from 0 to 100.");
    let mut user_guess: String = String::new();
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Fatal error while reading line");
    user_guess.trim().parse()
}

fn equals(guessed_number: u32, secret_number: u32) -> bool {
    match guessed_number.cmp(&secret_number) {
        Ordering::Less => {
            println!("The secret number is higher.");
            false
        }
        Ordering::Greater => {
            println!("The secret number is lower.");
            false
        }
        Ordering::Equal => {
            println!("You Win!");
            true
        }
    }
}
