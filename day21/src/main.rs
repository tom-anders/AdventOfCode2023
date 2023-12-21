use aoc_derive::aoc_main;
use itertools::Itertools;
use std::collections::HashSet;
use utils::grid::Grid;
use utils::math::Vec2D;
use utils::*;

fn wrap_pos_to_grid(pos: &Vec2D, grid: &Grid<char>) -> Vec2D {
    let (y_max, x_max) = (grid.num_rows() as i64, grid.num_cols() as i64);
    (
        if pos.x < 0 { (pos.x + ((-pos.x / x_max) + 1) * x_max) % x_max } else { pos.x % x_max },
        if pos.y < 0 { (pos.y + ((-pos.y / y_max) + 1) * y_max) % y_max } else { pos.y % y_max },
    )
        .into()
}

fn part1(grid: &Grid<char>, max_steps: usize) -> usize {
    let mut positions =
        HashSet::from([grid.iter().find_map(|(pos, &c)| (c == 'S').then_some(pos)).unwrap()]);
    for _ in 0..max_steps {
        positions = positions
            .into_iter()
            .flat_map(|pos| {
                pos.orthogonal_neighbors()
                    .filter(|n| grid.get(*n) != Some(&'#'))
                    .collect_vec()
                    .into_iter()
            })
            .collect();
    }
    positions.len()
}

fn part2(grid: &Grid<char>, max_steps: usize) -> usize {
    let start = grid.iter().find_map(|(pos, &c)| (c == 'S').then_some(pos)).unwrap();

    let mut step = 0;
    let mut visited = HashSet::new();

    let mut next = HashSet::new();
    next.insert(start);

    let mut reachable = if max_steps % 2 == 0 { 1 } else { 0 };

    loop {
        let mut neighbors = HashSet::new();
        for node in next {
            visited.insert(node);

            for n in node.orthogonal_neighbors() {
                if grid[wrap_pos_to_grid(&n, grid)] != '#' && !visited.contains(&n) {
                    neighbors.insert(n);
                }
            }
        }

        if step % 2 != max_steps % 2 {
            reachable += neighbors.len();
        }

        if step == max_steps {
            return reachable;
        }

        next = neighbors;
        step += 1;
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_grid();

    // Throw this into wolframalpha to obtain the quadratic fit below
    for x in 0..6 {
        println!("({}, {}), ", x, part2(&grid, 65 + x * 131));
    }

    let x = 26501365_i64 / 131;

    (part1(&grid, 64), 14590 * x * x + 14694 * x + 3691)
}
