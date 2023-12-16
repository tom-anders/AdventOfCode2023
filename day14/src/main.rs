#![feature(let_chains)]
use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::grid::Grid;
use utils::math::Vec2D;
use utils::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, derive_more::Deref, derive_more::DerefMut, derive_more::From)]
struct Platform(Grid<char>);

impl Platform {
    fn move_rocks(&mut self, dir: (i64, i64)) {
        let positions_to_check = match dir {
            (0, -1) => self.coordinates_row_major().collect_vec(),
            (0, 1) => self.coordinates_row_major().rev().collect_vec(),
            (1, 0) => self.coordinates_col_major().rev().collect_vec(),
            (-1, 0) => self.coordinates_col_major().collect_vec(),
            _ => unreachable!(),
        };

        for pos in positions_to_check {
            self.move_rock(pos, dir.into());
        }
    }

    fn move_rock(&mut self, mut pos: Vec2D, dir: Vec2D) {
        loop {
            let next = pos + dir;
            if let Some('.') = self.get(next) && self[pos] == 'O' {
                self.swap(pos, next);
                pos = next;
            } else {
                break;
            }
        }
    }

    fn do_spin_cycle(&mut self) {
        for dir in [(0, -1), (-1,0), (0,1), (1,0)] {
            self.move_rocks(dir)
        }
    }

    fn get_load(&self) -> usize {
        self.rows()
            .rev()
            .enumerate()
            .map(|(i, row)| (i + 1) * row.filter(|(_, &c)| c == 'O').count())
            .sum()
    }

    fn find_cycle(&self) -> (usize, usize) {
        let mut grid = self.clone();
        let mut visited = HashMap::new();
        for i in 0.. {
            grid.do_spin_cycle();

            if let Some(&prev) = visited.get(&grid) {
                return (prev, i - prev);
            }

            visited.insert(grid.clone(), i);
        }
        unreachable!()
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let mut grid_part1: Platform = input.char_grid().into();
    grid_part1.move_rocks((0, -1));

    let mut grid_part2 = grid_part1.clone();
    let (cycle_start, cycle_period) = grid_part2.find_cycle();
    for _ in 0..(cycle_start + (1_000_000_000 - cycle_start) % cycle_period) {
        grid_part2.do_spin_cycle();
    }

    (grid_part1.get_load(), grid_part2.get_load())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
                "#,
            136,
            64
        );
    }
}
