use std::collections::HashMap;

use num::integer::lcm;

use crate::Problem;

enum Move {
    Right,
    Left,
}

fn parse_lines(lines: &[String]) -> (Vec<Move>, HashMap<&str, (&str, &str)>) {
    let moves = lines[0]
        .chars()
        .map(|c| match c {
            'R' => Move::Right,
            'L' => Move::Left,
            _ => panic!("invalid move"),
        })
        .collect();

    let map = lines[2..]
        .iter()
        .map(|l| {
            let (name, branches) = l.split_once(" = ").unwrap();
            let branches = branches
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();

            (name, branches)
        })
        .collect();

    (moves, map)
}

pub struct Problem8;
impl Problem for Problem8 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let (moves, map) = parse_lines(lines);
        let mut current = "AAA";
        let mut number_of_moves = 0;

        for m in moves.iter().cycle() {
            if current == "ZZZ" {
                break;
            }

            number_of_moves += 1;
            current = match m {
                Move::Right => map[current].1,
                Move::Left => map[current].0,
            };
        }

        number_of_moves.to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let (moves, map) = parse_lines(lines);
        let mut currents = map
            .keys()
            .filter_map(|k| k.ends_with('A').then_some((k, 0_u64)))
            .collect::<Vec<_>>();

        for m in moves.iter().cycle() {
            if currents.iter().all(|c| c.0.ends_with('Z')) {
                break;
            }

            let remaining = currents.iter_mut().filter(|(c, _)| !c.ends_with('Z'));

            match m {
                Move::Right => remaining.for_each(|(name, num)| {
                    *name = &map[*name].1;
                    *num += 1;
                }),
                Move::Left => remaining.for_each(|(name, num)| {
                    *name = &map[*name].0;
                    *num += 1;
                }),
            };
        }

        currents
            .into_iter()
            .map(|(_, n)| n)
            .reduce(lcm)
            .unwrap()
            .to_string()
    }
}
