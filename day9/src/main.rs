use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn extrapolate_front(hist: &[i64]) -> i64 {
    let mut hists = vec![hist.to_vec()];

    loop {
        let diffs =
            hists.last().unwrap().iter().tuple_windows().map(|(&a, &b)| b - a).collect_vec();
        if diffs.iter().all(|&h| h == 0) {
            break;
        }
        hists.push(diffs);
    }

    for i in (0..hists.len() - 1).rev() {
        let new_val = hists[i].first().unwrap() - hists[i + 1].first().unwrap();
        hists[i].insert(0, new_val);
    }

    hists[0][0]
}

fn extrapolate(hist: &[i64]) -> i64 {
    if hist.iter().all(|&h| h == 0) {
        return 0;
    }

    let diffs = hist.iter().tuple_windows().map(|(&a, &b)| b - a).collect_vec();

    hist.last().unwrap() + extrapolate(&diffs)
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let hists = input.lines().map(|l| extract_numbers(l).collect_vec()).collect_vec();

    (
        hists.iter().map(|h| extrapolate(h)).sum::<i64>(),
        hists.iter().map(|h| extrapolate_front(h)).sum::<i64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#,
            114,
            2
        );
    }
}
