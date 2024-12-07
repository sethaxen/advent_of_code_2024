use std::collections::{HashMap, HashSet};

use aoc::load_input;

type OrderingRules = HashMap<u32, HashSet<u32>>;

fn is_update_valid(updates: &Vec<u32>, orders: &OrderingRules) -> bool {
    let mut page_numbers = HashSet::new();
    for page_num in updates {
        page_numbers.insert(*page_num);
    }
    for page_num in updates.iter() {
        page_numbers.remove(page_num);
        if page_numbers.len() == 0 {
            break;
        }
        let after_pages = orders.get(page_num);
        if after_pages.is_none() {
            return false;
        }
        if !page_numbers.is_subset(&after_pages.unwrap()) {
            return false;
        }
    }
    true
}

fn solve_part1(input: &str) -> u32 {
    let mut orders: OrderingRules = HashMap::new();
    let mut lines = input.trim_end().lines();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        } else {
            let (before_str, after_str) = line
                .split_once('|')
                .unwrap();
            let before = before_str.parse::<u32>().unwrap();
            let after = after_str.parse::<u32>().unwrap();
            orders.entry(before)
                .or_insert(HashSet::new())
                .insert(after);
        }
    }
    let mut middle_sum = 0;
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        } else {
            let updates: Vec<u32> = line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
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
