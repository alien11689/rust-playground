use cookie::Cookie;
use reqwest::Error;
use reqwest::{Client, Response};
use std::collections::HashMap;
use std::env;
use std::fs;

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

    let token = read_advent_of_code_token().expect("Cannot read Advent of Code token");

    let client = Client::builder().cookie_store(true).build()?;

    let cookie: Cookie<'_> = Cookie::build(("session", token))
        .domain("https://adventofcode.com/")
        .path("/")
        .build();

    let mut users_to_stars: HashMap<String, u32> = HashMap::new();

    for year in 2015..=2024 {
        let filename = format!("/tmp/{}_{}.json", user_id, year);
        let file_exists = fs::exists(&filename).expect("Should be able to check file");

        if !file_exists {
            let response = request_leaderboard(&client, &cookie, year, user_id).await?;
            let bytes = response.bytes().await?;
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

async fn request_leaderboard(
    client: &Client,
    cookie: &Cookie<'_>,
    year: i32,
    user_id: &String,
) -> Result<Response, Error> {
    let path = format!("https://adventofcode.com/{year}/leaderboard/private/view/{user_id}.json");
    let response = client
        .get(path)
        .header("Cookie", cookie.to_string())
        .send()
        .await?;
    Ok(response)
}

fn read_advent_of_code_token() -> Result<String, std::io::Error> {
    let home_dir = home::home_dir().expect("No Home dir");
    // println!("Home dir: {:?}", home_dir);
    let expanded_path = home_dir.join(".adventofcode");
    println!("Expanded path: {:?}", expanded_path);
    let content = fs::read_to_string(expanded_path)?;
    Ok(content.trim().to_string())
}
