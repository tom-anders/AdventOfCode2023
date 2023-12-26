use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn write_graph(input: &Input, output: &str) -> Result<(), std::io::Error> {
    let mut file = BufWriter::new(File::create(output)?);
    use dot_writer::*;
    let mut writer = DotWriter::from(&mut file);

    let mut graph = writer.digraph();
    graph.graph_attributes().set("nodesep", "1.0", false).set("ranksep", "10.0", false);
    for line in input.lines() {
        let (from, to) = line.split(':').collect_tuple().unwrap();
        let to = to.split_ascii_whitespace();
        for t in to {
            graph.edge(from, t);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct WireGraph<'a>(HashMap<&'a str, HashSet<&'a str>>);

impl<'a> From<&'a str> for WireGraph<'a> {
    fn from(value: &'a str) -> Self {
        Self(
            value
                .lines()
                .map(|line| {
                    let (from, to) = line.split(':').collect_tuple().unwrap();
                    let to: HashSet<_> = to.split_ascii_whitespace().collect();
                    (from, to)
                })
                .collect(),
        )
    }
}

impl WireGraph<'_> {
    fn remove(&mut self, to: &str, from: &str) {
        self.0.get_mut(to).unwrap().remove(from);
    }

    fn group_size(&self, start: &str) -> usize {
        let mut visited = HashSet::new();
        let mut next = vec![start];
        while !next.is_empty() {
            visited.extend(next.clone());
            next = next
                .into_iter()
                .flat_map(|from| {
                    self.0
                        .get(from)
                        .into_iter()
                        .flat_map(|nodes| nodes.clone().into_iter())
                        .chain(self.0.iter().filter_map(|(k, v)| v.contains(from).then_some(*k)))
                        .filter(|&node| !visited.contains(node))
                })
                .collect();
        }
        visited.len()
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    write_graph(&input, "inputs/graph25.dot").unwrap();

    let mut graph = WireGraph::from(input.raw.as_str());
    graph.remove("lkf", "scf");
    graph.remove("pgl", "mtl");
    graph.remove("zxb", "zkv");

    graph.group_size("tnr") * graph.group_size("fgx")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        let example = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
        if let Err(e) = write_graph(&Input::from(example), "../inputs/example25.dot") {
            println!("Warning, failed to write graph: {e}");
        }

        let mut graph = WireGraph::from(example);
        graph.remove("pzl", "hfx");
        graph.remove("cmg", "bvb");
        graph.remove("jqt", "nvd");

        assert_eq!(graph.group_size("rhn") * graph.group_size("frs"), 54);
    }
}
