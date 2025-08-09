mod args;
mod game;

use crate::args::Args;
use clap::Parser;
use colored::*;
use rand::RngCore;
use std::io::{self, Write};
use std::time::Instant;

fn read_input(prompt: String) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let args = Args::parse();
    let seed = args.seed.unwrap_or_else(|| rand::rng().next_u64());
    let length = args.length;
    let options = args.options;
    let unique = args.unique;
    let random_code = game::generate_random_code(seed, length, options, unique);
    if !game::verify_guess(&random_code, length, options, unique) {
        eprintln!("Cannot generate code matching rules");
        std::process::exit(3);
    }
    println!(
        "Seed: {}, Code length: {}, digits: 0-{}",
        seed,
        length,
        options - 1
    );
    let mut i = 1u8;
    let start = Instant::now();
    while i <= args.attempts {
        let result = read_input(format!("Guess the number (try {}/{}): ", i, args.attempts));
        match result {
            Ok(text) => match game::convert_to_code(&text) {
                Some(guess) if game::verify_guess(&guess, length, options, unique) => {
                    if guess == random_code {
                        println!(
                            "{} Solved in {} seconds",
                            "Correct!".green(),
                            start.elapsed().as_secs()
                        );
                        return;
                    } else {
                        let clue = game::calculate_clue(&random_code, &guess);
                        println!(
                            "Correct: {}, miss placed: {}",
                            clue.correct.to_string().green(),
                            clue.miss_placed.to_string().yellow()
                        );
                        i += 1;
                    }
                }
                _ => {
                    println!("{}", "It's not the valid code".red());
                }
            },
            Err(_) => {
                eprintln!("{}", "Error, good bye!".red());
                std::process::exit(1);
            }
        }
    }
    println!("Answer was: {}", game::vec_u8_to_string(random_code).blue());
}
