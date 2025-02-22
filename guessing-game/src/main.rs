use rand::Rng;
use std::io;

fn main() {
    let answer = rand::rng().random_range(1..=100);
    println!("Guess the number in range 1..100!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guess: {guess}");

        match guess.trim().parse::<u16>() {
            Err(e) => println!("It's not a number! ({e})"),
            Ok(num) if !(1..=100).contains(&num) => println!("It's not a valid guess"),
            Ok(num) if num == answer => {
                println!("That's correct - the answer is {answer}");
                break;
            }
            Ok(num) if num > answer => println!("Answer is lower than {num}"),
            Ok(num) if num < answer => println!("Answer is higher than {num}"),
            _ => panic!("It should not happen"),
        }
    }
}
