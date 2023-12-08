use crate::Problem;

struct Race {
    time: u64,
    distance: u64,
}

fn parse_lines(lines: &[String]) -> Vec<Race> {
    let time = lines[0]
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|w| w.parse::<u64>().unwrap());
    let distance = lines[1]
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .map(|w| w.parse::<u64>().unwrap());

    time.zip(distance)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn solve(a: f64, b: f64, c: f64) -> (u64, u64) {
    let bn = -1_f64 * b;
    let bsq = b.powi(2);
    let ac = 4_f64 * a * c;
    let a2 = 2_f64 * a;
    // b is positive in this problem, so "-" ends up being the bigger one
    (
        ((bn + (bsq - ac).sqrt()) / a2).ceil() as u64,
        ((bn - (bsq - ac).sqrt()) / a2).floor() as u64,
    )
}

pub struct Problem6;
impl Problem for Problem6 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        parse_lines(lines)
            .into_iter()
            .map(|Race { time, distance }| {
                // -x^2 + t * x - (d + 1) >= 0
                let (one, two) = solve(-1_f64, time as f64, -1_f64 * ((distance + 1) as f64));
                two - one + 1
            })
            .product::<u64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let time = lines[0]
            .strip_prefix("Time: ")
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let distance = lines[1]
            .strip_prefix("Distance: ")
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        let (one, two) = solve(-1_f64, time as f64, -1_f64 * ((distance + 1) as f64));
        (two - one + 1).to_string()
    }
}
