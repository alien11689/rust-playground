mod day201501;

#[cfg(not(tarpaulin_include))]
pub fn main(path_prefix: &String) {
    let days = vec![day201501::main];
    println!("Year 2015");
    for day in days {
        day(path_prefix);
    }
}
