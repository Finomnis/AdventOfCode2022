mod parser {
    pub use crate::helpers::nom::*;

    use super::Hand;

    pub fn game_rule(input: &str) -> VResult<(Hand, char)> {
        separated_pair(
            alt((
                map(char('A'), |_| Hand::Rock),
                map(char('B'), |_| Hand::Paper),
                map(char('C'), |_| Hand::Scissors),
            )),
            space1,
            anychar,
        )(input)
    }
}

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

pub fn parse_input(input_data: &str) -> Vec<(Hand, char)> {
    input_data
        .trim_end()
        .lines()
        .map(parser::game_rule)
        .map(parser::finalize(input_data))
        .collect()
}

pub fn task1(input: &[(Hand, char)]) -> u32 {
    input
        .iter()
        .map(|&(opponents, ours_ch)| {
            let ours = match ours_ch {
                'X' => Hand::Rock,
                'Y' => Hand::Paper,
                _ => Hand::Scissors,
            };

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

pub fn task2(input: &[(Hand, char)]) -> u32 {
    input
        .iter()
        .map(|&(opponents, outcome_ch)| {
            let outcome = match outcome_ch {
                'X' => Outcome::Loose,
                'Y' => Outcome::Draw,
                _ => Outcome::Win,
            };

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

crate::aoc_tests! {
    task1: {
        simple => 15,
        complex => 14163,
    },
    task2: {
        simple => 12,
        complex => 12091,
    }
}
