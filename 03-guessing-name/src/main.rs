use rand::Rng; // import Rng (Random Number Generator) trait into scope
use std::{cmp::Ordering, io};

fn main() {
    // Use rand crates to generate a secret number
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        // println! is a macro. It's a code that will generate code at compile time
        println!("Please input your guess.");

        // ::new() -> attached function into the String type. It likes the static method in Java
        // :: -> path operator (module -> enum/function/method)
        // Rust infer the String type for Guess attribute
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // .method -> method attached to the instance
            .expect("failed to read the line");

        // We must cast guess to u32 to be able to compare it against secret_number
        // Instead of creating a new variable Rust will reuse (shadowing) it for new value
        // It's usefull when you want to cast a value to another type without creating a new variable
        //  -> Instead of let guess_u32 = guess_str.trim().parse().expect("...");
        //  -> we have let guess = guess.trim().parse().expect("...");
        // let guess: u32 = guess.trim().parse().expect("Please type a valid number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess {guess}");

        // Match use pattern matching to select the right branch
        // if cmp return the Less variant, it will print Too small
        // if cmp return the Equal variant, it will print You win
        // if cmp return the Greater variant, it will print Too big
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
