use clap::Parser;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use std::io::{self, Write};

#[derive(Parser, Debug, PartialEq)]
struct Args {
    #[arg(short, long)]
    seed: Option<u64>,

    #[arg(short, long, default_value_t = 4, value_parser = clap::value_parser!(u8).range(1..=10))]
    length: u8,

    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u8).range(1..=30))]
    attempts: u8,
}

fn generate_random_code(seed: u64, length: u8) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(seed);
    let random_code: Vec<u8> = (0..length).map(|_| rng.random_range(0..=9)).collect();
    random_code
}

fn read_input(prompt: String) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn convert_to_code(text: &str) -> Option<Vec<u8>> {
    text.trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8))
        .collect()
}

fn main() {
    let args = Args::parse();
    let seed = args.seed.unwrap_or_else(|| rand::rng().next_u64());
    let length = args.length;
    let random_code = generate_random_code(seed, length);
    println!(
        "Seed: {}, Code length: {} - result: {:?}",
        seed, length, random_code
    );
    let mut i = 1u8;
    while i <= args.attempts {
        let result = read_input(format!("Guess the number (try {}/{}): ", i, args.attempts));
        match result {
            Ok(text) => match convert_to_code(&text) {
                Some(guess) if guess.len() == length as usize => {
                    if guess == random_code {
                        println!("Correct!");
                        return;
                    } else {
                        i += 1;
                    }
                }
                _ => {
                    println!("It's not the {} number code", length);
                }
            },
            Err(_) => {
                eprintln!("Error, good bye!");
                return;
            }
        }
    }
    println!("Good bye");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use rstest::rstest;

    #[rstest]
    #[case(3567657657632322323, 4, vec![7, 3, 7, 8])]
    #[case(343243242, 6, vec![6, 1, 5, 2, 4, 0])]
    fn test_random_number_generation(
        #[case] seed: u64,
        #[case] length: u8,
        #[case] expected: Vec<u8>,
    ) {
        assert_eq!(generate_random_code(seed, length), expected);
    }

    #[rstest]
    #[case(vec!["name"], None, 4, 10)]
    #[case(vec!["name", "-s", "233213", "-l", "6", "-a", "5"], Some(233213), 6, 5)]
    #[case(vec![
            "name",
            "--seed",
            "233213",
            "--length",
            "6",
            "--attempts",
            "5",
        ], Some(233213), 6, 5)]
    fn args_parsed_successfully(
        #[case] input: Vec<&str>,
        #[case] expected_seed: Option<u64>,
        #[case] expected_length: u8,
        #[case] expected_attempts: u8,
    ) {
        let args = Args::parse_from(input);
        assert_eq!(
            args,
            Args {
                seed: expected_seed,
                length: expected_length,
                attempts: expected_attempts,
            }
        );
    }

    #[rstest]
    #[case(vec!["name", "--seed", "-32321321"])]
    #[case(vec!["name", "--seed", "87897987779878979898988932321321"])]
    #[case(vec!["name", "-l", "0"])]
    #[case(vec!["name", "-l", "11"])]
    #[case(vec!["name", "-a", "0"])]
    #[case(vec!["name", "-a", "31"])]
    fn reject_invalid_args(#[case] input: Vec<&str>) {
        let res = Args::try_parse_from(input);
        assert!(res.is_err())
    }

    #[rstest]
    #[case("1234", Some(vec![1, 2, 3, 4]))]
    #[case("1", Some(vec![1]))]
    #[case("4567890", Some(vec![4, 5, 6, 7, 8, 9, 0]))]
    #[case("", Some(vec![]))]
    #[case("1234a", None)]
    #[case("abc", None)]
    #[case("1a2,3k?>!@", None)]
    fn test_code_conversion(#[case] input: &str, #[case] expected: Option<Vec<u8>>) {
        assert_eq!(convert_to_code(input), expected);
    }
}
