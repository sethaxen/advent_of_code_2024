use std::collections::HashSet;

use aoc::load_input;

type OrderingRules = HashSet<(u32, u32)>;

fn cmp_orderings(a: u32, b: u32, orders: &OrderingRules) -> bool {
    a == b || orders.contains(&(a, b))
}

fn is_update_valid(updates: &Vec<u32>, orders: &OrderingRules) -> bool {
    updates.is_sorted_by(|a, b| cmp_orderings(*a, *b, orders))
}

fn solve_part1(input: &str) -> u32 {
    let mut orders: OrderingRules = HashSet::new();
    let mut lines = input.trim_end().lines();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        } else {
            let (before_str, after_str) = line.split_once('|').unwrap();
            let before = before_str.parse::<u32>().unwrap();
            let after = after_str.parse::<u32>().unwrap();
            orders.insert((before, after));
        }
    }
    let mut middle_sum = 0;
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        } else {
            let updates: Vec<u32> = line.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            let num_pages = updates.len();
            if is_update_valid(&updates, &orders) {
                middle_sum += updates[num_pages / 2];
            }
        }
    }
    middle_sum
}

fn main() {
    let input = load_input("day05.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
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
}
