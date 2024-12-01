use crate::helper::read_file_lines;
use regex::Regex;
use std::vec::Vec;

fn read_locations(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let regex = Regex::new(r" +").expect("Invalid regex");
    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = regex.split(line).collect();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[1].parse().unwrap();
        lefts.push(left);
        rights.push(right);
    }
    lefts.sort();
    rights.sort();
    (lefts, rights)
}

fn solve_part1(lefts: &[i32], rights: &[i32]) -> i32 {
    lefts
        .iter()
        .zip(rights.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn solve_part2(lefts: &[i32], rights: &[i32]) -> i32 {
    let mut res = 0;
    for left in lefts {
        let count = rights.iter().filter(|&right| *right == *left).count() as i32;
        res += left * count;
    }
    res
}

#[cfg(not(tarpaulin_include))]
pub fn main(path: &String) {
    let full_path = format!("{path}/resources/2024/01/input.txt");
    let lines = read_file_lines(&full_path);
    let (lefts, rights) = read_locations(&lines);
    println!("Day01");
    println!("{}", solve_part1(&lefts, &rights));
    println!("{}", solve_part2(&lefts, &rights));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_part1_pass_test_input1() {
        let lines = read_file_lines("./resources/2024/01/test1.txt");
        let (lefts, rights) = read_locations(&lines);
        assert_eq!(solve_part1(&lefts, &rights), 11);
    }

    #[test]
    fn should_part2_pass_test_input2() {
        let lines = read_file_lines("./resources/2024/01/test1.txt");
        let (lefts, rights) = read_locations(&lines);
        assert_eq!(solve_part2(&lefts, &rights), 31);
    }
}
