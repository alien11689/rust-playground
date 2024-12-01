mod day202401;

#[cfg(not(tarpaulin_include))]
pub fn main(path_prefix: &String) {
    let days = vec![day202401::main];
    println!("Year 2024");
    for day in days {
        day(path_prefix);
    }
}
