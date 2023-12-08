use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::{regex, regex_captures};
use rangemap::{RangeMap, RangeSet};
use utils::{Input, Solution, RegexHelper};

fn parse_range_map(s: &[&str]) -> RangeMap<u64, u64> {
    s.iter()
        .skip(1)
        .map(|line| {
            let (dest_range_start, source_range_start, len) = regex!(r"\d+")
                .find_parse_into_tuple(line);

            (
                source_range_start..source_range_start + len,
                dest_range_start,
            )
        })
        .collect()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let lines = input.lines().collect_vec();
    let mut blocks = lines.split(|line| line.is_empty());

    let seeds: Vec<u64> = regex!(r"\d+")
        .find_iter_parse(blocks.next().unwrap().first().unwrap())
        .collect_vec();

    let type_maps = blocks.map(parse_range_map).collect_vec();

    let part1 = seeds
        .iter()
        .map(|seed| {
            type_maps.iter().fold(*seed, |from, type_map| {
                match type_map.get_key_value(&from) {
                    Some((source_range, dest_start)) => dest_start + (from - source_range.start),
                    None => from,
                }
            })
        })
        .min()
        .unwrap();

    let part2 = seeds
        .into_iter()
        .tuples()
        .map(|(seed_start, len)| {
            let initial_range = RangeSet::from_iter(std::iter::once(seed_start..seed_start + len));
            type_maps
                .iter()
                .fold(initial_range, |current_ranges, type_map| {
                    current_ranges
                        .iter()
                        // For every of our ranges, find the ranges in the type_map that overlap
                        // with it.
                        .flat_map(|current_range| {
                            type_map
                                .overlapping(current_range)
                                .zip(std::iter::repeat(current_range))
                        })
                        // Calculate the intersection between our range and the overlapping range,
                        // then map it to the resulting destination range
                        .map(|((range_with_overlap, dest_start), current_range)| {
                            let intersection = current_range.start.max(range_with_overlap.start)
                                ..current_range.end.min(range_with_overlap.end);

                            let new_start =
                                dest_start + (intersection.start - range_with_overlap.start);

                            new_start..new_start + intersection.try_len().unwrap() as u64
                        })
                        // RangeMap::gaps() gives us the ranges where there's no overlap.
                        // Those are not modified, so simply add them our next set
                        .chain(current_ranges.iter().flat_map(|r| type_map.gaps(r)))
                        .collect()
                })
                // After the fold, we have RangeSet containing our initial seed range mapped to
                // location ranges.
                // Since RangeSet is ordered, the minimum is simply the start of the first range
                .iter()
                .next()
                .unwrap()
                .start
        })
        .min()
        .unwrap();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use utils::assert_example;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
                "#,
            35,
            46
        );
    }
}
