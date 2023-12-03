use crate::Problem;

pub struct Problem1;
impl Problem for Problem1 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        lines
            .iter()
            .map(|l| {
                let (first, last): (Option<u32>, Option<u32>) = l
                    .chars()
                    .filter(char::is_ascii_digit)
                    .map(|c| c.to_digit(10).expect("digit"))
                    .fold((None, None), |acc, n| match acc {
                        (None, _) => (Some(n), Some(n)),
                        (a @ Some(_), _) => (a, Some(n)),
                    });
                first.unwrap() * 10 + last.unwrap()
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let map = [
            ("one", 1),
            ("1", 1),
            ("two", 2),
            ("2", 2),
            ("three", 3),
            ("3", 3),
            ("four", 4),
            ("4", 4),
            ("five", 5),
            ("5", 5),
            ("six", 6),
            ("6", 6),
            ("seven", 7),
            ("7", 7),
            ("eight", 8),
            ("8", 8),
            ("nine", 9),
            ("9", 9),
        ];
        lines
            .iter()
            .map(|l| {
                let first = map
                    .iter()
                    .filter_map(|(p, n)| l.find(p).map(|index| (index, n)))
                    .min_by_key(|v| v.0)
                    .unwrap()
                    .1;
                let last = map
                    .iter()
                    .filter_map(|(p, n)| l.rfind(p).map(|index| (index, n)))
                    .max_by_key(|v| v.0)
                    .unwrap()
                    .1;
                first * 10 + last
            })
            .sum::<u32>()
            .to_string()
    }
}
