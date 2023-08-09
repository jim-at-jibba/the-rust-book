use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // cargo doc --open shows documentation of all crates installed
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess:");

    let mut guess = String::new();

    // read_line returns Result type
    // expect will return the values passed to it if its an Err
    // or the value returned from read_line if its OK
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Rust allows shadowing.
    // Often used when converting to different types
    let guess: u32 = guess
        .trim()
        // rust knows what to parse the value to by referencing the type
        // above
        .parse()
        .expect("Please type a number!");

    // println!("You guessed: {guess}");
    // The following way you can manipulate the value
    println!("you guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win"),
    }
}
