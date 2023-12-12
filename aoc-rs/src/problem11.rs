use itertools::Itertools;

use crate::{point::Point, Problem};

fn parse_lines(lines: &[String], modifier_base: u64) -> Vec<Point> {
    // ah, x expands too, so transpose and find any x modifier coordinates
    let mut xmodifiers = Vec::new();
    for x in 0..lines[0].len() {
        if lines
            .iter()
            .map(|line| line.as_bytes()[x])
            .all(|c| c as char == '.')
        {
            xmodifiers.push(x);
        }
    }

    let mut ymodifier = 0;
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            if line.find('#').is_none() {
                ymodifier += modifier_base - 1;
            }

            let y = y as u64 + ymodifier;
            let mut xmodifier = 0;

            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if xmodifiers.iter().any(|v| *v == x) {
                        xmodifier += modifier_base - 1;
                    }
                    (x as u64 + xmodifier, c)
                })
                .filter_map(|(x, c)| (c == '#').then_some((x, y).into()))
                // ah, why do we have to collect, it's a FnMut + modifier issue. but i don't want to fold
                .collect::<Vec<_>>()
        })
        .collect()
}

pub struct Problem11;
impl Problem for Problem11 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let galaxies = parse_lines(lines, 1);
        galaxies
            .iter()
            .tuple_combinations()
            .map(|(one, two)| one.manhattan_distance(two) as u64)
            .sum::<u64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let galaxies = parse_lines(lines, 1_000_000);
        galaxies
            .iter()
            .tuple_combinations()
            .map(|(one, two)| one.manhattan_distance(two) as u64)
            .sum::<u64>()
            .to_string()
    }
}
