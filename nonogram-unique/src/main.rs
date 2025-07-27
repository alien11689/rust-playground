use std::collections::HashMap;

type Row = [bool; 5];

#[derive(Hash, Eq, PartialEq, Debug)]
struct FixSizeClue {
    len: usize,
    data: [u8; 5],
}

type Clues = [FixSizeClue; 10];

fn get_bit(n: u32, i: u8) -> bool {
    n >> i & 1 == 1
}

fn get_clue(row: Row) -> FixSizeClue {
    let mut cur = 0u8;
    let mut last = false;
    let mut res = [0u8; 5];
    let mut len = 0;
    for x in row.iter() {
        if *x == last {
            cur += 1;
        } else {
            if last {
                res[len] = cur;
                len += 1;
            }
            last = *x;
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
        get_clue([
            get_bit(n, 0),
            get_bit(n, 1),
            get_bit(n, 2),
            get_bit(n, 3),
            get_bit(n, 4),
        ]),
        get_clue([
            get_bit(n, 5),
            get_bit(n, 6),
            get_bit(n, 7),
            get_bit(n, 8),
            get_bit(n, 9),
        ]),
        get_clue([
            get_bit(n, 10),
            get_bit(n, 11),
            get_bit(n, 12),
            get_bit(n, 13),
            get_bit(n, 14),
        ]),
        get_clue([
            get_bit(n, 15),
            get_bit(n, 16),
            get_bit(n, 17),
            get_bit(n, 18),
            get_bit(n, 19),
        ]),
        get_clue([
            get_bit(n, 20),
            get_bit(n, 21),
            get_bit(n, 22),
            get_bit(n, 23),
            get_bit(n, 24),
        ]),
        get_clue([
            get_bit(n, 0),
            get_bit(n, 5),
            get_bit(n, 10),
            get_bit(n, 15),
            get_bit(n, 20),
        ]),
        get_clue([
            get_bit(n, 1),
            get_bit(n, 6),
            get_bit(n, 11),
            get_bit(n, 16),
            get_bit(n, 21),
        ]),
        get_clue([
            get_bit(n, 2),
            get_bit(n, 7),
            get_bit(n, 12),
            get_bit(n, 17),
            get_bit(n, 22),
        ]),
        get_clue([
            get_bit(n, 3),
            get_bit(n, 8),
            get_bit(n, 13),
            get_bit(n, 18),
            get_bit(n, 23),
        ]),
        get_clue([
            get_bit(n, 4),
            get_bit(n, 9),
            get_bit(n, 14),
            get_bit(n, 19),
            get_bit(n, 24),
        ]),
    ]
}

fn main() {
    let mut mem = HashMap::<Clues, u8>::with_capacity(1<<25);
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
    use crate::{FixSizeClue, get_bit, get_clue};

    #[test]
    fn test_get_bit() {
        assert_eq!(get_bit(0, 0), false);
        assert_eq!(get_bit(1, 0), true);
        assert_eq!(get_bit(2, 0), false);
        assert_eq!(get_bit(2, 1), true);
    }

    fn to_fix_size_clue(nums: Vec<u8>) -> FixSizeClue {
        let mut data = [0u8; 5];
        let mut len = 0;
        for x in nums.iter() {
            data[len] = *x;
            len += 1;
        }
        FixSizeClue { len, data }
    }

    #[test]
    fn test_get_clue() {
        assert_eq!(
            get_clue([false, false, false, false, false]),
            to_fix_size_clue(vec![])
        );
        assert_eq!(
            get_clue([false, false, false, false, true]),
            to_fix_size_clue(vec![1])
        );
        assert_eq!(
            get_clue([true, true, false, false, true]),
            to_fix_size_clue(vec![2, 1])
        );
        assert_eq!(
            get_clue([true, true, false, true, true]),
            to_fix_size_clue(vec![2, 2])
        );
        assert_eq!(
            get_clue([true, true, true, true, true]),
            to_fix_size_clue(vec![5])
        );
    }
}
