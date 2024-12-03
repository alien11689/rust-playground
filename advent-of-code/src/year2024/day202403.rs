use crate::helper::read_file_lines;
use regex::Regex;

fn solve_part1(lines: &[String]) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut program = String::new();
    for line in lines {
        program = program + line;
    }
    let mut sum = 0;
    for capture in regex.captures_iter(program.as_str()) {
        // println!("Capture: {:?}", capture);
        let a = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += a * b;
    }
    sum
}

fn solve_part2(lines: &[String]) -> usize {
    // TODO
    0
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

    #[ignore]
    #[test]
    fn should_part2_pass_test_input2() {
        let lines = read_file_lines("./resources/2024/03/test1.txt");
        assert_eq!(solve_part2(&lines), 48);
    }
}
