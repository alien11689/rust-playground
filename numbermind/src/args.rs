use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
pub struct Args {
    #[arg(short, long)]
    pub seed: Option<u64>,

    #[arg(short, long, default_value_t = 4, value_parser = clap::value_parser!(u8).range(1..=10))]
    pub length: u8,

    #[arg(short, long, default_value_t = 10, value_parser = clap::value_parser!(u8).range(1..=30))]
    pub attempts: u8,

    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..=10))]
    pub options: u8,

    #[arg(short, long, default_value_t = false)]
    pub unique: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["name"], None, 4, 10, 6, false)]
    #[case(vec!["name", "-s", "233213", "-l", "6", "-a", "5", "-o", "7", "-u"], Some(233213), 6, 5, 7, true)]
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
            "--unique",
        ], Some(233213), 6, 5, 7, true)]
    fn args_parsed_successfully(
        #[case] input: Vec<&str>,
        #[case] expected_seed: Option<u64>,
        #[case] expected_length: u8,
        #[case] expected_attempts: u8,
        #[case] expected_options: u8,
        #[case] expected_unique: bool,
    ) {
        let args = Args::parse_from(input);
        assert_eq!(
            args,
            Args {
                seed: expected_seed,
                length: expected_length,
                attempts: expected_attempts,
                options: expected_options,
                unique: expected_unique,
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
}
