use std::str::{Chars, FromStr};

use itertools::Itertools;

use crate::grid::Grid;

pub struct Input {
    pub raw: String,
}

impl Input {
    pub fn new(input_file: &str) -> Input {
        Input {
            raw: std::fs::read_to_string(input_file).unwrap(),
        }
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> + '_ {
        self.raw.lines()
    }

    pub fn parse_blocks<T: FromStr>(&self) -> Vec<Vec<T>> {
        self.lines()
            .collect_vec()
            .split(|line| line.is_empty())
            .map(|lines| {
                lines
                    .iter()
                    .map(|line| line.parse().ok().unwrap())
                    .collect()
            })
            .collect()
    }

    pub fn split_and_parse<T: FromStr>(&self, sep: &'static str) -> impl Iterator<Item = T> + '_
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.raw.split(sep).map(|s| s.trim().parse().unwrap())
    }

    pub fn numbers(&self, sep: &'static str) -> impl Iterator<Item = i64> + '_ {
        self.split_and_parse(sep)
    }

    pub fn parse_lines<T: FromStr>(&self) -> impl Iterator<Item = T> + '_
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.raw.lines().map(|line| line.parse::<T>().unwrap())
    }

    pub fn parse_grid<T: FromStr>(&self, sep: &str) -> Grid<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.lines()
            .map(|line| line.split(sep).map(|s| s.parse().unwrap()).collect_vec())
            .into()
    }

    pub fn chars(&self) -> impl Iterator<Item = Chars<'_>> + '_ {
        self.lines().map(|line| line.chars())
    }

    pub fn get_line(&self, pos: usize) -> &str {
        self.raw.lines().nth(pos).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let input = Input {
            raw: "a\nb\n\nc\n".to_string(),
        };
        assert_eq!(input.lines().collect_vec(), vec!["a", "b", "", "c"]);

        assert_eq!(input.get_line(0), "a");
        assert_eq!(input.get_line(1), "b");
        assert_eq!(input.get_line(2), "");
        assert_eq!(input.get_line(3), "c");
    }

    #[test]
    fn parse_blocks() {
        let input = Input {
            raw: "1\n2\n\n3\n".to_string(),
        };
        assert_eq!(vec![vec![1, 2], vec![3]], input.parse_blocks());
    }

    #[test]
    fn numbers() {
        let input = Input {
            raw: "1,2,3".to_string(),
        };
        assert_eq!(vec![1, 2, 3], input.numbers(",").collect_vec());
    }

    #[test]
    fn split_and_parse() {
        let input = Input {
            raw: "1<<2<<+3".to_string(),
        };
        assert_eq!(vec![1, 2, 3], input.split_and_parse("<<").collect_vec());
    }

    #[test]
    fn parse_lines() {
        let input = Input {
            raw: "1\n2\n123\n".to_string(),
        };
        assert_eq!(vec![1, 2, 123], input.parse_lines().collect_vec());
    }

    #[test]
    fn chars() {
        let input = Input {
            raw: "ab\nc\n".to_string(),
        };
        assert_eq!(
            vec![vec!['a', 'b'], vec!['c']],
            input.chars().map(Itertools::collect_vec).collect_vec()
        );
    }

    #[test]
    fn parse_vec2() {
        use crate::vec2d::Vec2D;
        let input = Input {
            raw: "(1, 2)\n[3, 4]\n".to_string(),
        };
        assert_eq!(
            vec![Vec2D::new(1, 2), Vec2D::new(3, 4)],
            input.parse_lines().collect_vec()
        );
    }

    #[test]
    fn parse_grid() {
        let input = Input {
            raw: "a,bb,c\ndd,e,ff\n".to_string(),
        };
        assert_eq!(
            Grid::from(vec![
                vec!["a".to_string(), "bb".to_string(), "c".to_string()],
                vec!["dd".to_string(), "e".to_string(), "ff".to_string()]
            ]),
            input.parse_grid(",")
        );
    }
}