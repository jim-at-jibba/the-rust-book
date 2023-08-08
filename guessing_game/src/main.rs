use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    let mut guess = String::new();

    // read_line returns Result type
    // expect will return the values passed to it if its an Err
    // or the value returned from read_line if its OK
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // println!("You guessed: {guess}");
    // The following way you can manipulate the value
    println!("you guessed {}", guess)
}
