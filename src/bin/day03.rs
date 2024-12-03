use aoc::load_input;
use regex::Regex;

fn compute_muls(input: &str) -> usize {
    let pattern = Regex::new(r"mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();
    pattern
        .captures_iter(input)
        .map(|caps| {
            let l = caps.name("l").unwrap().as_str().parse::<usize>().unwrap();
            let r = caps.name("r").unwrap().as_str().parse::<usize>().unwrap();
            l * r
        })
        .sum()
}

fn solve_part1(input: &str) -> usize {
    compute_muls(input)
}

fn solve_part2(input: &str) -> usize {
    input
        .split("don't()")
        .enumerate()
        .map(|(i, b)| {
            let enabled = if i == 0 {
                b
            } else {
                b.split_once("do()").unwrap_or(("", "")).1
            };
            compute_muls(enabled)
        })
        .sum()
}

fn main() {
    let input = load_input("day03.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
    println!("Solution to part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT2), 48);
    }
}
