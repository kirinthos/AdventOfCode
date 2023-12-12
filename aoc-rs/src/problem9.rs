use itertools::Itertools;

use crate::Problem;

fn parse_lines(lines: &[String]) -> Vec<Vec<i64>> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn solve(lines: &[String], part2: bool) -> String {
    parse_lines(lines)
        .into_iter()
        .map(|mut measures| {
            if part2 {
                measures.reverse();
            }
            let mut n = *measures.last().unwrap();
            let mut m = measures;

            loop {
                m = m
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>();
                n += m.last().unwrap();
                if m.iter().all(|x| *x == 0) {
                    break;
                }
            }

            n
        })
        .sum::<i64>()
        .to_string()
}

pub struct Problem9;
impl Problem for Problem9 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        solve(lines, false)
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        solve(lines, true)
    }
}
