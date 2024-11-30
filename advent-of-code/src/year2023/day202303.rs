use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use crate::helper::{read_file_lines, Point2D};

fn parse_input(
    lines: &[String],
    is_interesting_symbol: fn(char) -> bool,
) -> (HashSet<Point2D>, Vec<(u32, HashSet<Point2D>)>) {
    let mut symbols: HashSet<Point2D> = HashSet::new();
    let mut numbers: Vec<(u32, HashSet<Point2D>)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut cur_num: u32 = 0;
        let mut cur_points: HashSet<Point2D> = HashSet::new();
        for (x, c) in line.chars().enumerate() {
            let p = Point2D {
                x: x as i32,
                y: y as i32,
            };
            if c.is_ascii_digit() {
                cur_points.insert(p);
                cur_num = cur_num * 10 + c.to_digit(10).unwrap();
            } else {
                if cur_num != 0 {
                    numbers.push((cur_num, cur_points));
                    cur_num = 0;
                    cur_points = HashSet::new();
                }
                if is_interesting_symbol(c) {
                    symbols.insert(p);
                }
            }
        }
        if cur_num != 0 {
            numbers.push((cur_num, cur_points));
        }
    }
    (symbols, numbers)
}

fn solve_part1(lines: &[String]) -> u32 {
    let (symbols, numbers) = parse_input(lines, |c| c != '.');
    let mut sum = 0;
    for (n, points) in numbers {
        let neighbours: HashSet<Point2D> = points.iter().flat_map(Point2D::neighbours).collect();
        let intersect = symbols.intersection(&neighbours);
        if intersect.count() > 0 {
            sum += n;
        }
    }
    sum
}

fn solve_part2(lines: &[String]) -> u32 {
    let (gears, numbers) = parse_input(lines, |c| c == '*');
    let mut gears_adj = HashMap::new();
    for (n, points) in numbers {
        let neighbours: HashSet<Point2D> = points.iter().flat_map(Point2D::neighbours).collect();
        let intersect = gears.intersection(&neighbours);
        if let Some(gear) = intersect.last() {
            let entry = gears_adj.entry(*gear).or_insert(Vec::new());
            entry.push(n);
        }
    }
    gears_adj
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .reduce(|acc, e| acc + e)
        .unwrap()
}

#[cfg(not(tarpaulin_include))]
pub fn main(path: &String) {
    let full_path = format!("{path}/resources/2023/03/input.txt");
    let lines = read_file_lines(&full_path);
    println!("Day 03");
    println!("Part 1");
    println!("{}", solve_part1(&lines));
    println!("Part 2");
    println!("{}", solve_part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_part1_pass_test() {
        let lines = read_file_lines("./resources/2023/03/test1.txt");
        assert_eq!(solve_part1(&lines), 4361);
    }

    #[test]
    fn should_part2_pass_test() {
        let lines = read_file_lines("./resources/2023/03/test1.txt");
        assert_eq!(solve_part2(&lines), 467835);
    }
}
