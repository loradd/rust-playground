use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please insert your guess...");
    // mutable string variable
    let mut guess = String::new();
    // read input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line...");
    // print guess
    println!("You guessed: {}", guess);
}
