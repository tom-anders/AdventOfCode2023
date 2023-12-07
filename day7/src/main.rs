use std::collections::{BTreeMap, HashMap};
use std::convert::Infallible;
use std::str::FromStr;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::ParseInput;
use utils::*;

#[derive(Debug, Clone, Copy, parse_display::FromStr, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart1 {
    #[display("2")]
    _2,
    #[display("3")]
    _3,
    #[display("4")]
    _4,
    #[display("5")]
    _5,
    #[display("6")]
    _6,
    #[display("7")]
    _7,
    #[display("8")]
    _8,
    #[display("9")]
    _9,
    #[display("T")]
    _10,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, parse_display::FromStr, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart2 {
    J,
    #[display("2")]
    _2,
    #[display("3")]
    _3,
    #[display("4")]
    _4,
    #[display("5")]
    _5,
    #[display("6")]
    _6,
    #[display("7")]
    _7,
    #[display("8")]
    _8,
    #[display("9")]
    _9,
    #[display("T")]
    _10,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HandPart1 {
    ty: HandType,
    cards: [CardPart1; 5],
    bid: usize,
}

fn hand_type<C>(card_counts: BTreeMap<C, usize>) -> HandType {
    use HandType::*;
    match card_counts.len() {
        1 => FiveOAK,
        2 => {
            if card_counts.values().any(|&v| v == 4) {
                FourOAK
            } else {
                FullHouse
            }
        }
        3 => {
            if card_counts.values().any(|&v| v == 3) {
                ThreeOAK
            } else {
                TwoPair
            }
        }
        4 => OnePair,
        5 => HighCard,
        _ => unreachable!(),
    }
}

impl FromStr for HandPart1 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split(' ').collect_tuple().unwrap();

        let cards = cards.chars().map(|c| c.to_string().parse().unwrap()).collect_vec();

        let card_counts = cards.clone().into_iter().fold(BTreeMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });

        Ok(Self {
            ty: hand_type(card_counts),
            cards: cards.try_into().unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HandPart2 {
    ty: HandType,
    cards: [CardPart2; 5],
    bid: usize,
}

impl FromStr for HandPart2 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split(' ').collect_tuple().unwrap();

        let cards: Vec<CardPart2> = cards.chars().map(|c| c.to_string().parse().unwrap()).collect();

        let mut card_counts = cards.clone().into_iter().fold(BTreeMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });

        // Since there are no straights and no colors, we always want the joker to turn
        // into the card we already have the most of.
        // If there are multiple cards with the same amount, we'll want the joker
        // to be the highest card of them
        if let Some((&current_best_card, _)) = card_counts
            .iter()
            .filter(|(&card, _)| card != CardPart2::J)
            // First sort by amount, then break ties by card type
            .max_by_key(|(&card, &v)| (v, card))
        {
            let num_jokers = *card_counts.get(&CardPart2::J).unwrap_or(&0);
            *card_counts.get_mut(&current_best_card).unwrap() += num_jokers;
            card_counts.remove(&CardPart2::J);
        } else {
            // all cards are jokers - turn them into aces
            card_counts = [(CardPart2::A, 5)].into_iter().collect();
        }

        Ok(Self {
            ty: hand_type(card_counts),
            cards: cards.try_into().unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let part1: usize = HandPart1::parse_lines(&input)
        .sorted()
        .collect_vec()
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    let part2: usize = HandPart2::parse_lines(&input)
        .sorted()
        .collect_vec()
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::assert_example;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
                "#,
            6440,
            5905
        );
    }
}
