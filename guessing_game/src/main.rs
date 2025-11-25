use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let between_start = secret_number - 5;
    let between_end = secret_number + 5;

    println!("The secret number is {secret_number} somewhere between: {} and {}", between_start, between_end);

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}

/*
guess = guess.trim().to_string();

if guess == secret_number.to_string() {
    println!("You guessed: {guess}");
} else {
    println!("You HAVE NOT guessed it, it was: {secret_number}, but you provided: {guess}.");
}
*/