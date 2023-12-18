use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::regex_find;
use utils::math::Vec2D;
use utils::*;

#[derive(Debug, Clone, Copy, parse_display::FromStr)]
enum Dir {
    R,
    L,
    D,
    U,
}

impl Dir {
    fn heading(&self) -> Vec2D {
        match self {
            Dir::R => (1, 0),
            Dir::L => (-1, 0),
            Dir::D => (0, 1),
            Dir::U => (0, -1),
        }
        .into()
    }
}

#[derive(Debug, Clone)]
struct Instruction(Dir, i64);

impl Instruction {
    fn parse_part1(s: &str) -> Self {
        let (dir, dist, _) = s.split_ascii_whitespace().collect_tuple().unwrap();
        Self(dir.parse().unwrap(), dist.parse().unwrap())
    }

    fn parse_part2(s: &str) -> Self {
        let color_str = &regex_find!(r"#[a-z,0-9]+", s).unwrap()[1..];
        Self(
            match &color_str[5..] {
                "0" => Dir::R,
                "1" => Dir::D,
                "2" => Dir::L,
                "3" => Dir::U,
                _ => unreachable!(),
            },
            i64::from_str_radix(&color_str[..5], 16).unwrap(),
        )
    }
}

fn area(instructions: Vec<Instruction>) -> i64 {
    use Dir::*;
    let vertices = instructions.into_iter().circular_tuple_windows().scan(
        Vec2D::zero(),
        |pos, (Instruction(dir, dist), Instruction(next_dir, _))| {
            *pos += dist * dir.heading();
            Some(
                *pos + match (dir, next_dir) {
                    (R, D) | (D, R) => (1, 0),
                    (D, L) | (L, D) => (1, 1),
                    (L, U) | (U, L) => (0, 1),
                    (U, R) | (R, U) => (0, 0),
                    _ => unreachable!("always expect a turn!"),
                },
            )
        },
    );

    // shoelace formula
    vertices.tuple_windows().map(|(p1, p2)| p1.x * p2.y - p1.y * p2.x).sum::<i64>() / 2
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (
        area(input.lines().map(Instruction::parse_part1).collect()),
        area(input.lines().map(Instruction::parse_part2).collect()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
                "#,
            62,
            952408144115_i64
        );
    }
}
