use cookie::Cookie;
use reqwest::Client;
use reqwest::Error;
use std::collections::HashMap;
use std::env;
use std::fs;

const SESSION_ID_FILE: &str = "~/.adventofcode";

#[derive(serde::Deserialize, Debug)]
struct LeaderBoard {
    members: HashMap<String, Member>,
}

#[derive(serde::Deserialize, Debug)]
struct Member {
    stars: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let params: Vec<String> = env::args().skip(1).collect();
    // TODO verify parameter count
    let user_id = &params[0];

    let home_dir = home::home_dir().expect("No Home dir");
    // println!("Home dir: {:?}", home_dir);
    let expanded_path = home_dir.join(SESSION_ID_FILE.trim_start_matches("~/"));
    // println!("Expanded path: {:?}", expanded_path);
    let content = fs::read_to_string(expanded_path).expect("File should exist");
    let content = content.trim();
    // println!("{content}");

    let client = Client::builder().cookie_store(true).build()?;

    let cookie = Cookie::build(("session", content))
        .domain("https://adventofcode.com/")
        .path("/")
        .build();

    let mut users_to_stars: HashMap<String, u32> = HashMap::new();

    for year in 2015..=2024 {
        let filename = format!("/tmp/{}_{}.json", user_id, year);
        let file_exists = fs::exists(&filename).expect("Should be able to check file");

        if !file_exists {
            let path =
                format!("https://adventofcode.com/{year}/leaderboard/private/view/{user_id}.json");
            let bytes = client
                .get(path)
                .header("Cookie", cookie.to_string())
                .send()
                .await?
                .bytes()
                .await?;
            fs::write(&filename, bytes).expect("Should be able to write to file");
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
