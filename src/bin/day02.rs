use aoc::load_input;
use itertools::Itertools;

fn parse_report(report: &str) -> impl Iterator<Item = u32> + '_ {
    return report.split_whitespace().map(|n| n.parse::<u32>().unwrap());
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

fn solve_part1(input: &str) -> usize {
    let num_safe = input
        .trim_end()
        .lines()
        .map(|r| are_levels_safe(parse_report(r)))
        .filter(|b| *b)
        .count();
    return num_safe;
}

fn main() {
    let input = load_input("day02.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
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
}
