use std::convert::Infallible;
use std::str::FromStr;

use aoc_derive::aoc_main;
use indicatif::ParallelProgressIterator;
use itertools::{iproduct, Itertools};
use ndarray::{arr1, array, s, Array};
use ndarray_linalg::Solve;
use rayon::prelude::*;
use utils::ParseInput;
use utils::*;

type Vector = ndarray::Array1<f64>;

#[derive(Debug, Clone)]
struct Path {
    pos_t0: Vector,
    vel: Vector,
}

impl Path {
    fn pos_at(&self, t: f64) -> Vector {
        self.pos_t0.clone() + t * self.vel.clone()
    }

    fn inside_test_area_at(&self, t: f64, min: f64, max: f64) -> bool {
        let pos = self.pos_at(t);
        pos[0] >= min && pos[0] <= max && pos[1] >= min && pos[1] <= max
    }
}

impl FromStr for Path {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z, vx, vy, vz) =
            extract_numbers::<i64>(s).map(|n| n as f64).collect_tuple().unwrap();
        Ok(Self { pos_t0: arr1(&[x, y, z]), vel: arr1(&[vx, vy, vz]) })
    }
}

fn find_x_y_intersection(lhs: &Path, rhs: &Path) -> Option<Vector> {
    array![[-lhs.vel[0], rhs.vel[0]], [-lhs.vel[1], rhs.vel[1]]]
        .solve(&array![lhs.pos_t0[0] - rhs.pos_t0[0], lhs.pos_t0[1] - rhs.pos_t0[1]])
        .ok()
}

fn part1(input: &Input, min: f64, max: f64) -> usize {
    Path::parse_lines(input)
        .tuple_combinations()
        .filter(|(lhs, rhs)| match find_x_y_intersection(lhs, rhs) {
            None => false,
            Some(times) => {
                times[0] > 0.0
                    && lhs.inside_test_area_at(times[0], min, max)
                    && times[1] > 0.0
                    && rhs.inside_test_area_at(times[1], min, max)
            }
        })
        .count()
}

fn check_velocity(rock_vel: &Vector, hailstones: &[Path; 3]) -> Option<Vector> {
    let len = 3 * hailstones.len();
    let mut matrix = Array::zeros((len, len));
    let mut rhs = Array::zeros(len);
    for (i, Path { pos_t0, vel }) in hailstones.iter().enumerate() {
        for j in 0..3 {
            matrix[(i * 3 + j, i)] = rock_vel[j] - vel[j];

            matrix[(i * 3 + j, hailstones.len() + j)] = 1.0;

            rhs[i * 3 + j] = pos_t0[j];
        }
    }

    // solve() expect the matrix to be square, but since we have less unknowns than equations
    // we need to ensure that the matrix is not singular by adding 3 more "fake" equations here
    // Below, we then only accept solutions where the last 3 unknowns are basically 0
    for j in 0..3 {
        matrix[(j * 3 + j, 2 * hailstones.len() + j)] = 1.0;
    }

    matrix.solve(&rhs).ok().and_then(|solution| {
        (solution.slice(s![hailstones.len() + 3..]).iter().all(|x| x.abs() < 0.1))
            .then_some(solution.slice(s![hailstones.len()..hailstones.len() + 3]).to_owned())
    })
}

fn part2(input: &Input) -> usize {
    let max = 1000_i64;
    let hailstones = Path::parse_lines(input).collect_vec();

    (-max..max)
        .sorted_unstable_by_key(|vx| vx.abs())
        .collect_vec()
        .into_par_iter()
        .progress()
        .find_map_any(|vx| {
            iproduct!(-max..max, -max..max)
                .sorted_unstable_by_key(|(vy, vz)| vy.abs() + vz.abs())
                .find_map(|(vy, vz)| {
                    // solving for hitting 3 hailstones yields enough equations 
                    // to yield a solution (3 colision times + 3 positions)
                    // The other rocks must then also be hit (as the input promises there's a solution)
                    check_velocity(&arr1(&[vx as f64, vy as f64, vz as f64]), hailstones[..3].try_into().unwrap())
                })
                .map(|res| (res[0] + res[1] + res[2]) as usize)
        })
        .unwrap()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (part1(&input, 200000000000000.0, 400000000000000.0), part2(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let example = &Input::from(
            r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
                "#,
        );
        assert_eq!(part1(example, 7.0, 27.0), 2);
    }
}
