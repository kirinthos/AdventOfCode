use std::collections::HashSet;

use crate::{point::Point, Problem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Animal,
}

impl Square {
    fn available_spaces(&self) -> Vec<(i32, i32)> {
        match self {
            Square::Vertical => [(0, -1), (0, 1)].into(),
            Square::Horizontal => [(-1, 0), (1, 0)].into(),
            Square::BendNE => [(0, -1), (1, 0)].into(),
            Square::BendNW => [(0, -1), (-1, 0)].into(),
            Square::BendSW => [(0, 1), (-1, 0)].into(),
            Square::BendSE => [(0, 1), (1, 0)].into(),
            Square::Ground => vec![],
            Square::Animal => [(-1, 0), (1, 0), (0, -1), (0, 1)].into(),
        }
    }
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        match value {
            '|' => Square::Vertical,
            '-' => Square::Horizontal,
            'L' => Square::BendNE,
            'J' => Square::BendNW,
            '7' => Square::BendSW,
            'F' => Square::BendSE,
            '.' => Square::Ground,
            'S' => Square::Animal,
            _ => panic!("invalid map square"),
        }
    }
}

impl From<Square> for char {
    fn from(value: Square) -> Self {
        match value {
            Square::Vertical => '|',
            Square::Horizontal => '-',
            Square::BendNE => 'L',
            Square::BendNW => 'J',
            Square::BendSW => '7',
            Square::BendSE => 'F',
            Square::Ground => '.',
            Square::Animal => 'S',
        }
    }
}

fn parse_lines(lines: &[String]) -> Vec<Vec<Square>> {
    lines
        .iter()
        .map(|l| l.chars().map(From::from).collect())
        .collect()
}

fn find_pipe_loop(grid: &Vec<Vec<Square>>) -> Vec<Point> {
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, s)| matches!(s, Square::Animal).then_some((x, y)))
        })
        .unwrap();

    let mut moves: Vec<Point> = vec![start.into()];
    while moves.len() == 1 || moves.first().unwrap() != moves.last().unwrap() {
        let pos = moves.last().unwrap();
        moves.push(
            grid[pos.y() as usize][pos.x() as usize]
                .available_spaces()
                .into_iter()
                .map(|n| {
                    let n: Point = n.into();
                    *pos + n
                })
                .find(|next| {
                    if !(next.x() >= 0
                        && next.y() >= 0
                        && next.x() < grid[0].len() as i32
                        && next.y() < grid.len() as i32)
                    {
                        return false;
                    }

                    if moves.len() == 1 {
                        let piece = grid[next.y() as usize][next.x() as usize];
                        let d = *next - *pos;
                        match (d.x(), d.y()) {
                            (1, 0) => matches!(
                                piece,
                                Square::Horizontal | Square::BendSW | Square::BendNW
                            ),
                            (-1, 0) => matches!(
                                piece,
                                Square::Horizontal | Square::BendSE | Square::BendNE
                            ),
                            (0, 1) => {
                                matches!(piece, Square::Vertical | Square::BendNE | Square::BendNW)
                            }
                            (0, -1) => {
                                matches!(piece, Square::Vertical | Square::BendSE | Square::BendSW)
                            }
                            _ => panic!("illegal move"),
                        }
                    } else {
                        moves[moves.len() - 2] != *next
                    }
                })
                .unwrap(),
        );
    }

    moves
}

fn is_inside(grid: &[Vec<Square>], moves: &HashSet<Point>, p: Point) -> bool {
    if moves.contains(&p) {
        return false;
    }

    let y = p.y() as usize;
    let mut count = 0;
    let mut opening_piece = None;
    for x in 0..p.x() {
        if !moves.contains(&(x, p.y()).into()) {
            continue;
        }

        let piece = grid[y][x as usize];
        match piece {
            Square::Vertical => count += 1,
            Square::BendNE if opening_piece.is_none() => opening_piece = Some(piece),
            Square::BendSE if opening_piece.is_none() => opening_piece = Some(piece),
            Square::BendNW => {
                if matches!(opening_piece, Some(Square::BendSE)) {
                    count += 1;
                }
                opening_piece = None;
            }
            Square::BendSW => {
                if matches!(opening_piece, Some(Square::BendNE)) {
                    count += 1;
                }
                opening_piece = None;
            }
            Square::Animal => panic!("please turn animal into pipe"),
            _ => {}
        }
    }

    count % 2 == 1
}

