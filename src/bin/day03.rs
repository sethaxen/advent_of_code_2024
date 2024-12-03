use aoc::load_input;
use regex::Regex;

fn solve_part1(input: &str) -> usize {
    let pattern = Regex::new(r"mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();
    return pattern
        .captures_iter(input)
        .map(|caps| {
            let l = caps.name("l").unwrap().as_str().parse::<usize>().unwrap();
            let r = caps.name("r").unwrap().as_str().parse::<usize>().unwrap();
            return l * r;
        })
        .sum();
}

fn main() {
    let input = load_input("day03.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 161);
    }
}
