use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let mut grid = input.char_grid();

    // TODO add a add_padding method to our grid
    for row in 0..grid.num_rows() {
        grid.inner_mut()[row].push('.');
    }
    let grid = grid;

    let numbers = grid
        .rows()
        .map(|row| {
            row.fold((vec![], vec![]), |(mut numbers, mut current), (pos, c)| {
                if c.is_ascii_digit() {
                    current.push((pos, c));
                } else if !current.is_empty() {
                    numbers.push(current);
                    current = vec![];
                }
                (numbers, current)
            })
        })
        .flat_map(|row| row.0)
        .collect_vec();

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
                .fold("".to_string(), |acc, (_, n)| acc + &n.to_string())
                .parse::<u64>()
                .unwrap()
        })
        .sum_u64();

    (part1)
}
