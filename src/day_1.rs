use std::collections::HashMap;

fn parse_input() -> (Vec<u32>, Vec<u32>) {
    include_str!("../inputs/input-1.txt").lines()
        .map(|line| {
            let mut numbers = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap());

            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip()
}

fn occurrence_scores(items: &[u32]) -> HashMap<u32, u64> {
    debug_assert!(items.is_sorted());

    let mut iter = items.iter().copied();
    let Some(mut prev) = iter.next() else { return HashMap::new(); };
    let mut count: u64 = 1;

    let mut scores = HashMap::with_capacity(items.len());
    for item in iter {
        if item != prev {
            scores.insert(prev, count * prev as u64);
            prev = item;
            count = 1;
            continue;
        }

        count += 1;
    }

    scores.insert(prev, count * prev as u64);

    scores
}

pub fn main() {
    let (mut left, mut right) = parse_input();

    left.sort_unstable();
    right.sort_unstable();

    let total_distance = left.iter().zip(&right)
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>();

    println!("Part 1 | Total distance: {total_distance}");


    let occurrence_scores = occurrence_scores(&right);
    let similarity = left.iter()
        .map(|n| occurrence_scores.get(n).copied().unwrap_or_default())
        .sum::<u64>();

    println!("Part 2 | Total similarity: {similarity}");
}
