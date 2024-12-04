use crate::helper::{read_file_lines, Point2D};
use std::collections::HashMap;

fn solve_part1(lines: &[String]) -> i32 {
    let mut map: HashMap<Point2D, char> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Point2D::new(x as i32, y as i32), c);
        }
    }
    let xs: Vec<&Point2D> = map
        .iter()
        .filter(|(_, c)| **c == 'X')
        .map(|(p, _)| p)
        .collect();
    xs.iter().map(|p| count_xmas_starting_from(p, &map)).sum()
}

fn count_xmas_starting_from(p: &Point2D, map: &HashMap<Point2D, char>) -> i32 {
    let mut count = 0;
    for possible_m in p.neighbours() {
        if let Some('M') = map.get(&possible_m) {
            let dx = possible_m.x - p.x;
            let dy = possible_m.y - p.y;
            if let Some('A') = map.get(&possible_m.mv(dx, dy)) {
                if let Some('S') = map.get(&possible_m.mv(dx * 2, dy * 2)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_part2(lines: &[String]) -> i32 {
    0
}

#[cfg(not(tarpaulin_include))]
pub fn main(path: &String) {
    let full_path = format!("{path}/resources/2024/04/input.txt");
    let lines = read_file_lines(&full_path);
    println!("Day04");
    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_part1_pass_test_input1() {
        let lines = read_file_lines("./resources/2024/04/test1.txt");
        assert_eq!(solve_part1(&lines), 18);
    }

    #[ignore]
    #[test]
    fn should_part2_pass_test_input2() {
        let lines = read_file_lines("./resources/2024/04/test1.txt");
        assert_eq!(solve_part2(&lines), 9);
    }
}
