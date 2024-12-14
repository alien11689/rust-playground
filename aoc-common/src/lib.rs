use cookie::Cookie;
use reqwest::Client;
use std::error::Error;
use std::fs;

pub mod json;

pub struct AocClient<'a> {
    client: Client,
    cookie: Cookie<'a>,
}

impl Default for AocClient<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AocClient<'a> {
    pub fn new() -> AocClient<'a> {
        let token = read_advent_of_code_token().expect("Cannot read Advent of Code token");

        let client = Client::builder()
            .cookie_store(true)
            .build()
            .expect("Cannot create HTTP client");

        let cookie: Cookie<'a> = Cookie::build(("session", token))
            .domain("https://adventofcode.com/")
            .path("/")
            .build();

        AocClient { client, cookie }
    }

    pub async fn save_leaderboard_to_file(
        &self,
        year: u32,
        user_id: &String,
        file_name: &String,
    ) -> Result<(), Box<dyn Error>> {
        let path =
            format!("https://adventofcode.com/{year}/leaderboard/private/view/{user_id}.json");
        self.get_and_save_to_file(path, file_name).await
    }

    pub async fn save_input_to_file(
        &self,
        year: u32,
        day: u32,
        file_name: &String,
    ) -> Result<(), Box<dyn Error>> {
        let path = format!("https://adventofcode.com/{year}/day/{day}/input");
        self.get_and_save_to_file(path, file_name).await
    }

    async fn get_and_save_to_file(
        &self,
        path: String,
        file_name: &String,
    ) -> Result<(), Box<dyn Error>> {
        let response = self
            .client
            .get(path)
            .header("Cookie", self.cookie.to_string())
            .send()
            .await?;
        let bytes = response.bytes().await?;
        fs::write(file_name, bytes)?;
        Ok(())
    }
}

fn read_advent_of_code_token() -> Result<String, std::io::Error> {
    let home_dir = home::home_dir().expect("No Home dir");
    // println!("Home dir: {:?}", home_dir);
    let expanded_path = home_dir.join(".adventofcode");
    // println!("Expanded path: {:?}", expanded_path);
    let content = fs::read_to_string(expanded_path)?;
    Ok(content.trim().to_string())
}
