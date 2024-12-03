use crate::helper::read_file_lines;
use regex::Regex;

fn solve_part1(lines: &[String]) -> i32 {
    let program = join_lines(lines);
    calculate_sum(program)
}

fn calculate_sum(program: String) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for capture in regex.captures_iter(program.as_str()) {
        // println!("Capture: {:?}", capture);
        let a = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += a * b;
    }
    sum
}

fn join_lines(lines: &[String]) -> String {
    let mut program = String::new();
    for line in lines {
        program = program + line;
    }
    program
}

fn solve_part2(lines: &[String]) -> i32 {
    let program = join_lines(lines);
    let split_regex = Regex::new(r"do(n't|)\(\)").unwrap();
    let mut split_indices: Vec<usize> = Vec::new();
    split_indices.push(0);
    for part in split_regex.find_iter(program.as_str()) {
        // println!("{:?}", part);
        // println!("{:?}", program[part.start()..program.len()].to_string());
        split_indices.push(part.start())
    }
    split_indices.push(program.len());
    // println!("{:?}", split_indices);
    let mut sum = 0;
    for chunk in split_indices.windows(2) {
        let part = program[chunk[0]..chunk[1]].to_string();
        if !part.starts_with(r"don't()") {
            sum += calculate_sum(part);
        }
    }
    sum
}

#[cfg(not(tarpaulin_include))]
pub fn main(path: &String) {
    let full_path = format!("{path}/resources/2024/03/input.txt");
    let lines = read_file_lines(&full_path);
    println!("Day03");
    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_part1_pass_test_input1() {
        let lines = read_file_lines("./resources/2024/03/test1.txt");
        assert_eq!(solve_part1(&lines), 161);
    }

    #[test]
    fn should_part2_pass_test_input2() {
        let lines = read_file_lines("./resources/2024/03/test1.txt");
        assert_eq!(solve_part2(&lines), 48);
    }
}
