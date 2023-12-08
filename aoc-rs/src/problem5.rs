use std::ops::RangeInclusive;

use crate::Problem;

#[derive(Debug, Clone)]
struct SourceMap {
    source_range: RangeInclusive<u64>,
    destination_start: u64,
}

impl SourceMap {
    fn map_source(&self, source: u64) -> Option<u64> {
        self.source_range
            .contains(&source)
            .then(|| source - self.source_range.start() + self.destination_start)
    }
}

impl From<&String> for SourceMap {
    fn from(value: &String) -> Self {
        let mut values = value.split_whitespace().map(|n| n.parse::<u64>().unwrap());
        let destination_start = values.next().unwrap();
        let s = values.next().unwrap();
        let n = values.next().unwrap();

        Self {
            source_range: s..=(s + n),
            destination_start,
        }
    }
}

fn parse_lines(lines: &[String]) -> (Vec<u64>, Vec<Vec<SourceMap>>) {
    let seeds = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let maps = lines[2..]
        .split(|s| s.is_empty())
        .map(|map| map.iter().skip(1).map(From::from).collect())
        .collect();

    (seeds, maps)
}

pub struct Problem5;
impl Problem for Problem5 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let (seeds, maps) = parse_lines(lines);
        seeds
            .into_iter()
            .map(|seed| {
                maps.iter().fold(seed, |acc, section_maps| {
                    section_maps
                        .iter()
                        .find_map(|source_map| source_map.map_source(acc))
                        .unwrap_or(acc)
                })
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let (seeds, maps) = parse_lines(lines);
        seeds
            .chunks(2)
            .flat_map(|s| s[0]..(s[0] + s[1]))
            .map(|seed| {
                maps.iter().fold(seed, |acc, section_maps| {
                    section_maps
                        .iter()
                        .find_map(|source_map| source_map.map_source(acc))
                        .unwrap_or(acc)
                })
            })
            .min()
            .unwrap()
            .to_string()
    }
}
