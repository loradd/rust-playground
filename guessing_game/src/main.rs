use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = secret();
    loop {
        match guess() {
            Err(_) => return,
            Ok(guess) => if equals(guess, secret) {
                return
            }
        };
    }

}

fn secret() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn guess() -> Result<u32, std::num::ParseIntError>{
    println!("Insert a number from 0 to 100.");
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Fatal error while reading line");
    guess.trim().parse()
}

fn equals(guess: u32, secret_number: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("The secret number is higher.");
            false
        },
        Ordering::Greater => {
            println!("The secret number is lower.");
            false
        },
        Ordering::Equal => {
            println!("You Win!");
            true
        }
    }
}

