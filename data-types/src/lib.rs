pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Clone, Debug, Copy)]
pub struct MyType {
    a: i32,
    b: i32,
    pub c: f32,
}

impl MyType {
    pub fn new(a: i32, b: i32, c: f32) -> MyType {
        MyType { a, b, c }
    }

    pub fn sum(&self) -> f32 {
        self.a as f32 + self.b as f32 + self.c
    }
}

pub mod lib2;

#[cfg(test)]
mod tests {

    pub struct Point(i32, i32);

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_named_tuple() {
        let result = Point(5, 4);
        assert_eq!(result.0, 5);
        assert_eq!(result.1, 4);
    }

    #[test]
    fn should_be_a_point() {
        let p = Point(5, 4);
        assert_eq!(p.0, 5);
        assert_eq!(p.1, 4);
    }

    #[test]
    fn shoud_sum() {
        let mt = MyType::new(3, 4, 5.0);
        assert_eq!(mt.sum(), 12_f32);
        assert_eq!(mt.clone().a, mt.a);
    }

    #[test]
    fn should_print_bla() {
        assert_eq!(lib2::bla(), "bla");
    }
}
