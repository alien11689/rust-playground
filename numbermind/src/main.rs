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

    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..=10))]
    options: u8,
}

fn generate_random_code(seed: u64, length: u8, options: u8) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(seed);
    let random_code: Vec<u8> = (0..length).map(|_| rng.random_range(0..options)).collect();
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

#[derive(Debug, PartialEq)]
struct Clue {
    correct: u8,
    miss_placed: u8,
}

impl Clue {
    fn new(correct: u8, miss_placed: u8) -> Clue {
        Clue {
            correct,
            miss_placed,
        }
    }
}

fn calculate_clue(solution: &[u8], guess: &[u8]) -> Clue {
    let mut correct = 0;
    let mut miss_placed = 0;
    let mut solution_parts = vec![];
    let mut guess_parts = vec![];
    for i in 0..solution.len() {
        if solution[i] == guess[i] {
            correct += 1;
        } else {
            solution_parts.push(solution[i]);
            guess_parts.push(guess[i]);
        }
    }
    solution_parts.sort();
    guess_parts.sort();
    let mut solution_i = 0;
    let mut guess_i = 0;
    while solution_i < solution_parts.len() && guess_i < guess_parts.len() {
        let cur_s = solution_parts[solution_i];
        let cur_g = guess_parts[guess_i];
        if cur_s == cur_g {
            miss_placed += 1;
            solution_i += 1;
            guess_i += 1;
        } else if cur_s < cur_g {
            solution_i += 1;
        } else {
            guess_i += 1;
        }
    }
    Clue::new(correct, miss_placed)
}

fn vec_u8_to_string(digits: Vec<u8>) -> String {
    digits.iter().map(|d| d.to_string()).collect()
}

// todo limit repetitions
// todo add colors
// todo calculate time spent
// todo refactor
fn main() {
    let args = Args::parse();
    let seed = args.seed.unwrap_or_else(|| rand::rng().next_u64());
    let length = args.length;
    let options = args.options;
    let random_code = generate_random_code(seed, length, options);
    println!(
        "Seed: {}, Code length: {}, digits: 0-{}",
        seed,
        length,
        options - 1
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
                        let clue = calculate_clue(&random_code, &guess);
                        println!(
                            "Correct: {}, miss placed: {}",
                            clue.correct, clue.miss_placed
                        );
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
    println!("Answer was: {}", vec_u8_to_string(random_code));
    println!("Good bye");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use rstest::rstest;

    #[rstest]
    #[case(3567657657632322323, 4, vec![7, 3, 7, 8], 10)]
    #[case(343243242, 6, vec![6, 1, 5, 2, 4, 0], 10)]
    #[case(3567657657632322323, 4, vec![3, 1, 3, 4], 5)]
    #[case(3567657657632322323, 6, vec![2, 1, 3, 3, 0, 1], 4)]
    fn test_random_number_generation(
        #[case] seed: u64,
        #[case] length: u8,
        #[case] expected: Vec<u8>,
        #[case] options: u8,
    ) {
        assert_eq!(generate_random_code(seed, length, options), expected);
    }

    #[rstest]
    #[case(vec!["name"], None, 4, 10, 6)]
    #[case(vec!["name", "-s", "233213", "-l", "6", "-a", "5", "-o", "7"], Some(233213), 6, 5, 7)]
    #[case(vec![
            "name",
            "--seed",
            "233213",
            "--length",
            "6",
            "--attempts",
            "5",
            "--options",
            "7",
        ], Some(233213), 6, 5, 7)]
    fn args_parsed_successfully(
        #[case] input: Vec<&str>,
        #[case] expected_seed: Option<u64>,
        #[case] expected_length: u8,
        #[case] expected_attempts: u8,
        #[case] expected_options: u8,
    ) {
        let args = Args::parse_from(input);
        assert_eq!(
            args,
            Args {
                seed: expected_seed,
                length: expected_length,
                attempts: expected_attempts,
                options: expected_options,
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
    #[case(vec!["name", "-o", "0"])]
    #[case(vec!["name", "-o", "11"])]
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

    #[rstest]
    #[case(vec![1, 2, 3, 4], vec![1, 5, 3, 2], Clue::new(2, 1))]
    #[case(vec![1, 2, 3, 4], vec![1, 2, 3, 4], Clue::new(4, 0))]
    #[case(vec![1, 2, 3, 4], vec![4, 3, 2, 1], Clue::new(0, 4))]
    #[case(vec![1, 2, 3, 4], vec![5, 6, 7, 8], Clue::new(0, 0))]
    #[case(vec![1, 2, 3, 4], vec![1, 3, 2, 4], Clue::new(2, 2))]
    #[case(vec![1, 1, 2, 2], vec![1, 2, 1, 2], Clue::new(2, 2))]
    #[case(vec![1, 2, 3, 1], vec![1, 1, 2, 3], Clue::new(1, 3))]
    #[case(vec![1, 2, 2, 3], vec![2, 2, 1, 3], Clue::new(2, 2))]
    #[case(vec![1], vec![1], Clue::new(1, 0))]
    #[case(vec![1], vec![2], Clue::new(0, 0))]
    #[case(vec![1, 2], vec![2, 1], Clue::new(0, 2))]
    #[case(vec![1, 1], vec![1, 1], Clue::new(2, 0))]
    #[case(vec![1, 1], vec![1, 2], Clue::new(1, 0))]
    #[case(vec![3, 4, 5], vec![3, 4, 6], Clue::new(2, 0))]
    #[case(vec![3, 4, 5], vec![5, 3, 4], Clue::new(0, 3))]
    #[case(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], Clue::new(5, 0))]
    #[case(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1], Clue::new(1, 4))]
    #[case(vec![1, 1, 2, 2, 3], vec![1, 2, 1, 3, 2], Clue::new(1, 4))]
    #[case(vec![0, 1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1, 0], Clue::new(0, 6))]
    #[case(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 2, 0, 0, 5, 6, 7], Clue::new(5, 0))]
    #[case(vec![1, 2, 3, 4, 5, 6, 7], vec![7, 6, 5, 4, 3, 2, 1], Clue::new(1, 6))]
    #[case(vec![9,8,7,6,5,4,3,2], vec![2,3,4,5,6,7,8,9], Clue::new(0, 8))]
    #[case(vec![1,1,1,1,1,1,1,1,1], vec![1,1,1,1,1,1,1,1,1], Clue::new(9, 0))]
    #[case(vec![1,2,3,4,5,6,7,8,9], vec![9,8,7,6,5,4,3,2,1], Clue::new(1, 8))]
    #[case(vec![1,2,3,4,5,6,7,8,9,0], vec![0,9,8,7,6,5,4,3,2,1], Clue::new(0, 10))]
    fn test_calculate_clue(
        #[case] solution: Vec<u8>,
        #[case] guess: Vec<u8>,
        #[case] expected: Clue,
    ) {
        assert_eq!(calculate_clue(&solution, &guess), expected);
    }

    #[rstest]
    #[case(vec![1,2,3,4], "1234")]
    #[case(vec![1], "1")]
    #[case(vec![], "")]
    fn test_calculate_vec_u8_to_string(#[case] input: Vec<u8>, #[case] expected: &str) {
        assert_eq!(vec_u8_to_string(input), expected);
    }
}
