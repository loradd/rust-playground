use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Generating random secret number...");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please insert your guess...");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");
        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_error) => {
                println!("Bye...");
                return;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Nope...too small!"),
            Ordering::Greater => println!("Nope...too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
