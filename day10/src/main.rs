use aoc_derive::aoc_main;
use itertools::{FoldWhile, Itertools};
use num::Integer;
use utils::math::{Box2D, Vec2D};
use utils::sparse_grid::SparseGrid;
use utils::*;

fn try_find_loop(
    start: Vec2D,
    start_heading: Vec2D,
    grid: &SparseGrid<char>,
) -> Option<SparseGrid<char>> {
    let mut pos = start;
    let mut heading = start_heading;
    let mut main_loop = SparseGrid::new();
    loop {
        main_loop.insert(pos, *grid.get(pos).unwrap());
        pos += heading;

        if pos == start {
            return Some(main_loop);
        }

        heading = match (heading.xy_tuple(), grid.get(pos)?) {
            ((1, 0) | (-1, 0), '-') => heading.xy_tuple(),
            ((0, 1) | (0, -1), '|') => heading.xy_tuple(),

            ((-1, 0), 'L') => (0, -1),
            ((0, 1), 'L') => (1, 0),

            ((1, 0), 'J') => (0, -1),
            ((0, 1), 'J') => (-1, 0),

            ((0, -1), '7') => (-1, 0),
            ((1, 0), '7') => (0, 1),

            ((0, -1), 'F') => (1, 0),
            ((-1, 0), 'F') => (0, 1),
            _ => return None,
        }
        .into();
    }
}

fn is_inside_loop(point: Vec2D, bounds: &Box2D, main_loop: &SparseGrid<char>) -> bool {
    if main_loop.contains_key(&point) {
        return false;
    }

    (point.y..)
        .fold_while(0, |num_crossings, y| {
            if y > bounds.upper().y {
                return FoldWhile::Done(num_crossings);
            }

            FoldWhile::Continue(
                num_crossings
                    + match main_loop.get((point.x, y)) {
                        None => 0,
                        Some('7' | '|' | 'J') => 0, // no crossing, "squeezing through" the loop
                        Some(_) => 1,
                    },
            )
        })
        .into_inner()
        .is_odd()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_sparse_grid();

    let start = grid.iter().find_map(|(&pos, &c)| (c == 'S').then_some(pos)).unwrap();

    let main_loop = [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .find_map(|pos| try_find_loop(start, pos.into(), &grid))
        .expect("Did not find loop!");

    let loop_bounds: Box2D = main_loop.iter().map(|(pos, _)| *pos).collect();

    let part2 =
        loop_bounds.points_inside().filter(|&p| is_inside_loop(p, &loop_bounds, &main_loop)).count();

    (main_loop.len() / 2, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_part2!(
            r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
            "#,
            4
        );
        assert_part2!(
            r#"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
            "#,
            4
        );
        assert_example!(
            r#".....
.S-7.
.|.|.
.L-J.
.....
"#,
            4
        );
        assert_example!(
            r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#,
            4
        );
        assert_example!(
            r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
                "#,
            8
        );
    }
}
