use std::collections::HashSet;

use aoc_derive::aoc_main;
use utils::{grid::Grid, math::Vec2D, *};

fn find_longest_hike_part1(
    pos: Vec2D,
    distance: usize,
    mut visited: HashSet<Vec2D>,
    grid: &Grid<char>,
) -> Option<usize> {
    if pos.y as usize == grid.num_rows() - 1 {
        return Some(distance);
    }
    if visited.contains(&pos) {
        return None;
    }
    visited.insert(pos);

    pos.orthogonal_neighbors()
        .filter(|&neighbor| match grid.get(neighbor) {
            None | Some('#') => false,
            Some('.') => true,
            Some('v') => (neighbor - pos) == (0, 1),
            Some('^') => (neighbor - pos) == (0, -1),
            Some('<') => (neighbor - pos) == (-1, 0),
            Some('>') => (neighbor - pos) == (1, 0),
            _ => unreachable!(),
        })
        .flat_map(|neighbor| find_longest_hike_part1(neighbor, distance + 1, visited.clone(), grid))
        .max()
}

fn find_longest_hike_part2(
    pos: Vec2D,
    distance: usize,
    mut visited: HashSet<Vec2D>,
    grid: &Grid<char>,
) -> Option<usize> {
    if pos.y as usize == grid.num_rows() - 1 {
        println!("Found hike: {distance:?}");
        return Some(distance);
    }
    if visited.contains(&pos) {
        return None;
    }
    visited.insert(pos);

    pos.orthogonal_neighbors()
        .filter(|&neighbor| match grid.get(neighbor) {
            None | Some('#') => false,
            Some(_) => true,
        })
        .flat_map(|neighbor| find_longest_hike_part2(neighbor, distance + 1, visited.clone(), grid))
        .max()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_grid();

    let part1 = find_longest_hike_part1(
        grid.row(0).find_map(|(pos, c)| (*c == '.').then_some(pos)).unwrap(),
        0,
        HashSet::new(),
        &grid,
    );

    let part2 = find_longest_hike_part2(
        grid.row(0).find_map(|(pos, c)| (*c == '.').then_some(pos)).unwrap(),
        0,
        HashSet::new(),
        &grid,
    );

    (part1.unwrap(), part2.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
                "#,
                94, 154
        );
    }
}
