use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|v| v.parse().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|elv| elv.iter().cloned().sum())
        .max()
        .unwrap_or_default()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|elv| elv.iter().cloned().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = crate::utils::day_input!(day1);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 24000);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 74198);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 45000);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 209914);
    }
}
