use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::{regex, regex_captures};
use utils::*;

#[derive(Debug, Clone, Copy)]
enum GoTo<'a> {
    NextWorkflow(&'a str),
    Accept,
    Reject,
}

impl<'a> From<&'a str> for GoTo<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "A" => GoTo::Accept,
            "R" => GoTo::Reject,
            next => GoTo::NextWorkflow(next),
        }
    }
}

#[derive(Debug, Clone, Copy, parse_display::FromStr)]
enum Op {
    #[display("<")]
    Less,
    #[display(">")]
    Greater,
}

#[derive(Debug, Clone, Copy)]
enum Rule<'a> {
    GoTo(GoTo<'a>),
    If { lhs: &'a str, op: Op, rhs: usize, then: GoTo<'a> },
}

impl Rule<'_> {
    fn test(&self, part: &Part) -> Option<&GoTo> {
        match self {
            Rule::GoTo(go_to) => Some(go_to),
            Rule::If { lhs, op, rhs, then: then_go_to } => match op {
                Op::Less if part.ratings[lhs] < *rhs => Some(then_go_to),
                Op::Greater if part.ratings[lhs] > *rhs => Some(then_go_to),
                _ => None,
            },
        }
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        if let Some((_, lhs, op, rhs, then)) =
            regex_captures!(r"([a-z])(<|>)(-?\d+):([A-Z,a-z]+)", s)
        {
            Rule::If {
                lhs,
                rhs: rhs.parse().unwrap(),
                op: op.parse().unwrap_or_else(|_| panic!("Invalid operator: {op}")),
                then: then.into(),
            }
        } else {
            Rule::GoTo(s.into())
        }
    }
}

fn parse_workflow(s: &str) -> (&str, Vec<Rule>) {
    let (name, rules) = regex!(r"([a-z]+)\{(.*)\}").capture_into_tuple(s);
    (name, rules.split(',').map(Rule::from).collect())
}

#[derive(Debug)]
struct Part {
    ratings: HashMap<&'static str, usize>,
}

impl Part {
    fn new(s: &str) -> Self {
        let (x, m, a, s) = extract_numbers(s).collect_tuple().unwrap();
        Self { ratings: [("x", x), ("m", m), ("a", a), ("s", s)].into_iter().collect() }
    }

    fn rating_sum(&self) -> usize {
        self.ratings.values().sum()
    }
}

fn is_part_accepted(part: &Part, workflows: &HashMap<&str, Vec<Rule>>) -> bool {
    let mut current_workflow = &workflows["in"];
    loop {
        for rule in current_workflow {
            match rule.test(part) {
                Some(GoTo::Accept) => return true,
                Some(GoTo::Reject) => return false,
                Some(GoTo::NextWorkflow(next_workflow)) => {
                    current_workflow = &workflows[next_workflow];
                    break;
                }
                None => (),
            }
        }
    }
}

#[derive(Debug, Clone, derive_more::From)]
struct PartRange {
    ratings: HashMap<&'static str, std::ops::Range<usize>>,
}

impl PartRange {
    fn new() -> Self {
        Self {
            ratings: [("x", (1..4001)), ("m", (1..4001)), ("a", (1..4001)), ("s", (1..4001))]
                .into_iter()
                .collect(),
        }
    }

    fn combinations(&self) -> usize {
        self.ratings.values().map(|range| range.len()).product()
    }
}

fn check_part_range<'a, 'b: 'a>(
    part_range: PartRange,
    mut current_workflow: impl Iterator<Item = &'a Rule<'b>>,
    workflows: &HashMap<&str, Vec<Rule<'b>>>,
) -> Vec<PartRange> {
    let handle_goto = |go_to, part_range| match go_to {
        GoTo::Accept => vec![part_range],
        GoTo::Reject => vec![],
        GoTo::NextWorkflow(next_workflow) => {
            check_part_range(part_range, workflows[next_workflow].iter(), workflows)
        }
    };

    match *current_workflow.next().expect("Should never reach end of workflow!") {
        Rule::GoTo(go_to) => handle_goto(go_to, part_range),
        Rule::If { lhs, op, rhs, then } => {
            let ratings = &part_range.ratings[lhs];

            let (then_range, else_range) = match op {
                Op::Less => ((ratings.start..rhs), (rhs..ratings.end)),
                Op::Greater => ((rhs + 1..ratings.end), (ratings.start..rhs + 1)),
            };

            let mut then_part_range = part_range.clone();
            *then_part_range.ratings.get_mut(lhs).unwrap() = then_range;

            let mut else_part_range = part_range;
            *else_part_range.ratings.get_mut(lhs).unwrap() = else_range;

            handle_goto(then, then_part_range)
                .into_iter()
                .chain(check_part_range(else_part_range, current_workflow, workflows))
                .collect()
        }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (workflows, parts) = input.blocks().collect_tuple().unwrap();

    let workflows = workflows.lines().map(parse_workflow).collect();

    let part1 = parts
        .lines()
        .map(Part::new)
        .filter_map(|part| is_part_accepted(&part, &workflows).then_some(part.rating_sum()))
        .sum_usize();

    let accepted_part_ranges =
        check_part_range(PartRange::new(), workflows["in"].iter(), &workflows);

    let part2 = accepted_part_ranges.iter().map(|r| r.combinations()).sum_usize();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
                "#,
            19114,
            167409079868000_usize
        );
    }
}
