use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::regex;
use utils::*;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
struct Maze<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    instructions: Vec<char>,
}

impl Maze<'_> {
    fn next_node(&self, current_node: &str, i: usize) -> &str {
        let current_node = &self.nodes[current_node];
        if self.instructions[i % self.instructions.len()] == 'L' {
            current_node.left
        } else {
            current_node.right
        }
    }

    fn solve_part1(&self) -> usize {
        (0..)
            .scan("AAA", |current_node, i| {
                if current_node == &"ZZZ" {
                    None
                } else {
                    *current_node = self.next_node(current_node, i);
                    Some(())
                }
            })
            .count()
    }

    fn find_cycle(&self, start: &str) -> (usize, usize) {
        let mut first_target_hit = None;
        let mut current_node = start;

        (0..)
            .find_map(|i| {
                current_node = self.next_node(current_node, i);

                if current_node.ends_with('Z') {
                    if let Some(first_target_hit) = first_target_hit {
                        return Some((first_target_hit + 1, i - first_target_hit));
                    } else {
                        first_target_hit = Some(i);
                    }
                }
                None
            })
            .unwrap()
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (instructions, nodes) = input.raw.split("\n\n").collect_tuple().unwrap();

    let maze = Maze {
        nodes: nodes
            .lines()
            .map(|line| {
                let (name, left, right) =
                    regex!(r"[A-Z0-9]+").find_into_tuple(line);
                (name, Node { left, right })
            })
            .collect(),
        instructions: instructions.chars().collect(),
    };

    let nodes_part2 = maze.nodes.keys().filter(|name| name.ends_with('A')).copied().collect_vec();

    let node_times =
        nodes_part2.into_iter().map(|start_node| maze.find_cycle(start_node)).collect_vec();

    let (hit_for_max_cycle_time, max_cycle_time) =
        node_times.iter().copied().max_by_key(|(_, cycle)| *cycle).unwrap();

    let part2 = (hit_for_max_cycle_time..)
        .step_by(max_cycle_time)
        .find(|i| {
            node_times
                .iter()
                .all(|(first_hit, cycle)| i >= first_hit && (i - first_hit) % cycle == 0)
        })
        .unwrap();

    (maze.solve_part1(), part2)
}
