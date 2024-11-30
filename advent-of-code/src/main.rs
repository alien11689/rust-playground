mod helper;
mod year2015;
mod year2023;

use std::env;

#[cfg(not(tarpaulin_include))]
fn main() {
    let params: Vec<String> = env::args().skip(1).collect();
    let default_path_prefix = String::from(".");
    let path_prefix = params.first().unwrap_or(&default_path_prefix);
    let years = vec![year2015::main, year2023::main];
    for year in years {
        year(path_prefix);
    }
}
