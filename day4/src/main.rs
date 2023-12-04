use std::collections::HashSet;
use std::str::FromStr;

use aoc_derive::aoc_main;
use itertools::Itertools;
use regex::Regex;
use utils::*;

#[derive(Debug, derive_more::Deref, Clone)]
struct CardSet(HashSet<u64>);

impl FromStr for CardSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            Regex::new(r"\d+")
                .unwrap()
                .find_iter(s)
                .map(|m| m.as_str().parse().unwrap())
                .collect(),
        ))
    }
}

#[derive(Debug, Clone)]
struct Card {
    winning: CardSet,
    mine: CardSet,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s.split(':').last().unwrap();

        let (winning, mine) = Regex::new(r"(.*) \| (.*)")
            .unwrap()
            .captures(games)
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str())
            .collect_tuple().unwrap();

        Ok(Self {
            winning: winning.parse().unwrap(),
            mine: mine.parse().unwrap(),
        })
    }
}

impl Card {
    fn num_matches(&self) -> usize {
        self.winning.intersection(&self.mine).collect_vec().len()
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let cards = Card::parse_lines(&input).collect_vec();

    let part1 = cards
        .iter()
        .map(|card| {
            let matches = card.num_matches();
            if matches > 0 {
                2_usize.pow(matches as u32 - 1)
            } else {
                0
            }
        })
        .sum_u64();

    let mut cards_part2 = cards.iter().map(|c| (c, 1)).collect_vec();

    for i in 0..cards_part2.len() {
        let (card, amount) = cards_part2[i];
        let wins = card.num_matches();
        for j in 0..wins {
            cards_part2[i + j + 1].1 += amount;
        }
    }

    (part1, cards_part2.iter().map(|(_, n)| *n).sum_u64())
}
