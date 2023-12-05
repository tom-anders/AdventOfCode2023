use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::regex_captures;
use rangemap::RangeMap;
use rayon::prelude::*;
use utils::*;

fn parse_map(s: &[&str]) -> RangeMap<u64, u64> {
    s.iter()
        .skip(1)
        .map(|line| {
            let (_, dest_range_start, source_range_start, len) =
                regex_captures!(r"(\d+) (\d+) (\d+)", line).unwrap();

            let (dest_range_start, source_range_start, len) = (
                dest_range_start.parse::<u64>().unwrap(),
                source_range_start.parse::<u64>().unwrap(),
                len.parse::<u64>().unwrap(),
            );
            (
                source_range_start..source_range_start + len,
                dest_range_start,
            )
        })
        .collect()
}

fn map_from(map: &RangeMap<u64, u64>, from: u64) -> u64 {
    map.get_key_value(&from)
        .map(|(source_range, dest_start)| dest_start + (from - source_range.start))
        .unwrap_or(from)
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let lines = input.lines().collect_vec();
    let mut blocks = lines.split(|line| line.is_empty());

    let seeds: Vec<u64> = extract_numbers(
        blocks
            .next()
            .unwrap()
            .first()
            .unwrap()
            .split(": ")
            .last()
            .unwrap(),
    )
    .collect();

    let seed_to_soil = parse_map(blocks.next().unwrap());
    let soil_to_fert = parse_map(blocks.next().unwrap());
    let fert_to_water = parse_map(blocks.next().unwrap());
    let water_to_light = parse_map(blocks.next().unwrap());
    let light_to_temp = parse_map(blocks.next().unwrap());
    let temp_to_humidity = parse_map(blocks.next().unwrap());
    let humidity_to_location = parse_map(blocks.next().unwrap());

    let seed_to_location = |seed: u64| {
        let soil = map_from(&seed_to_soil, seed);
        let fert = map_from(&soil_to_fert, soil);
        let water = map_from(&fert_to_water, fert);
        let light = map_from(&water_to_light, water);
        let temp = map_from(&light_to_temp, light);
        let hum = map_from(&temp_to_humidity, temp);
        map_from(&humidity_to_location, hum)
    };

    let part1 = seeds.iter().copied().map(seed_to_location).min().unwrap();

    let part2 = seeds
        .into_iter()
        .tuples()
        .collect_vec() // need to collect once so that size is know, allowing parallel iteration
        .into_par_iter()
        .flat_map(|(seed_start, len)| seed_start..seed_start + len)
        .map(seed_to_location)
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
        // Any source numbers that aren't mapped correspond to the same destination number. So,
        // seed number 10 corresponds to soil number 10.
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
