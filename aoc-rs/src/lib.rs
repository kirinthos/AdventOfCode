pub mod point;
pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;

use num_enum::TryFromPrimitive;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(TryFromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Problems {
    Invalid = 0,
    Problem1,
    Problem2,
    Problem3,
    Problem4,
    Problem5,
    Problem6,
    Problem7,
    Problem8,
    Problem9,
    Problem10,
    Problem11,
    Problem12,
    Problem13,
    Problem14,
    Problem15,
    Problem16,
    Problem17,
    Problem18,
    Problem19,
    Problem20,
    Problem21,
    Problem22,
    Problem23,
    Problem24,
    Problem25,
}

pub enum SubProblem {
    ExamplePart1,
    Part1,
    ExamplePart2,
    Part2,
}

pub trait Problem {
    fn solve(&mut self, lines: &[String], part: SubProblem) -> String {
        match part {
            SubProblem::ExamplePart1 => self.solve_part1(lines),
            SubProblem::Part1 => self.solve_part1(lines),
            SubProblem::ExamplePart2 => self.solve_part2(lines),
            SubProblem::Part2 => self.solve_part2(lines),
        }
    }

    fn solve_part1(&mut self, lines: &[String]) -> String;

    fn solve_part2(&mut self, lines: &[String]) -> String;
}

// helper functions
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}
