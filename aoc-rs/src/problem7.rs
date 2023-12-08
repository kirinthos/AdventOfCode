use std::{cmp::Ordering, collections::HashMap, ops::Deref};

use crate::Problem;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    score: Score,
    bid: u64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.score.partial_cmp(&other.score) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        Some(
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(one, two)| one.partial_cmp(two).unwrap())
                .find(|ord| *ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
        )
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Eq)]
struct HandPart2(Hand);
impl PartialOrd for HandPart2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.score.partial_cmp(&other.score) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        Some(
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(one, two)| match (one, two) {
                    (Card::Jack, Card::Jack) => Ordering::Equal,
                    (Card::Jack, _) => Ordering::Less,
                    (_, Card::Jack) => Ordering::Greater,
                    _ => one.partial_cmp(two).unwrap(),
                })
                .find(|ord| *ord != Ordering::Equal)
                .expect("no duplicate hands"),
        )
    }
}
impl Ord for HandPart2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for HandPart2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for HandPart2 {
    type Target = Hand;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn score_from_cards(cards: &str, part2: bool) -> Score {
    let counts = count_characters(cards);
    let mut score = match counts.len() {
        1 => Score::FiveOfAKind,
        2 => {
            let c = counts.values().max().unwrap();
            match c {
                4 => Score::FourOfAKind,
                3 => Score::FullHouse,
                _ => panic!("logic error"),
            }
        }
        3 => {
            let count = counts.values().max().unwrap();
            match count {
                3 => Score::ThreeOfAKind,
                2 => Score::TwoPair,
                _ => panic!("logic error"),
            }
        }
        4 => Score::OnePair,
        5 => Score::HighCard,
        _ => panic!("illegal hand size"),
    };

    if part2 {
        score = match (counts.get(&'J'), score) {
            (None, _) => score,

            // JWXYZ
            (Some(1), Score::HighCard) => Score::OnePair,
            // JWWXY
            (Some(1), Score::OnePair) => Score::ThreeOfAKind,
            // JWWXX
            (Some(1), Score::TwoPair) => Score::FullHouse,
            // JWWWX
            (Some(1), Score::ThreeOfAKind) => Score::FourOfAKind,
            // JWWWW
            (Some(1), Score::FourOfAKind) => Score::FiveOfAKind,

            // JJWXY
            (Some(2), Score::OnePair) => Score::ThreeOfAKind,
            // JJWWX
            (Some(2), Score::TwoPair) => Score::FourOfAKind,
            // JJWWW
            (Some(2), Score::FullHouse) => Score::FiveOfAKind,

            // JJJWX
            (Some(3), Score::ThreeOfAKind) => Score::FourOfAKind,
            // JJJWW
            (Some(3), Score::FullHouse) => Score::FiveOfAKind,

            // JJJJW
            (Some(4), Score::FourOfAKind) => Score::FiveOfAKind,
            // JJJJJ
            (Some(5), Score::FiveOfAKind) => score,

            _ => panic!("didn't account for this hand: {}", cards),
        };
    }

    score
}

fn parse_lines(lines: &[String], part2: bool) -> Vec<Hand> {
    lines
        .iter()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            let score = score_from_cards(cards, part2);
            let cards: Vec<Card> = cards.chars().map(From::from).collect();
            let bid = bid.parse().unwrap();

            Hand { cards, score, bid }
        })
        .collect()
}

fn count_characters(s: &str) -> HashMap<char, u64> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

pub struct Problem7;
impl Problem for Problem7 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let mut hands = parse_lines(lines, false);
        hands.sort();
        hands
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank as u64 + 1))
            .sum::<u64>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let mut hands = parse_lines(lines, true)
            .into_iter()
            .map(HandPart2)
            .collect::<Vec<_>>();
        hands.sort();

        for h in hands.iter() {
            println!("{:?}", h);
        }

        hands
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank as u64 + 1))
            .sum::<u64>()
            .to_string()
    }
}
