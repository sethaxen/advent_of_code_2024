use aoc::load_input;
use itertools::Itertools;

fn parse_report(report: &str) -> Vec<u32> {
    return report
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
}

fn iter_excluding_index<T>(iter: impl Iterator<Item = T>, i: usize) -> impl Iterator<Item = T> {
    return iter.enumerate().filter_map(
        move |(index, item)| {
            if index == i {
                None
            } else {
                Some(item)
            }
        },
    );
}

fn are_levels_safe(levels: impl Iterator<Item = u32>) -> bool {
    let unique_level_signs: Vec<i32> = levels
        .tuple_windows()
        .map(|(first, last)| {
            let diff = first as i32 - last as i32;
            return if diff.abs() > 3 || diff == 0 {
                0
            } else {
                diff.signum()
            };
        })
        .unique()
        .collect();
    return unique_level_signs.len() == 1 && unique_level_signs[0] != 0;
}

fn are_levels_almost_safe(levels: Vec<u32>) -> bool {
    let num_levels = levels.len();
    for i in 0..num_levels {
        if are_levels_safe(iter_excluding_index(levels.iter().copied(), i)) {
            return true;
        }
    }
    return false;
}

fn solve_part1(input: &str) -> usize {
    let num_safe = input
        .trim_end()
        .lines()
        .map(|r| are_levels_safe(parse_report(r).into_iter()))
        .filter(|b| *b)
        .count();
    return num_safe;
}

fn solve_part2(input: &str) -> usize {
    let num_safe = input
        .trim_end()
        .lines()
        .map(|r| are_levels_almost_safe(parse_report(r)))
        .filter(|b| *b)
        .count();
    return num_safe;
}

fn main() {
    let input = load_input("day02.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
    println!("Solution to part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    ";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 4);
    }
}
