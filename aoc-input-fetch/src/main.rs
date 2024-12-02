use cookie::Cookie;
use reqwest::Client;
use reqwest::Error;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use tokio::task;

const SESSION_ID_FILE: &str = "~/.adventofcode";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let params: Vec<String> = env::args().skip(1).collect();
    // TODO verify parameter count
    let year = &params[0];
    let day = &params[1];

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

    let path = format!("https://adventofcode.com/{year}/day/{day}/input");
    let res = client
        .get(path)
        .header("Cookie", cookie.to_string())
        .send()
        .await?;

    if res.status().is_success() {
        let content = res.text().await?;
        // TODO set file location
        let file_save = task::spawn_blocking(move || {
            let mut file = File::create("input.txt").expect("Cannot create file");
            file.write_all(content.as_bytes())
                .expect("Cannot write to file");
            Ok::<(), io::Error>(())
        })
        .await;
        if let Err(text) = file_save {
            println!("File save operation failed: {}", text);
        }
    } else {
        println!("No file content: {}", res.status());
    }
    Ok(())
}
