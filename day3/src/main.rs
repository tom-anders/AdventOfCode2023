use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_grid().pad_edges('.');

    let numbers = grid
        .rows()
        .map(|row| {
            row.fold((vec![], vec![]), |(mut numbers, mut current), (pos, c)| {
                if c.is_ascii_digit() {
                    current.push((pos, *c));
                } else if !current.is_empty() {
                    numbers.push(current);
                    current = vec![];
                }
                (numbers, current)
            })
        })
        .flat_map(|row| row.0)
        .collect_vec();

    let part2 = numbers
        .clone()
        .iter()
        .filter_map(|number| {
            number
                .iter()
                .find_map(|(pos, _)| grid.all_neighbors(pos).find(|p| *grid.get(*p) == '*'))
                .map(|gear| {
                    (
                        gear,
                        number
                            .iter()
                            .map(|(_, n)| n)
                            .fold_digits_to_u64()
                    )
                })
        })
        .fold(HashMap::<_, Vec<_>>::new(), |mut map, (gear, n)| {
            map.entry(gear).or_default().push(n);
            map
        })
        .into_iter()
        .filter_map(|(_, n)| (n.len() == 2).then(|| n[0] * n[1]))
        .sum_u64();

    let part1 = numbers
        .iter()
        .filter(|number| {
            number.iter().any(|(pos, _)| {
                grid.all_neighbors(pos)
                    .any(|p| !grid.get(p).is_ascii_digit() && *grid.get(p) != '.')
            })
        })
        .map(|number| {
            number
                .iter()
                .map(|(_, n)| n)
                .fold_digits_to_u64()
        })
        .sum_u64();

    (part1, part2)
}
