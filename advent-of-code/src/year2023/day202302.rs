use std::collections::HashMap;
use std::vec::Vec;

use crate::helper::read_file_lines;

#[derive(PartialEq, Eq, Hash)]
enum Colour {
    Red,
    Green,
    Blue,
}

fn solve_part1(lines: &Vec<String>) -> i32 {
    let mut limits = HashMap::new();
    limits.insert(Colour::Red, 12);
    limits.insert(Colour::Green, 13);
    limits.insert(Colour::Blue, 14);

    let mut res = 0;

    for line in lines {
        let by_colon = line.split(": ").collect::<Vec<&str>>();
        let game_id: i32 = by_colon[0].split(' ').last().unwrap().parse().unwrap();
        let tries = by_colon.last().unwrap().split("; ").collect::<Vec<&str>>();
        let mut ok = true;
        'outer: for game_try in tries {
            let colours = game_try.split(", ").collect::<Vec<&str>>();
            for colour in colours {
                let orb_and_value = colour.split(' ').collect::<Vec<&str>>();
                let orb = match *orb_and_value.last().unwrap() {
                    "red" => Colour::Red,
                    "blue" => Colour::Blue,
                    "green" => Colour::Green,
                    c => panic!("Unknown colour {c}"),
                };
                let value: i32 = orb_and_value.first().unwrap().parse().unwrap();
                if *limits.get(&orb).unwrap() < value {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            res += game_id;
        }
    }
    res
}

fn solve_part2(lines: &Vec<String>) -> i32 {
    let mut res = 0;

    for line in lines {
        let mut limits = HashMap::new();
        limits.insert(Colour::Red, 0);
        limits.insert(Colour::Green, 0);
        limits.insert(Colour::Blue, 0);
        let tries = line
            .split(": ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split("; ")
            .collect::<Vec<&str>>();
        for game_try in tries {
            let colours = game_try.split(", ").collect::<Vec<&str>>();
            for colour in colours {
                let orb_and_value = colour.split(' ').collect::<Vec<&str>>();
                let orb = match *orb_and_value.last().unwrap() {
                    "red" => Colour::Red,
                    "blue" => Colour::Blue,
                    "green" => Colour::Green,
                    c => panic!("Unknown colour {c}"),
                };
                let value: i32 = orb_and_value.first().unwrap().parse().unwrap();
                let cur_limit = limits.get(&orb).unwrap();
                if *cur_limit < value {
                    limits.insert(orb, value);
                }
            }
        }
        let mut local = 1;
        for (_, v) in limits {
            local *= v;
        }
        res += local;
    }
    res
}

#[cfg(not(tarpaulin_include))]
pub fn main(path: &String) {
    let full_path = format!("{path}/resources/2023/02/input.txt");
    let lines = read_file_lines(&full_path);
    println!("Day02");
    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_part1_pass_test() {
        let lines = read_file_lines("./resources/2023/02/test1.txt");
        assert_eq!(solve_part1(&lines), 8);
    }

    #[test]
    fn should_part2_pass_test() {
        let lines = read_file_lines("./resources/2023/02/test1.txt");
        assert_eq!(solve_part2(&lines), 2286);
    }
}
