use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{char, u32},
    combinator::map,
    sequence::separated_pair,
};

use crate::utils::nom::extract_nom_value;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .trim_end()
        .lines()
        .map(separated_pair(
            map(separated_pair(u32, char('-'), u32), |(s, e)| s..=e),
            char(','),
            map(separated_pair(u32, char('-'), u32), |(s, e)| s..=e),
        ))
        .map(extract_nom_value(input))
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            (b.contains(a.start()) && b.contains(a.end()))
                || (a.contains(b.start()) && a.contains(b.end()))
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            b.contains(a.start())
                || b.contains(a.end())
                || a.contains(b.start())
                || a.contains(b.end())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = crate::utils::day_input!(day4);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 2);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 459);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 4);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 779);
    }
}
