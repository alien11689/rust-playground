use aoc_common::json::LeaderBoard;
use aoc_common::AocClient;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let params: Vec<String> = env::args().skip(1).collect();
    // TODO verify parameter count
    let user_id = &params[0];

    let aoc_client = AocClient::new();

    let mut users_to_stars: HashMap<String, u32> = HashMap::new();

    for year in 2015..=2024 {
        let filename = format!("/tmp/{}_{}.json", user_id, year);
        let file_exists = fs::exists(&filename).expect("Should be able to check file");

        if !file_exists {
            aoc_client
                .save_leaderboard_to_file(year, user_id, &filename)
                .await?;
        }

        let file = fs::File::open(filename).expect("File exists");
        let reader = std::io::BufReader::new(file);
        let leader_board: LeaderBoard =
            serde_json::from_reader(reader).expect("Valid json deserialized");
        leader_board.members.values().for_each(|member| {
            *users_to_stars.entry(member.name.clone()).or_insert(0) += member.stars;
        });
    }

    let mut sorted_users = users_to_stars.into_iter().collect::<Vec<(String, u32)>>();
    sorted_users.sort_by(|a, b| b.1.cmp(&a.1));

    for (user, stars) in sorted_users {
        println!("{} has {} stars", user, stars);
    }
    Ok(())
}
