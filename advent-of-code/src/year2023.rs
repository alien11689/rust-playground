mod day202301;
mod day202302;
mod day202303;
mod day202304;

#[cfg(not(tarpaulin_include))]
pub fn main(path_prefix: &String) {
    let days = vec![
        day202301::main,
        day202302::main,
        day202303::main,
        day202304::main,
    ];
    println!("Year 2023");
    for day in days {
        day(path_prefix);
    }
}
