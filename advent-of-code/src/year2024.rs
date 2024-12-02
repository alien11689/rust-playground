mod day202401;
mod day202402;

#[cfg(not(tarpaulin_include))]
pub fn main(path_prefix: &String) {
    let days = vec![day202401::main, day202402::main];
    println!("Year 2024");
    for day in days {
        day(path_prefix);
    }
}
