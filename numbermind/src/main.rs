use clap::Parser;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

#[derive(Parser, Debug, PartialEq)]
struct Args {
    #[arg(short, long)]
    seed: Option<u64>,

    #[arg(short, long, default_value_t = 4, value_parser = clap::value_parser!(u8).range(1..=10))]
    length: u8,
}

fn generate_random_code(seed: u64, length: u8) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(seed);
    let random_code: Vec<u8> = (0..length).map(|_| rng.random_range(0..=9)).collect();
    random_code
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
}

#[cfg(test)]
mod random_generation_test {
    use super::*;

    #[test]
    fn test_random_number_generation() {
        assert_eq!(
            generate_random_code(3567657657632322323, 4),
            vec![7, 3, 7, 8]
        );
        assert_eq!(generate_random_code(343243242, 6), vec![6, 1, 5, 2, 4, 0]);
    }
}

#[cfg(test)]
mod argument_parser {
    use super::*;
    use clap::Parser;

    #[test]
    fn default_parameters() {
        let args = Args::parse_from(&["name"]);
        assert_eq!(
            args,
            Args {
                seed: None,
                length: 4,
            }
        );
    }

    #[test]
    fn short_override() {
        let args = Args::parse_from(&["name", "-s", "233213", "-l", "6"]);
        assert_eq!(
            args,
            Args {
                seed: Some(233213),
                length: 6,
            }
        );
    }

    #[test]
    fn long_override() {
        let args = Args::parse_from(&["name", "--seed", "233213", "--length", "6"]);
        assert_eq!(
            args,
            Args {
                seed: Some(233213),
                length: 6,
            }
        );
    }

    #[test]
    fn reject_negative_seed() {
        let res = Args::try_parse_from(&["name", "--seed", "-32321321"]);
        assert!(res.is_err())
    }

    #[test]
    fn reject_too_big_seed() {
        let res = Args::try_parse_from(&["name", "--seed", "87897987779878979898988932321321"]);
        assert!(res.is_err())
    }

    #[test]
    fn reject_too_small_length() {
        let res = Args::try_parse_from(&["name", "-l", "0"]);
        assert!(res.is_err())
    }

    #[test]
    fn reject_too_big_length() {
        let res = Args::try_parse_from(&["name", "-l", "11"]);
        assert!(res.is_err())
    }
}
