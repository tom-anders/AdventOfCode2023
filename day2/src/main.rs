use aoc_derive::{aoc_main, CollectFromStr, HashMapFromStr};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use parse_display::FromStr;
#[allow(unused_imports)]
use utils::ParseInput;
#[allow(unused_imports)]
use utils::*;

use std::collections::HashMap;

#[derive(Debug, FromStr, PartialEq, Eq, Hash)]
#[display(style = "snake_case")]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, HashMapFromStr)]
#[sep = ","]
#[inner_sep = " "]
#[reverse]
struct Reveal(HashMap<Color, u32>);

impl Reveal {
    fn get(&self, color: &Color) -> u32 {
        *self.0.get(color).unwrap_or(&0)
    }
}

#[derive(Debug, derive_more::Deref, CollectFromStr)]
#[sep = ";"]
struct Reveals(Vec<Reveal>);

#[derive(Debug, FromStr)]
#[display("Game {id}: {reveals}")]
struct Game {
    id: u32,
    reveals: Reveals,
}

impl Game {
    pub fn possible(&self) -> bool {
        use Color::*;
        self.reveals
            .iter()
            .all(|r| r.get(&Red) <= 12 && r.get(&Green) <= 13 && r.get(&Blue) <= 14)
    }

    pub fn max(&self, color: Color) -> u32 {
        self.reveals.iter().map(|r| r.get(&color)).max().unwrap()
    }

    pub fn power(&self) -> u32 {
        use Color::*;
        self.max(Red) * self.max(Blue) * self.max(Green)
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let games = Game::parse_lines(&input);

    (
        games
            .clone()
            .filter_map(|g| g.possible().then_some(g.id))
            .sum_u64(),
        games.map(|g| g.power()).sum_u64(),
    )
}
