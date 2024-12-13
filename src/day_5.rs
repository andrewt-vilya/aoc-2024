const INPUT: &str = include_str!("../inputs/input-5.txt");

mod structure {
    #[derive(Clone, Copy, Debug)]
    pub struct Precedence(u32, u32);

    impl Precedence {
        pub fn new(before: u32, after: u32) -> Self {
            debug_assert_ne!(before, after);
            Self(before, after)
        }
    }

    pub struct Update(Vec<u32>);

    impl Update {
        pub fn new(items: Vec<u32>) -> Self { Self(items) }

        fn indices(&self, Precedence(before, after): Precedence) -> Option<(usize, usize)> {
            Some((
                self.0.iter().position(|&item| item == before)?,
                self.0.iter().position(|&item| item == after)?,
            ))
        }

        fn respects(&self, prec: Precedence) -> Option<bool> {
            self.indices(prec).map(|(left, right)| left < right)
        }

        pub fn is_in_order(&self, precedences: &[Precedence]) -> bool {
            precedences.iter()
                .all(|&p| self.respects(p).unwrap_or(true))
        }

        pub fn middle_number(&self) -> u32 {
            debug_assert!(self.0.len() % 2 == 1);
            self.0[self.0.len() / 2]
        }

        pub fn topo_sort(&mut self, precs: &[Precedence]) {
            while !self.is_in_order(precs) {
                for &prec in precs {
                    let Some((left, right)) = self.indices(prec) else { continue; };
                    if left > right {
                        self.0[right..left+1].rotate_right(1);
                    }
                }
            }
        }
    }
}

use structure::{Precedence, Update};

fn parse_input(input: &str) -> (Vec<Precedence>, Vec<Update>) {
    let mut lines = input.lines();
    let orderings = (&mut lines)
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once('|').unwrap())
        .map(|(left, right)| Precedence::new(left.parse().unwrap(), right.parse().unwrap()))
        .collect();

    let updates = lines
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .map(Update::new)
        .collect();

    (orderings, updates)
}

pub fn main() {
    let (precedences, updates) = parse_input(INPUT);
    let (ordered, mut unordered) = updates.into_iter()
        .partition::<Vec<_>, _>(|u| u.is_in_order(&precedences));

    let ordered_sum = ordered.iter().map(|u| u.middle_number()).sum::<u32>();
    println!("Part 1 | Sum: {ordered_sum}");

    for update in &mut unordered {
        update.topo_sort(&precedences);
    }

    let unordered_sum = unordered.iter().map(|u| u.middle_number()).sum::<u32>();
    println!("Part 2 | Sum: {unordered_sum}");
}

#[cfg(test)]
mod tests {
    use crate::day_5::structure::Update;

    use super::parse_input;

    const INPUT: &str = indoc::indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn correct_order() {
        let (precs, updates) = parse_input(INPUT);

        let correct_order = updates.iter()
            .filter(|update| update.is_in_order(&precs))
            .count();

        assert_eq!(3, correct_order);
    }

    #[test]
    fn middle_sum() {
        let (precs, updates) = parse_input(INPUT);

        let middle_sum = updates.iter()
            .filter(|update| update.is_in_order(&precs))
            .map(Update::middle_number)
            .sum::<u32>();

        assert_eq!(143, middle_sum);
    }

    #[test]
    fn unordered_middle_sum() {
        let (precs, updates) = parse_input(INPUT);

        let mut unordered = updates.into_iter()
            .filter(|update| !update.is_in_order(&precs))
            .collect::<Vec<_>>();

        for update in &mut unordered {
            update.topo_sort(&precs);
        }

        let middle_sum = unordered.iter()
            .map(Update::middle_number)
            .sum::<u32>();

        assert_eq!(123, middle_sum);
    }
}
