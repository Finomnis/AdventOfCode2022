use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    character::complete::{char, space1},
    combinator::map,
    sequence::separated_pair,
};

use crate::utils::nom::extract_nom_value;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Loose,
    Draw,
}

#[aoc_generator(day2, part1)]
pub fn input_generator(input: &str) -> Vec<(Hand, Hand)> {
    let input = input.trim_end();
    input
        .lines()
        .map(separated_pair(
            alt((
                map(char('A'), |_| Hand::Rock),
                map(char('B'), |_| Hand::Paper),
                map(char('C'), |_| Hand::Scissors),
            )),
            space1,
            alt((
                map(char('X'), |_| Hand::Rock),
                map(char('Y'), |_| Hand::Paper),
                map(char('Z'), |_| Hand::Scissors),
            )),
        ))
        .map(extract_nom_value(input))
        .collect()
}

#[aoc_generator(day2, part2)]
pub fn input_generator_2(input: &str) -> Vec<(Hand, Outcome)> {
    let input = input.trim_end();
    input
        .lines()
        .map(separated_pair(
            alt((
                map(char('A'), |_| Hand::Rock),
                map(char('B'), |_| Hand::Paper),
                map(char('C'), |_| Hand::Scissors),
            )),
            space1,
            alt((
                map(char('X'), |_| Outcome::Loose),
                map(char('Y'), |_| Outcome::Draw),
                map(char('Z'), |_| Outcome::Win),
            )),
        ))
        .map(extract_nom_value(input))
        .collect()
}

impl Hand {
    pub fn outcome_against(self, other: Hand) -> Outcome {
        match (self, other) {
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (hand1, hand2) if hand1 == hand2 => Outcome::Draw,
            _ => Outcome::Loose,
        }
    }

    pub fn from_outcome(other: Hand, outcome: Outcome) -> Hand {
        match (other, outcome) {
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Rock, Outcome::Loose) => Hand::Scissors,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Paper, Outcome::Loose) => Hand::Rock,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Scissors, Outcome::Loose) => Hand::Paper,
            (hand, Outcome::Draw) => hand,
        }
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Hand, Hand)]) -> u32 {
    input
        .into_iter()
        .map(|&(opponents, ours)| {
            let base_points = match ours {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };

            let win_points = match ours.outcome_against(opponents) {
                Outcome::Loose => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
            };

            base_points + win_points
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Hand, Outcome)]) -> u32 {
    input
        .into_iter()
        .map(|&(opponents, outcome)| {
            let ours = Hand::from_outcome(opponents, outcome);

            let base_points = match ours {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };

            let win_points = match ours.outcome_against(opponents) {
                Outcome::Loose => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
            };

            base_points + win_points
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = crate::utils::day_input!(day2);

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 15);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 14163);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&input_generator_2(EXAMPLE)), 12);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator_2(INPUT)), 12091);
    }
}
