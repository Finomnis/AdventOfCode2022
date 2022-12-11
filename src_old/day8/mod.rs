use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
};

use crate::utils::nom::extract_nom_value;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    let input = input.trim();

    let single_digit = map(one_of("0123456789"), |ch| ch as u8 - '0' as u8);

    extract_nom_value(input)(separated_list1(line_ending, many1(single_digit))(input))
}

#[aoc(day8, part1)]
pub fn solve_part1(trees: &[Vec<u8>]) -> u64 {
    println!("{:?}", trees);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = crate::utils::day_input!(day8);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 95437);
    }

    // #[test]
    // fn part1() {
    //     assert_eq!(solve_part1(&input_generator(INPUT)), 1915606);
    // }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(solve_part2(&input_generator(EXAMPLE)), 24933642);
    // }

    // #[test]
    // fn part2() {
    //     assert_eq!(solve_part2(&input_generator(INPUT)), 5025657);
    // }
}
