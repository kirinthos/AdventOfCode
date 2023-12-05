use std::{cmp::max, ops::Not};

use strum::EnumString;

use crate::Problem;

#[derive(EnumString)]
enum Color {
    #[strum(ascii_case_insensitive)]
    Red,
    #[strum(ascii_case_insensitive)]
    Green,
    #[strum(ascii_case_insensitive)]
    Blue,
}

pub struct Problem2;
impl Problem for Problem2 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .filter_map(|l| {
                let (id, games) = l.split_once(": ").unwrap();
                games
                    .split("; ")
                    .all(|game| {
                        game.split(", ")
                            .any(|draw| {
                                let (count, color) = draw.split_once(' ').unwrap();
                                let count: u64 = count.parse().unwrap();
                                let color: Color = color.try_into().unwrap();

                                match color {
                                    Color::Red => count > 12,
                                    Color::Green => count > 13,
                                    Color::Blue => count > 14,
                                }
                            })
                            .not()
                    })
                    .then(|| id.split_once(' ').unwrap().1.parse::<u64>().unwrap())
            })
            .sum::<u64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| {
                let (_, games) = l.split_once(": ").unwrap();
                let rgb = games.split("; ").flat_map(|g| g.split(", ")).fold(
                    (0_u64, 0_u64, 0_u64),
                    |acc, n| {
                        let (count, color) = n.split_once(' ').unwrap();
                        let count: u64 = count.parse().unwrap();
                        let color: Color = color.try_into().unwrap();

                        match color {
                            Color::Red => (max(acc.0, count), acc.1, acc.2),
                            Color::Green => (acc.0, max(acc.1, count), acc.2),
                            Color::Blue => (acc.0, acc.1, max(acc.2, count)),
                        }
                    },
                );
                rgb.0 * rgb.1 * rgb.2
            })
            .sum::<u64>()
            .to_string()
    }
}
