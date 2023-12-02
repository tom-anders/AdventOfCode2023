use std::collections::HashMap;
use std::str::FromStr;

use aoc_derive::aoc_main;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use parse_display::FromStr;
#[allow(unused_imports)]
use utils::ParseInput;
#[allow(unused_imports)]
use utils::*;

#[derive(Debug, FromStr, PartialEq, Eq, Hash)]
#[display(style = "snake_case")]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct Reveal {
    cubes: HashMap<Color, u32>,
}

impl Reveal {
    fn get(&self, color: &Color) -> u32 {
        *self.cubes.get(color).unwrap_or(&0)
    }
}

impl FromStr for Reveal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cubes: s
                .split(',')
                .flat_map(|s| {
                    let (num, color) = s.trim().split(' ').collect_tuple()?;
                    Some((color.parse().ok()?, num.parse().ok()?))
                })
                .collect(),
        })
    }
}

#[derive(Debug, derive_more::Deref)]
struct Reveals(Vec<Reveal>);

impl FromStr for Reveals {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(';').flat_map(|r| r.parse()).collect()))
    }
}

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
            .sum::<u32>(),
        games.map(|g| g.power()).sum::<u32>(),
    )
}
