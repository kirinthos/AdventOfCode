use std::collections::HashMap;

use crate::Problem;

#[derive(Debug, Clone)]
enum Space {
    Symbol(Symbol),
    Number(Number),
}

#[derive(Debug, Clone)]
struct Symbol {
    symbol: char,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Number {
    x: usize,
    y: usize,
    width: usize,
}

fn parse_lines(lines: &[String]) -> Vec<Space> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let mut result = row.chars().enumerate().fold(
                (Vec::with_capacity(row.len()), None),
                |(mut spaces, current), (x, character)| match (character, current) {
                    ('.', None) => (spaces, None),

                    // start parsing a digit
                    (c, None) if c.is_ascii_digit() => (spaces, Some(Number { x, y, width: 1 })),
                    // continue parsing digit
                    (c, Some(mut n)) if c.is_ascii_digit() => {
                        n.width += 1;
                        (spaces, Some(n))
                    }

                    // finish parsing a digit and encounter a space
                    ('.', Some(n)) => {
                        spaces.push(Space::Number(n));
                        (spaces, None)
                    }
                    // finish parsing a digit after encountering a symbol
                    (symbol, Some(n)) => {
                        spaces.push(Space::Number(n));
                        // and a symbol, because '.' is handled above
                        spaces.push(Space::Symbol(Symbol { symbol, x, y }));
                        (spaces, None)
                    }

                    // parse a character
                    (symbol, None) => {
                        spaces.push(Space::Symbol(Symbol { symbol, x, y }));
                        (spaces, None)
                    }
                },
            );
            // and don't forget to add a number if it's the last thing we're doing!
            if let Some(n) = result.1 {
                result.0.push(Space::Number(n));
            }
            result.0
        })
        .collect()
}

pub struct Problem3;
impl Problem for Problem3 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let spaces = parse_lines(lines);
        let board: HashMap<_, _> = spaces
            .iter()
            .map(|s| match s {
                Space::Symbol(ref v) => ((v.x, v.y), s),
                Space::Number(ref v) => ((v.x, v.y), s),
            })
            .collect();

        spaces
            .iter()
            .filter_map(|s| match s {
                Space::Symbol(_) => None,
                Space::Number(n) => Some(n),
            })
            .filter_map(|Number { x, y, width }| {
                let x = *x;
                let y = *y;
                let x1 = x.saturating_sub(1);
                let y1 = y.saturating_sub(1);
                let xmax = x + width;
                (x1..=xmax)
                    .flat_map(|x| [(x, y1), (x, y + 1)])
                    .chain([(x1, y), (xmax, y)])
                    .any(|point| matches!(board.get(&point), Some(Space::Symbol(_))))
                    .then(|| lines[y][x..xmax].parse::<u64>().unwrap())
            })
            .sum::<u64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        // ooh this is a bad solution
        let spaces = parse_lines(lines);
        let numbers = spaces
            .iter()
            .filter_map(|s| match s {
                Space::Symbol(_) => None,
                Space::Number(n) => Some(n),
            })
            .collect::<Vec<_>>();

        spaces
            .iter()
            .filter_map(|s| match s {
                Space::Symbol(s @ Symbol { symbol: '*', .. }) => Some(s),
                _ => None,
            })
            .filter_map(|gear| {
                let gearx = gear.x as i32;
                let geary = gear.y as i32;
                let (one, two) = numbers.iter().fold((None, None), |acc, n| {
                    let nx = n.x as i32;
                    let nx2 = (n.x + n.width) as i32;
                    let ny = n.y as i32;
                    let found =
                        (nx..nx2).any(|x| (geary - ny).abs() <= 1 && (gearx - x).abs() <= 1);
                    match (found, acc) {
                        (true, (None, None)) => (Some(n), None),
                        (true, (a @ Some(_), None)) => (a, Some(n)),
                        (true, _) => acc,
                        (false, _) => acc,
                    }
                });

                match (one, two) {
                    (Some(one), Some(two)) => {
                        let one = lines[one.y][one.x..(one.x + one.width)]
                            .parse::<u64>()
                            .unwrap();
                        let two = lines[two.y][two.x..(two.x + two.width)]
                            .parse::<u64>()
                            .unwrap();
                        Some(one * two)
                    }
                    _ => None,
                }
            })
            .sum::<u64>()
            .to_string()

        // for each *, loop through numbers and pull out any that intersect

        // if len() > 1, multiply
        // sum result
    }
}