pub struct Problem10;
impl Problem for Problem10 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let grid = parse_lines(lines);
        let moves = find_pipe_loop(&grid);

        ((moves.len() - 1) / 2).to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let mut grid = parse_lines(lines);
        let moves = find_pipe_loop(&grid);
        let (d1, d2) = (moves[1] - moves[0], moves[moves.len() - 2] - moves[0]);

        grid[moves[0].y() as usize][moves[0].x() as usize] =
            match ((d1.x(), d1.y()), (d2.x(), d2.y())) {
                ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => Square::BendNE,
                ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => Square::BendSE,
                ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => Square::BendSW,
                ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => Square::BendNW,
                ((-1, 0), (1, 0)) | ((1, 0), (-1, 0)) => Square::Horizontal,
                ((0, -1), (0, 1)) | ((0, 1), (0, -1)) => Square::Vertical,
                _ => panic!("logic error"),
            };

        let moves: HashSet<Point> = moves.into_iter().collect();

        /*
        for (y, row) in grid.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                print!(
                    "{}",
                    match is_inside(&grid, &moves, (x, y).into()) {
                        true => 'I',
                        false if moves.contains(&(x, y).into()) => col.into(),
                        _ => '.',
                    }
                );
            }
            println!();
        }
        */

        grid.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, _col)| is_inside(&grid, &moves, (*x, y).into()))
                    .count() as u64
            })
            .sum::<u64>()
            .to_string()
    }
}

/* Wanted to do "clockwise winding", but it worked for all examples, not my puzzle input
 * couldn't find the issue with it, though...so trying something else
 * oh...after i got the answer with my solution above, this solution below was off by 1?
 * I wonder if it's the fucking start pipe?
fn solve_part2(&mut self, lines: &[String]) -> String {
    let grid = parse_lines(lines);
    let mut moves = find_pipe_loop(&grid);
    let (p1, p2) = (moves[1] - moves[0], moves[moves.len() - 2] - moves[0]);
    match ((p1.x(), p1.y()), (p2.x(), p2.y())) {
        ((0, 1), (-1, 0)) | ((1, 0), (0, 1)) | ((0, -1), (1, 0)) | ((-1, 0), (0, -1)) => {
            println!("reverse!");
            moves.reverse()
        }

        _ => {}
    };
    moves.reverse();
    let moves_set: HashSet<_> = moves.iter().cloned().collect();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    // for any point in moves, look "to the right" of that piece's flow direction
    for (prev, m) in moves.iter().tuple_windows() {
        let d = *m - *prev;
        visit(&grid, &moves_set, *m + (-d.y(), d.x()), &mut visited);
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            print!(
                "{}",
                match (visited[y][x], moves_set.contains(&(x, y).into())) {
                    (true, _) => 'I',
                    (_, true) => col.into(),
                    (false, false) => '.',
                }
            );
        }
        println!();
    }

    visited
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|v| match v {
                    true => 1,
                    false => 0,
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}


fn visit(grid: &[Vec<Square>], moves: &HashSet<Point>, p: Point, visited: &mut [Vec<bool>]) {
    let (x, y) = (p.x() as usize, p.y() as usize);
    if p.x() < 0 || p.y() < 0 || x >= grid[0].len() || y >= grid.len() {
        return;
    }

    if visited[y][x] || moves.contains(&p) {
        return;
    }

    visited[y][x] = true;
    for d in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
        visit(grid, moves, p + d, visited);
    }
}
*/
