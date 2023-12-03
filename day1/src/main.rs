use aoc_derive::aoc_main;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use parse_display::FromStr;
#[allow(unused_imports)]
use utils::ParseInput;
#[allow(unused_imports)]
use utils::*;

fn sum_calibration_values<'a, P>(lines: impl Iterator<Item = &'a str>, map_to_number: P) -> usize
where
    P: Fn(&str) -> Option<&str> + Copy,
{
    lines
        .map(|line| {
            let numbers = (0..line.len())
                .map(|i| &line[i..])
                .flat_map(map_to_number)
                .collect_vec();

            [numbers.first().unwrap(), numbers.last().unwrap()].into_iter().fold_digits_to_number::<usize, _>()
        })
        .sum()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (
        sum_calibration_values(input.lines(), |s| {
            (s.chars().next()?.is_ascii_digit()).then_some(&s[..1])
        }),
        sum_calibration_values(input.lines(), |s| {
            s.chars()
                .next()?
                .is_ascii_digit()
                .then_some(&s[..1])
                .or_else(|| {
                    [
                        ("one", "1"),
                        ("two", "2"),
                        ("three", "3"),
                        ("four", "4"),
                        ("five", "5"),
                        ("six", "6"),
                        ("seven", "7"),
                        ("eight", "8"),
                        ("nine", "9"),
                    ]
                    .into_iter()
                    .find_map(|(dig_str, dig)| s.starts_with(dig_str).then_some(dig))
                })
        }),
    )
}
