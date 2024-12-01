use aoc::load_input;
use std::collections::BTreeMap;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    return input
        .trim_end()
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();
}

fn solve_part1(input: &str) -> u32 {
    let (mut left_list, mut right_list) = parse_input(&input);
    left_list.sort();
    right_list.sort();
    let distance = left_list
        .iter()
        .zip(&right_list)
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    return distance;
}

fn solve_part2(input: &str) -> u32 {
    let (left_list, right_list) = parse_input(&input);
    let mut right_count: BTreeMap<&u32, u32> = BTreeMap::new();
    for id in right_list.iter() {
        *right_count.entry(id).or_insert(0) += 1;
    }
    let similarity = left_list
        .iter()
        .map(|id| right_count.get(id).unwrap_or(&0) * id)
        .sum();
    return similarity;
}

fn main() {
    let input = load_input("day01.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
    println!("Solution to part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    ";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 31);
    }
}
