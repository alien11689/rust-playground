use std::collections::HashSet;
use std::fs;

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("File should exist")
}

pub fn read_file_lines(file_name: &str) -> Vec<String> {
    read_file(file_name)
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn neighbours(&self) -> HashSet<Point2D> {
        let mut neighbours = HashSet::new();
        neighbours.insert(Point2D {
            x: self.x + 1,
            y: self.y,
        });
        neighbours.insert(Point2D {
            x: self.x + 1,
            y: self.y - 1,
        });
        neighbours.insert(Point2D {
            x: self.x + 1,
            y: self.y + 1,
        });
        neighbours.insert(Point2D {
            x: self.x,
            y: self.y + 1,
        });
        neighbours.insert(Point2D {
            x: self.x,
            y: self.y - 1,
        });
        neighbours.insert(Point2D {
            x: self.x - 1,
            y: self.y,
        });
        neighbours.insert(Point2D {
            x: self.x - 1,
            y: self.y - 1,
        });
        neighbours.insert(Point2D {
            x: self.x - 1,
            y: self.y + 1,
        });
        neighbours
    }
}
