use aoc_derive::aoc_main;
use utils::graphs::{dijkstra, WeightedGraph};
use utils::grid::Grid;
use utils::math::Vec2D;
use utils::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, derive_more::Constructor)]
struct Node {
    pos: Vec2D,
    heading: Vec2D,
    straights: u8,
}

struct Map {
    grid: Grid<u8>,
    min_straight: u8,
    max_straight: u8,
}

impl WeightedGraph for Map {
    type Node = Node;

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Node, graphs::Cost)> + 'a {
        [
            (node.straights < self.max_straight).then_some(Node::new(
                node.pos + node.heading,
                node.heading,
                node.straights + 1,
            )),
            (node.straights >= self.min_straight).then_some(Node::new(
                node.pos + node.heading.rotated_left(),
                node.heading.rotated_left(),
                0,
            )),
            (node.straights >= self.min_straight).then_some(Node::new(
                node.pos + node.heading.rotated_right(),
                node.heading.rotated_right(),
                0,
            )),
        ]
        .into_iter()
        .flatten()
        .filter_map(|node| self.grid.get(node.pos).map(|val| (node, *val as usize)))
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let map1 = Map { grid: input.parse_grid_from_characters(), min_straight: 0, max_straight: 2 };
    let map2 = Map { grid: input.parse_grid_from_characters(), min_straight: 3, max_straight: 9 };

    let start1 = Node::new(Vec2D::new(0, 0), Vec2D::new(1, 0), 0);
    let start2 = Node::new(Vec2D::new(0, 0), Vec2D::new(0, 1), 0);
    let target = (map1.grid.num_cols() - 1, map1.grid.num_rows() - 1);

    (
        dijkstra(&map1, [start1, start2], |node| node.pos == target).unwrap(),
        dijkstra(&map2, [start1, start2], |node| node.pos == target && node.straights >= 3)
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
                "#,
            102,
            94
        );
        assert_part2!(
            r#"111111111111
999999999991
999999999991
999999999991
999999999991"#,
            71
        );
    }
}
