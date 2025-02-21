#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    let mut res = 0;
    if b < 0 {
        for _ in b..0 {
            unsafe {
                res = add(res, a);
            }
        }
        res = -res;
    } else {
        for _ in 0..b {
            unsafe {
                res = add(res, a);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_both_positive() {
        let result = multiply(2, 5);
        assert_eq!(result, 10);
    }

    #[test]
    fn it_works_for_both_negative() {
        let result = multiply(-3, -5);
        assert_eq!(result, 15);
    }

    #[test]
    fn it_works_for_first_negative() {
        let result = multiply(-3, 5);
        assert_eq!(result, -15);
    }

    #[test]
    fn it_works_for_second_negative() {
        let result = multiply(3, -5);
        assert_eq!(result, -15);
    }
}
