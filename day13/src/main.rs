use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::grid::Grid;
use utils::*;

#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
struct Pattern(Grid<char>);

impl Pattern {
    fn find_reflection(&self, ignore: Option<usize>) -> Option<usize> {
        for refl_col in 0..self.num_cols() - 1 {
            if (0..=refl_col.min(self.num_cols() / 2)).all(|test_col| {
                let (left, right) = (refl_col - test_col, refl_col + test_col + 1);
                if right >= self.num_cols() {
                    return true;
                }
                self.col_values(left ).eq(self.col_values(right ))
            }) {
                let score = refl_col + 1;

                if ignore.is_none() || ignore.is_some_and(|ignore| ignore != score) {
                    return Some(score);
                }
            }
        }
        for refl_row in 0..self.num_rows() - 1 {
            if (0..=refl_row.min(self.num_rows() / 2)).all(|test_row| {
                let (left, right) = (refl_row - test_row, refl_row + test_row + 1);
                if right >= self.num_rows() {
                    return true;
                }
                self.row_values(left ).eq(self.row_values(right ))
            }) {
                let score = 100 * (refl_row + 1);

                if ignore.is_none() || ignore.is_some_and(|ignore| ignore != score) {
                    return Some(score);
                }
            }
        }

        None
    }

    fn brute_force_smudge(&self) -> usize {
        let prev_refl = self.find_reflection(None).unwrap();
        for (pos, c) in self.iter() {
            let mut new_grid = self.clone();
            new_grid[pos] = if *c == '#' { '.' } else { '#' };
            if let Some(reflection) = new_grid.find_reflection(Some(prev_refl)) {
                if reflection != prev_refl {
                    return reflection;
                }
            }
        }
        panic!("no new reflection found!");
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grids = input
        .lines()
        .collect_vec()
        .split(|line| line.is_empty())
        .map(|lines| {
            Pattern(Grid::new(lines.iter().map(|line| line.chars().collect_vec()).collect_vec()))
        })
        .collect_vec();

    (
        grids.iter().map(|g| g.find_reflection(None).unwrap()).sum::<usize>(),
        grids.iter().map(|g| g.brute_force_smudge()).sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
                "#,
            405,
            400
        );
    }
}
