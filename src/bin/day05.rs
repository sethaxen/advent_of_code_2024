use std::cmp::Ordering;
use std::collections::HashSet;

use aoc::load_input;

type OrderingRules = HashSet<(u32, u32)>;

fn cmp_orderings(a: u32, b: u32, orders: &OrderingRules) -> bool {
    a == b || orders.contains(&(a, b))
}

fn cmp_orderings2(a: u32, b: u32, orders: &OrderingRules) -> Ordering {
    if a == b {
        Ordering::Equal
    } else if orders.contains(&(a, b)) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn is_update_valid(updates: &Vec<u32>, orders: &OrderingRules) -> bool {
    updates.is_sorted_by(|a, b| cmp_orderings(*a, *b, orders))
}

fn make_update_valid(updates: &Vec<u32>, orders: &OrderingRules) -> Vec<u32> {
    let mut valid_updates = updates.clone();
    valid_updates.sort_by(|a, b| cmp_orderings2(*a, *b, orders));
    valid_updates
}

fn parse_ordering_rules(ordering_rules: &str) -> OrderingRules {
    let mut orders = HashSet::new();
    for line in ordering_rules.lines() {
        let (before, after) = line.split_once('|').unwrap();
        orders.insert((
            before.parse::<u32>().unwrap(),
            after.parse::<u32>().unwrap(),
        ));
    }
    orders
}

fn parse_updates<'a>(updates_str: &'a str) -> impl Iterator<Item = Vec<u32>> + 'a {
    updates_str
        .trim()
        .lines()
        .map(|line| line.split(',').map(|s| s.parse::<u32>().unwrap()).collect())
}

fn count_valid_middle_pages(
    updates_iter: impl Iterator<Item = Vec<u32>>,
    orders: &OrderingRules,
) -> u32 {
    let mut middle_sum = 0;
    for updates in updates_iter {
        let num_pages = updates.len();
        if is_update_valid(&updates, orders) {
            middle_sum += updates[num_pages / 2];
        }
    }
    middle_sum
}

fn count_invalid_middle_pages(
    updates_iter: impl Iterator<Item = Vec<u32>>,
    orders: &OrderingRules,
) -> u32 {
    let mut middle_sum = 0;
    for updates in updates_iter {
        let num_pages = updates.len();
        if !is_update_valid(&updates, orders) {
            let valid_updates: Vec<u32> = make_update_valid(&updates, orders);
            middle_sum += valid_updates[num_pages / 2];
        }
    }
    middle_sum
}

fn solve_part1(input: &str) -> u32 {
    let (ordering_rules_str, updates_str) = input.trim_end().split_once("\n\n").unwrap();
    let orders = parse_ordering_rules(ordering_rules_str);
    let updates_iter = parse_updates(updates_str);
    count_valid_middle_pages(updates_iter, &orders)
}

fn solve_part2(input: &str) -> u32 {
    let (ordering_rules_str, updates_str) = input.trim_end().split_once("\n\n").unwrap();
    let orders = parse_ordering_rules(ordering_rules_str);
    let updates_iter = parse_updates(updates_str);
    count_invalid_middle_pages(updates_iter, &orders)
}

fn main() {
    let input = load_input("day05.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
    println!("Solution to part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53\n\
    97|13\n\
    97|61\n\
    97|47\n\
    75|29\n\
    61|13\n\
    75|53\n\
    29|13\n\
    97|29\n\
    53|29\n\
    61|53\n\
    97|53\n\
    61|29\n\
    47|13\n\
    75|47\n\
    97|75\n\
    47|61\n\
    75|61\n\
    47|29\n\
    75|13\n\
    53|13\n\n\
    75,47,61,53,29\n\
    97,61,53,29,13\n\
    75,29,13\n\
    75,97,47,61,53\n\
    61,13,29\n\
    97,13,75,29,47\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 123);
    }
}
