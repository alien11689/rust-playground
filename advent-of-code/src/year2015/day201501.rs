use crate::helper::read_file_lines;

fn solve_part1(input: &str) -> i32 {
    let mut pos = 0;
    for cur in input.chars() {
        match cur {
            '(' => pos += 1,
            ')' => pos -= 1,
            _ => {
                panic!("Unknown char {cur}")
            }
        }
    }
    pos
}

fn solve_part2(input: &str) -> i32 {
    let mut pos = 0;
    for (idx, cur) in input.chars().enumerate() {
        match cur {
            '(' => pos += 1,
            ')' => pos -= 1,
            _ => {
                panic!("Unknown char {cur}")
            }
        }
        if pos < 0 {
            return (idx as i32) + 1;
        }
    }
    panic!("No basement for {input}")
}

#[cfg(not(tarpaulin_include))]
pub fn main(path: &String) {
    let full_path = format!("{path}/resources/2015/01/input.txt");
    let lines = read_file_lines(&full_path);
    let input = lines.first().unwrap();
    println!("Day 01");
    println!("Part 1");
    println!("{}", solve_part1(input));
    println!("Part 2");
    println!("{}", solve_part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case ("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn should_part1_pass_test_inputs(#[case] inp: &str, #[case] res: i32) {
        assert_eq!(solve_part1(&String::from(inp)), res);
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn should_part2_pass_test(#[case] inp: &str, #[case] res: i32) {
        assert_eq!(solve_part2(&String::from(inp)), res);
    }
}
