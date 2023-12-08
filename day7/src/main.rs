use std::collections::BTreeMap;
use std::convert::Infallible;
use std::str::FromStr;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone, Copy, parse_display::FromStr, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    JPart2,
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
struct Hand {
    ty: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split(' ').collect_tuple().unwrap();

        let cards = cards.chars().map(|c| c.to_string().parse().unwrap()).collect_vec();

        Ok(Self::new(cards.try_into().unwrap(), bid.parse().unwrap()))
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        let mut card_counts = cards.into_iter().fold(BTreeMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });

        // Since there are no straights and no colors, we always want the joker (in part2) to turn
        // into the card we already have the most of. If there are multiple cards with the same
        // amount, we'll want the joker to be the highest card of them
        if let Some((&current_best_card, _)) = card_counts
            .iter()
            .filter(|(&card, _)| card != Card::JPart2)
            // First sort by amount, then break ties by card type
            .max_by_key(|(&card, &v)| (v, card))
        {
            let num_jokers = *card_counts.get(&Card::JPart2).unwrap_or(&0);
            *card_counts.get_mut(&current_best_card).unwrap() += num_jokers;
            card_counts.remove(&Card::JPart2);
        } else {
            // all cards are jokers - turn them into aces
            card_counts = [(Card::A, 5)].into_iter().collect();
        }

        use HandType::*;
        let ty = match card_counts.len() {
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
        };

        Self { ty, cards, bid }
    }

    fn into_part2(self) -> Self {
        let mut new_cards = self.cards;
        for card in &mut new_cards {
            if *card == Card::J {
                *card = Card::JPart2;
            }
        }
        Self::new(new_cards, self.bid)
    }
}

#[derive(Debug)]
struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    fn total_winnings(&self) -> usize {
        self.hands.iter().sorted().enumerate().map(|(rank, hand)| (rank + 1) * hand.bid).sum()
    }

    fn into_part2(self) -> Self {
        let hands = self.hands.into_iter()
            .map(|hand| hand.into_part2())
            .collect_vec();
        Self { hands }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let hands = Hands { hands: input.lines().map(|line| line.parse().unwrap()).collect() };

    (hands.total_winnings(), hands.into_part2().total_winnings())
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
