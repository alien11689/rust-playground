use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let file_names: Vec<String> = env::args().skip(1).collect();
    for file_name in file_names.iter() {
        println!(
            "===========\nReading from file {:?}\n===========",
            fs::canonicalize(PathBuf::from(file_name))
        );
        let content = fs::read_to_string(file_name).expect("File should exist");
        println!("{content}");
    }
}
