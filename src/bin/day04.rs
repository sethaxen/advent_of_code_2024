use aoc::load_input;
use std::cmp::min;

type Grid<T> = Vec<Vec<T>>;
type Point<T> = (T, T);

// parse the input into a grid of bytes
fn parse_grid(input: &str) -> Grid<u8> {
    input
        .trim_end()
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect()
}

// convert a word to a u32 by treating it as a sequence of bytes
fn bytes_as_u32(word: &str) -> u32 {
    word.chars()
        .map(|c| c as u8)
        .fold(0, |acc, b| (acc << 8) | b as u32)
}

fn bytes_as_u16(word: &str) -> u16 {
    word.chars()
        .map(|c| c as u8)
        .fold(0, |acc, b| (acc << 8) | b as u16)
}

fn count_matches_line(
    grid: &Grid<u8>,
    start: Point<usize>,
    direction: Point<isize>,
    size: usize,
    patterns: &[u32], // words represented as bytes
) -> u32 {
    let mut match_count = 0;
    let mut bytes = 0; // sliding window of bytes
    let (mut row_idx, mut col_idx) = start;
    for _ in 0..size {
        bytes = (bytes << 8) | grid[row_idx][col_idx] as u32; // shift bytes and add new byte
        if patterns.contains(&bytes) {
            match_count += 1;
        }
        row_idx = row_idx.checked_add_signed(direction.0).unwrap_or(0);
        col_idx = col_idx.checked_add_signed(direction.1).unwrap_or(0);
    }
    match_count
}

fn solve_part1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let patterns = [bytes_as_u32("XMAS"), bytes_as_u32("SAMX")];
    let mut match_count = 0;
    let height = grid.len();
    let width = grid[0].len();

    // Count matches in rows and columns
    for idx in 0..height {
        match_count += count_matches_line(&grid, (idx, 0), (0, 1), width, &patterns);
    }
    for idx in 0..width {
        match_count += count_matches_line(&grid, (0, idx), (1, 0), height, &patterns);
    }

    // Count matches in diagonals and anti-diagonals
    for row_idx in 0..height - 3 {
        match_count += count_matches_line(
            &grid,
            (row_idx, 0),
            (1, 1),
            min(width, height - row_idx),
            &patterns,
        );
    }
    for col_idx in 1..width - 3 {
        match_count += count_matches_line(
            &grid,
            (0, col_idx),
            (1, 1),
            min(height, width - col_idx),
            &patterns,
        );
    }
    for col_idx in 3..width {
        match_count += count_matches_line(
            &grid,
            (0, col_idx),
            (1, -1),
            min(height, col_idx + 1),
            &patterns,
        );
    }
    for row_idx in 1..height - 3 {
        match_count += count_matches_line(
            &grid,
            (row_idx, width - 1),
            (1, -1),
            min(height, width - row_idx),
            &patterns,
        );
    }
    match_count
}

fn solve_part2(input: &str) -> u32 {
    let grid = parse_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let patterns = [bytes_as_u16("MS"), bytes_as_u16("SM")];
    let mut match_count: u32 = 0;
    for row_idx in 1..rows - 1 {
        for col_idx in 1..cols - 1 {
            if grid[row_idx][col_idx] == b'A' {
                let diag_tail = (grid[row_idx - 1][col_idx - 1] as u16) << 8
                    | grid[row_idx + 1][col_idx + 1] as u16;
                let anti_diag_tail = (grid[row_idx - 1][col_idx + 1] as u16) << 8
                    | grid[row_idx + 1][col_idx - 1] as u16;
                if patterns.contains(&diag_tail) && patterns.contains(&anti_diag_tail) {
                    match_count += 1;
                }
            } else {
                continue;
            }
        }
    }
    match_count
}

fn main() {
    let input = load_input("day04.txt");
    println!("Solution to part 1: {}", solve_part1(&input));
    println!("Solution to part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM\n\
    MSAMXMSMSA\n\
    AMXSXMAAMM\n\
    MSAMASMSMX\n\
    XMASAMXAMM\n\
    XXAMMXXAMA\n\
    SMSMSASXSS\n\
    SAXAMASAAA\n\
    MAMMMXMMMM\n\
    MXMXAXMASX\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 9);
    }
}
