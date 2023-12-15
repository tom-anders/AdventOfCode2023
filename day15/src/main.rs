use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn my_hash(s: &str) -> usize {
    s.chars().fold(0, |hash, c| ((hash + c as usize) * 17) % 256)
}

#[derive(Debug)]
struct MyHashMap<'a> {
    buckets: [Vec<(&'a str, usize)>; 256],
}

impl<'a> MyHashMap<'a> {
    fn new() -> Self {
        Self { buckets: std::array::from_fn(|_| vec![]) }
    }

    fn bucket(&mut self, key: &str) -> &mut Vec<(&'a str, usize)> {
        &mut self.buckets[my_hash(key)]
    }

    fn remove(&mut self, key: &str) {
        self.bucket(key).retain(|(k, _)| *k != key);
    }

    fn insert(&mut self, key: &'a str, val: usize) {
        let bucket = self.bucket(key);

        if let Some(v) = bucket.iter_mut().find_map(|(k, v)| (*k == key).then_some(v)) {
            *v = val;
        } else {
            bucket.push((key, val));
        }
    }

    fn add_focusing_power(&self) -> usize {
        self.buckets
            .iter()
            .enumerate()
            .map(|(bucket_num, bucket)| {
                (bucket_num + 1)
                    * bucket
                        .iter()
                        .enumerate()
                        .map(|(slot_number, (_, val))| (slot_number + 1) * val)
                        .sum::<usize>()
            })
            .sum()
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let part1 = input.raw.trim().split(',').map(my_hash).sum::<usize>();

    let part2 = input
        .raw
        .trim()
        .split(',')
        .fold(MyHashMap::new(), |mut map, instruction| {
            if let Some((key, val)) = instruction.split('=').collect_tuple() {
                map.insert(key, val.parse().unwrap());
            } else {
                let key = &instruction[..instruction.len() - 1];
                map.remove(key);
            }
            map
        })
        .add_focusing_power();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#, 1320, 145);
    }
}
