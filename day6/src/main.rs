use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::regex;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (times, records) = input
        .lines()
        .map(|line| regex!(r"\d+").find_iter_parse::<u64>(line).collect_vec())
        .collect_tuple()
        .unwrap();

    let part1 = std::iter::zip(times.iter(), records.iter())
        .map(|(&time, &record)| {
            (0..time)
                .filter(|button_time| button_time * (time - button_time) > record)
                .count()
        })
        .product::<usize>();

    let record_part2 = records.into_iter().fold_digits_to_u64();
    let time_part2 = times.into_iter().fold_digits_to_u64();
    let part2 = (0..time_part2)
        .filter(|button_time| button_time * (time_part2 - button_time) > record_part2)
        .count();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::assert_example;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"Time:      7  15   30
Distance:  9  40  200"#,
            288,
            71503
        );
    }
}
