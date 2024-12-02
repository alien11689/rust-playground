use crate::helper::read_file_lines;
use regex::Regex;
use std::collections::HashSet;
use std::vec::Vec;

fn solve_part1(lines: &[String]) -> usize {
    let numbers = read_input(lines);
    numbers.iter().filter(|cur| is_safe(cur)).count()
}

fn read_input(lines: &[String]) -> Vec<Vec<i32>> {
    let regex = Regex::new(r" +").unwrap();
    lines
        .iter()
        .map(|line| {
            regex
                .split(line)
                .map(|n| n.parse::<i32>().expect("valid number"))
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_safe(vec: &Vec<i32>) -> bool {
    let mut diffs: Vec<i32> = Vec::new();
    let mut prev: Option<&i32> = None;
    for cur in vec {
        match prev {
            None => {
                prev = Some(cur);
            }
            Some(p) => {
                diffs.push(p - cur);
                prev = Some(cur);
            }
        }
    }
    let valid: HashSet<i32> = HashSet::from([1, 2, 3]);
    // println!("{:?}", diffs);
    let correct = (diffs.iter().all(|i| *i > 0) || diffs.iter().all(|i| *i < 0))
        && diffs.iter().map(|i| i.abs()).all(|i| valid.contains(&i));
    // println!("{:?}", correct);
    correct
}

fn solve_part2(lines: &[String]) -> usize {
    read_input(lines)
        .iter()
        .filter(|cur| is_safe_with_damper(cur))
        .count()
}

fn is_safe_with_damper(vec: &Vec<i32>) -> bool {
    if is_safe(vec) {
        return true;
    }
    for (idx, _) in vec.iter().enumerate() {
        let mut cur: Vec<i32> = Vec::new();
        for (i, _) in vec.iter().enumerate() {
            if i != idx {
                cur.push(vec[i]);
            }
        }
        if is_safe(&cur) {
            return true;
        }
    }
    false
}

#[cfg(not(tarpaulin_include))]
pub fn main(path: &String) {
    let full_path = format!("{path}/resources/2024/02/input.txt");
    let lines = read_file_lines(&full_path);
    println!("Day02");
    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_part1_pass_test_input1() {
        let lines = read_file_lines("./resources/2024/02/test1.txt");
        assert_eq!(solve_part1(&lines), 2);
    }

    #[test]
    fn should_part2_pass_test_input2() {
        let lines = read_file_lines("./resources/2024/02/test1.txt");
        assert_eq!(solve_part2(&lines), 4);
    }
}
