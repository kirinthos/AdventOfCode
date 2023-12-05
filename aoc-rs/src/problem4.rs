use std::collections::HashSet;

use crate::Problem;

type Card = HashSet<u64>;

fn parse_card(line: &str) -> Card {
    line.split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn parse_lines(lines: &[String]) -> Vec<(Card, Card)> {
    lines
        .iter()
        .map(|line| {
            let (winners, card) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

            (parse_card(winners), parse_card(card))
        })
        .collect()
}

pub struct Problem4;
impl Problem for Problem4 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let games = parse_lines(lines);
        games
            .into_iter()
            .filter_map(
                |(winners, card)| match winners.intersection(&card).count() {
                    0 => None,
                    v => Some(2_u64.pow((v - 1) as u32)),
                },
            )
            .sum::<u64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let games = parse_lines(lines);
        let mut copies = vec![1_u64; games.len()];
        copies[0] = 1;

        for (i, (winners, card)) in games.into_iter().enumerate() {
            let n = winners.intersection(&card).count();
            let win = copies[i];
            copies[(i + 1)..=(i + n)].iter_mut().for_each(|x| *x += win);
        }

        copies.into_iter().sum::<u64>().to_string()
    }
}
