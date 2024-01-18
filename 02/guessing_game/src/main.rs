/**
 * File:        main.rs
 * Description: Number guessing game in rust
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Flushing error");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // Please! God damn it! I hate this hacker crap!
                println!("Ah-ah-ah! You didn't say the magic word!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner, winner chicken dinner!");
                break;
            }
        }
    }
}
