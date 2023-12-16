use std::collections::HashSet;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::grid::Grid;
use utils::math::Vec2D;
use utils::*;

fn count_energized_tiles(
    initial_pos: impl Into<Vec2D>,
    initial_heading: impl Into<Vec2D>,
    grid: &Grid<char>,
) -> usize {
    let mut visited = HashSet::new();
    cast_rays(initial_pos.into(), initial_heading.into(), grid, &mut visited);
    visited.iter().unique_by(|(pos, _)| pos).count()
}

fn cast_rays(pos: Vec2D, heading: Vec2D, grid: &Grid<char>, visited: &mut HashSet<(Vec2D, Vec2D)>) {
    if visited.contains(&(pos, heading)) {
        return;
    }

    let (new_heading1, maybe_new_heading2) = match (grid.get(pos), heading.xy_tuple()) {
        (None, _) => return,
        (Some('-'), (1, 0) | (-1, 0)) => (heading, None),
        (Some('|'), (0, 1) | (0, -1)) => (heading, None),

        (Some('-'), (0, 1) | (0, -1)) => (heading.rotated_left(), Some(heading.rotated_right())),
        (Some('|'), (1, 0) | (-1, 0)) => (heading.rotated_left(), Some(heading.rotated_right())),

        (Some('/'), (1, 0) | (-1, 0)) => (heading.rotated_left(), None),
        (Some('/'), (0, 1) | (0, -1)) => (heading.rotated_right(), None),

        (Some('\\'), (1, 0) | (-1, 0)) => (heading.rotated_right(), None),
        (Some('\\'), (0, 1) | (0, -1)) => (heading.rotated_left(), None),

        (Some('.'), _) => (heading, None),

        (Some(_), _) => unreachable!(),
    };

    visited.insert((pos, heading));

    cast_rays(pos + new_heading1, new_heading1, grid, visited);
    if let Some(new_heading2) = maybe_new_heading2 {
        cast_rays(pos + new_heading2, new_heading2, grid, visited);
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_grid();

    let part1 = count_energized_tiles((0, 0), (1, 0), &grid);

    let part2 = grid
        .row(0)
        .map(|(pos, _)| count_energized_tiles(pos, (0, 1), &grid))
        .chain(
            grid.rows().last().unwrap().map(|(pos, _)| count_energized_tiles(pos, (0, -1), &grid)),
        )
        .chain(grid.col(0).map(|(pos, _)| count_energized_tiles(pos, (1, 0), &grid)))
        .chain(
            grid.cols().last().unwrap().map(|(pos, _)| count_energized_tiles(pos, (-1, 0), &grid)),
        )
        .max()
        .unwrap();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
                "#,
            46,
            51
        );
    }
}
