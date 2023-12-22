use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::ops::RangeInclusive;
use std::str::FromStr;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::ParseInput;
use utils::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

impl Brick {
    fn x_and_y_overlap(&self, other: &Brick) -> bool {
        let no_overlap = (self.x.end() < other.x.start() || self.x.start() > other.x.end())
            || (self.y.end() < other.y.start() || self.y.start() > other.y.end());
        !no_overlap
    }
}

impl FromStr for Brick {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_min, y_min, z_min, x_max, y_max, z_max) =
            extract_numbers(s).collect_tuple().unwrap();
        Ok(Self { x: x_min..=x_max, y: y_min..=y_max, z: z_min..=z_max })
    }
}

fn disintegrate_brick(
    mut bricks: HashMap<usize, (Brick, HashSet<usize>)>,
    start_brick: usize,
) -> usize {
    let mut next_to_remove = HashSet::from([start_brick]);
    let mut total_removed = 0;
    loop {
        for to_remove in &next_to_remove {
            bricks.remove(to_remove);
        }

        next_to_remove = bricks
            .iter()
            .filter_map(|(name, (brick, supporting))| {
                (brick.z.start() != &1 // supported by the ground
                    &&
                    supporting.iter().all(|s| next_to_remove.contains(s)))
                .then_some(*name)
            })
            .collect();

        for (_, supporting) in bricks.values_mut() {
            for to_remove in &next_to_remove {
                supporting.remove(to_remove);
            }
        }

        total_removed += next_to_remove.len();

        if next_to_remove.is_empty() {
            return total_removed;
        }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let mut bricks = Brick::parse_lines(&input).enumerate().collect_vec();

    loop {
        let movable_bricks = bricks
            .iter()
            .enumerate()
            .filter_map(|(brick_index, (_, brick_to_check))| {
                if brick_to_check.z.start() == &1 {
                    return None;
                }

                let blocking_brick = bricks.iter().find(|(_, brick)| {
                    brick != brick_to_check
                        && *brick.z.end() + 1 == *brick_to_check.z.start()
                        && brick.x_and_y_overlap(brick_to_check)
                });
                blocking_brick.is_none().then_some(brick_index)
            })
            .collect_vec();

        if movable_bricks.is_empty() {
            break;
        }

        for brick_index in movable_bricks.into_iter() {
            bricks[brick_index].1.z =
                *bricks[brick_index].1.z.start() - 1..=*bricks[brick_index].1.z.end() - 1
        }
    }

    let bricks_with_supporting_bricks: HashMap<usize, _> = bricks
        .iter()
        .map(|(name, brick_to_check)| {
            let supporting_bricks: HashSet<_> = bricks
                .iter()
                .filter_map(|(brick_name, brick)| {
                    (brick != brick_to_check
                        && *brick.z.end() + 1 == *brick_to_check.z.start()
                        && brick.x_and_y_overlap(brick_to_check))
                    .then_some(*brick_name)
                })
                .collect();
            (*name, (brick_to_check.clone(), supporting_bricks))
        })
        .collect();

    let bricks_that_can_be_removed = bricks
        .iter()
        .filter(|(name, _)| {
            !bricks_with_supporting_bricks.iter().any(|(_, (_, supporting_bricks))| {
                supporting_bricks.len() == 1 && supporting_bricks.contains(name)
            })
        })
        .collect_vec();

    let part2 = bricks_with_supporting_bricks
        .keys()
        .map(|&name| disintegrate_brick(bricks_with_supporting_bricks.clone(), name))
        .sum_usize();

    (bricks_that_can_be_removed.len(), part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
                "#,
            5,
            7
        );
    }
}
