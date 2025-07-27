use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct FixSizeClue {
    len: usize,
    data: [u8; 5],
}

type Clues = [FixSizeClue; 10];

fn get_bit(n: u32, i: u8) -> bool {
    n >> i & 1 == 1
}

fn get_clue(n: u32, idx: [u8; 5]) -> FixSizeClue {
    let mut cur = 0u8;
    let mut last = false;
    let mut res = [0u8; 5];
    let mut len = 0;
    for i in idx.iter() {
        let x = get_bit(n, *i);
        if x == last {
            cur += 1;
        } else {
            if last {
                res[len] = cur;
                len += 1;
            }
            last = x;
            cur = 1;
        }
    }
    if last {
        res[len] = cur;
        len += 1;
    }
    FixSizeClue { len, data: res }
}

fn get_clues(n: u32) -> Clues {
    [
        get_clue(n, [0, 1, 2, 3, 4]),
        get_clue(n, [5, 6, 7, 8, 9]),
        get_clue(n, [10, 11, 12, 13, 14]),
        get_clue(n, [15, 16, 17, 18, 19]),
        get_clue(n, [20, 21, 22, 23, 24]),
        get_clue(n, [0, 5, 10, 15, 20]),
        get_clue(n, [1, 6, 11, 16, 21]),
        get_clue(n, [2, 7, 12, 17, 22]),
        get_clue(n, [3, 8, 13, 18, 23]),
        get_clue(n, [4, 9, 14, 19, 24]),
    ]
}

fn main() {
    let mut mem = HashMap::<Clues, u8>::with_capacity(1 << 25);
    for n in 0u32..(1 << 25) {
        *mem.entry(get_clues(n)).or_insert(0) += 1;
    }
    println!("All possible nonograms: {:?}", 1 << 25);
    println!("Possible clues set: {:?}", mem.len());
    println!(
        "Clues leading to single solution: {:?}",
        mem.values().filter(|x| **x == 1).count()
    );
}

#[cfg(test)]
mod tests {
    use crate::get_bit;

    #[test]
    fn test_get_bit() {
        assert_eq!(get_bit(0, 0), false);
        assert_eq!(get_bit(1, 0), true);
        assert_eq!(get_bit(2, 0), false);
        assert_eq!(get_bit(2, 1), true);
    }
}
