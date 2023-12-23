use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

use aoc_derive::aoc_main;
use itertools::Itertools;
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Node {
    edges: HashMap<Vec2D, usize>,
    visited: bool,
}

fn find_longest_hike_part2(
    cur_pos: Vec2D,
    cur_distance: usize,
    nodes: &mut HashMap<Vec2D, Node>,
    target: Vec2D,
) -> Option<usize> {
    if cur_pos == target {
        return Some(cur_distance);
    }

    let mut max = None;
    for (pos, dist) in nodes[&cur_pos].edges.clone().iter()
        .filter(|(next, _)| !nodes[next].visited)
        .sorted_by_key(|(_, &dist)| dist) {
        nodes.get_mut(pos).unwrap().visited = true;
        max = max.max(find_longest_hike_part2(*pos, cur_distance + dist, nodes, target));
        nodes.get_mut(pos).unwrap().visited = false;
    }
    max
}

fn find_nodes(
    cur_pos: Vec2D,
    prev_node: Vec2D,
    prev_pos: Vec2D,
    distance: usize,
    nodes: &mut HashMap<Vec2D, Node>,
    grid: &Grid<char>,
) {
    if distance == 0 {
        nodes.insert(cur_pos, Node::default());
    }

    let neighbors = cur_pos
        .orthogonal_neighbors()
        .filter(|n| *n != prev_pos && !matches!(grid.get(*n), None | Some('#')))
        .collect_vec();

    if neighbors.len() == 1 {
        find_nodes(neighbors[0], prev_node, cur_pos, distance + 1, nodes, grid);
    } else {
        let existing_entry = nodes.entry(cur_pos);

        let node_already_visited =
            matches!(existing_entry, std::collections::hash_map::Entry::Occupied(_));

        existing_entry.or_default().borrow_mut().edges.insert(prev_node, distance);

        nodes.get_mut(&prev_node).unwrap().edges.insert(cur_pos, distance);

        if !node_already_visited {
            for neighbor in neighbors {
                find_nodes(neighbor, cur_pos, cur_pos, 1, nodes, grid);
            }
        }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let grid = input.char_grid();
    dbg!(grid.num_rows(), grid.num_cols());

    let start = grid.row(0).find_map(|(pos, c)| (*c == '.').then_some(pos)).unwrap();
    let part1 = find_longest_hike_part1(start, 0, HashSet::new(), &grid);

    let mut nodes = HashMap::new();
    find_nodes(start, start, start, 0, &mut nodes, &grid);

    let target = nodes
        .iter()
        .find_map(|(&pos, _)| (pos.y as usize == grid.num_rows() - 1).then_some(pos))
        .unwrap();

    nodes.get_mut(&start).unwrap().visited = true;
    let part2 = find_longest_hike_part2(start, 0, &mut nodes, target);

    // TODO we can implement From<Option> for solution  and unwrap
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
            94,
            154
        );
    }
}
