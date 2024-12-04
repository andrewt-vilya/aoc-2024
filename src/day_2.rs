use std::cmp::Ordering;

fn parse_input() -> Vec<Vec<u8>> {
    include_str!("../inputs/input-2.txt").lines()
        .map(|line| line.split(' ')
            .map(|s| s.parse().expect("Invalid number"))
            .collect()
        )
        .collect()
}

fn is_strictly_monotonic(values: &[u8]) -> bool {
    if values.len() == 1 { return true; }

    let mut diffs = values.array_windows().map(|[a, b]| a.cmp(b));

    match diffs.next().unwrap() {
        Ordering::Equal => false,
        Ordering::Less    => diffs.all(|d| d == Ordering::Less),
        Ordering::Greater => diffs.all(|d| d == Ordering::Greater),
    }
}

fn is_gradual(values: &[u8]) -> bool {
    values.array_windows()
        .all(|&[a, b]| a.abs_diff(b) < 4)
}

fn is_safe(values: &[u8]) -> bool {
    is_strictly_monotonic(values) && is_gradual(values)
}

fn is_safe_padded(values: &[u8]) -> bool {
    let mut vec = Vec::with_capacity(values.len());

    // Try all possible removals
    for i in 0..values.len() {
        vec.clear();
        vec.extend_from_slice(values);
        vec.remove(i);
        if is_safe(&vec) { return true; }
    }

    false
}

pub fn main() {
    let input = parse_input();

    let (safe, dangerous) = input.into_iter()
        .partition::<Vec<_>, _>(|report| is_safe(report));

    println!("Part 1 | Safe report count: {}", safe.len());


    let padded_count = dangerous.into_iter()
        .filter(|report| is_safe_padded(report))
        .count();

    println!("Part 2 | Safe report count: {}", safe.len() + padded_count);
}
