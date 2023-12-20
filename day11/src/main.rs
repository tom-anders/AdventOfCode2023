use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::grid::Grid;
use utils::*;

#[derive(Debug, derive_more::Constructor)]
struct Universe {
    galaxies: Grid<char>,
    grow_factor: i64,
}

impl Universe {
    pub fn solve(&self) -> i64 {
        let empty_cols = self
            .galaxies
            .cols()
            .enumerate()
            .filter_map(|(i, mut col)| col.all(|(_, &c)| c == '.').then_some(i))
            .collect_vec();

        let empty_rows = self
            .galaxies
            .rows()
            .enumerate()
            .filter_map(|(i, mut col)| col.all(|(_, &c)| c == '.').then_some(i))
            .collect_vec();

        self.galaxies
            .iter()
            .filter_map(|(pos, &c)| (c == '#').then_some(pos))
            .collect_vec()
            .into_iter()
            .tuple_combinations()
            .map(|(start, end)| {
                let (min_x, max_x) = if start.x < end.x { (start, end) } else { (end, start) };
                let (min_y, max_y) = if start.y < end.y { (start, end) } else { (end, start) };
                let dist = (end - start).manhattan_dist()
                    + (self.grow_factor - 1).max(1)
                        * empty_cols
                            .iter()
                            .filter(|&&c| c < max_x.x as usize && c > min_x.x as usize)
                            .count() as i64
                    + (self.grow_factor - 1).max(1)
                        * empty_rows
                            .iter()
                            .filter(|&&r| r < max_y.y as usize && r > min_y.y as usize)
                            .count() as i64;
                dist
            })
            .sum()
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (
        Universe::new(input.char_grid(), 1).solve(),
        Universe::new(input.char_grid(), 1_000_000).solve(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
            let example =
            Input::from(r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
                "#);
            assert_eq!(Universe::new(example.char_grid(), 1).solve(), 374);
            assert_eq!(Universe::new(example.char_grid(), 10).solve(), 1030);
            assert_eq!(Universe::new(example.char_grid(), 100).solve(), 8410);
    }
}
