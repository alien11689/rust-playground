use aoc_common::AocClient;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let params: Vec<String> = env::args().skip(1).collect();
    // TODO verify parameter count
    let year: &u32 = &params[0].parse().expect("Year parameter must be number");
    let day: &u32 = &params[1].parse().expect("Day parameter must be number");

    let aoc_client = AocClient::new();
    aoc_client
        .save_input_to_file(*year, *day, &"input.txt".to_string())
        .await?;
    Ok(())
}
