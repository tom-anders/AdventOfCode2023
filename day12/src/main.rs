use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use rayon::prelude::*;
use utils::*;

fn count_arrangements(springs: &str, numbers: Vec<usize>) -> usize {
    count_arrangements_impl(&springs.chars().collect_vec(), &numbers, 0, &mut HashMap::new())
}

#[derive(PartialEq, Eq, Hash)]
struct CacheKey(Vec<char>, Vec<usize>, usize);

fn count_arrangements_impl(
    springs: &[char],
    numbers: &[usize],
    broken_count: usize,
    cache: &mut HashMap<CacheKey, usize>,
) -> usize {
    if let Some(result) = cache.get(&CacheKey(springs.to_vec(), numbers.to_vec(), broken_count)) {
        return *result;
    }

    let result = match (springs.first(), springs.get(1)) {
        (Some('.'), Some(_)) => match (numbers, broken_count) {
            (_, 0) => count_arrangements_impl(&springs[1..], numbers, 0, cache),
            ([], _) => {
                if broken_count == 0 {
                    1
                } else {
                    0
                }
            }

            ([n, ..], _) if *n == broken_count => {
                count_arrangements_impl(&springs[1..], &numbers[1..], 0, cache)
            }
            _ => 0,
        },
        (Some('.'), None) => match (numbers, broken_count) {
            ([n], _) if broken_count == *n => 1,
            ([], 0) => 1,
            _ => 0,
        },
        (Some('#'), None) => match numbers {
            [n] if broken_count + 1 == *n => 1,
            _ => 0,
        },
        (Some('#'), Some(_)) => count_arrangements_impl(
            &springs[1..],
            numbers,
            broken_count + 1,
            cache,
        ),
        (Some('?'), _) => {
            count_arrangements_impl(
                &std::iter::once('#').chain(springs[1..].iter().copied()).collect_vec(),
                numbers,
                broken_count,
                cache,
            ) + count_arrangements_impl(
                &std::iter::once('.').chain(springs[1..].iter().copied()).collect_vec(),
                numbers,
                broken_count,
                cache,
            )
        }
        _ => unreachable!(),
    };

    cache.insert(CacheKey(springs.to_vec(), numbers.to_vec(), broken_count), result);

    result
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let part1 = input
        .lines()
        .map(|line| {
            let (springs, numbers) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let numbers = extract_numbers::<usize>(numbers).collect_vec();
            count_arrangements(springs, numbers)
        })
        .sum::<usize>();

    let part2 = input
        .lines()
        .collect_vec()
        .into_par_iter()
        .map(|line| {
            let (springs, numbers) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let numbers = std::iter::repeat(extract_numbers::<usize>(numbers).collect_vec())
                .take(5)
                .flatten()
                .collect_vec();
            #[allow(unstable_name_collisions)] // std implementation will have same behavior as itertools
            let springs_repeated: String = std::iter::repeat(springs.chars())
                .take(5)
                .intersperse("?".chars())
                .flatten()
                .collect();
            count_arrangements(&springs_repeated, numbers)
        })
        .sum::<usize>();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arrangements() {
        assert_eq!(count_arrangements("?#?", vec![2]), 2);
        assert_eq!(count_arrangements("??", vec![1]), 2);
        assert_eq!(count_arrangements("??.", vec![1]), 2);
        assert_eq!(count_arrangements("??", vec![2]), 1);
        assert_eq!(count_arrangements("??", vec![2]), 1);

        assert_eq!(count_arrangements("???.###", vec![1, 1, 3]), 1);
        assert_eq!(count_arrangements(".??..??...?##.", vec![1, 1, 3]), 4);
        assert_eq!(count_arrangements("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6]), 1);
        assert_eq!(count_arrangements("????.#...#...", vec![4, 1, 1]), 1);

        assert_eq!(count_arrangements("????.######..#####.", vec![1, 6, 5]), 4);
        assert_eq!(count_arrangements("?###????????", vec![3, 2, 1]), 10);
        assert_eq!(count_arrangements("???", vec![1, 1]), 1);
    }

    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
                "#,
            21,
            525152
        );
    }
}
